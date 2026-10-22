-- Strict mock of the Hyprland Lua API, used to validate generated configs
-- without a running compositor. Option, rule and dispatcher names come from
-- the Hyprland 0.56.2 sources (see tests/hyprland-0.56.2/).
--
-- usage: lua5.4 tests/hl_mock.lua <data dir> <config.lua>

local data_dir, config = arg[1], arg[2]
local errors = {}
local function err(msg) errors[#errors + 1] = msg end

local function lines(path)
    local t = {}
    for l in io.lines(path) do if l ~= "" then t[#t + 1] = l end end
    return t
end

-- "Int input:touchpad:tap-to-click" -> options["input.touchpad.tap_to_click"] = "Int"
local options = {}
for _, l in ipairs(lines(data_dir .. "/options.txt")) do
    local ty, name = l:match("^(%S+) (%S+)$")
    options[name:gsub(":", "."):gsub("-", "_")] = ty
end
local effects, props = {}, {}
for _, l in ipairs(lines(data_dir .. "/rule_effects.txt")) do effects[l] = true end
for _, l in ipairs(lines(data_dir .. "/rule_props.txt")) do props[l] = true end

local function is_color(v) return type(v) == "string" and (v:match("^rgba?%(%x+%)$") or v:match("^#%x+$")) end

local checks = {
    Int = function(v) return math.type(v) == "integer" end,
    Float = function(v) return type(v) == "number" end,
    Bool = function(v) return type(v) == "boolean" end,
    String = function(v) return type(v) == "string" end,
    CssGap = function(v) return math.type(v) == "integer" or type(v) == "table" end,
    Color = function(v) return is_color(v) or type(v) == "number" end,
    Gradient = function(v)
        if is_color(v) then return true end
        if type(v) ~= "table" or type(v.colors) ~= "table" or #v.colors == 0 then return false end
        for _, c in ipairs(v.colors) do if not is_color(c) then return false end end
        return v.angle == nil or type(v.angle) == "number"
    end,
}

local function check_config(tbl, prefix)
    for k, v in pairs(tbl) do
        local path = prefix and (prefix .. "." .. k) or k
        local ty = options[path]
        if ty then
            local check = checks[ty]
            if check and not check(v) then err(("hl.config: %s has wrong type (%s expected, got %s)"):format(path, ty, type(v))) end
        elseif type(v) == "table" then
            check_config(v, path)
        else
            err("hl.config: unknown option " .. path)
        end
    end
end

-- Dispatchers are opaque values; binds must receive one.
local DSP = {}
local function dsp(name, validate)
    return function(a, ...)
        if validate then validate(a, ...) end
        return setmetatable({ name = name }, DSP)
    end
end
local directions = { left = true, right = true, up = true, down = true }
local function need(cond, msg) if not cond then err(msg) end end

local function strict(name, t)
    return setmetatable(t, { __index = function(_, k) error(name .. "." .. tostring(k) .. " does not exist", 2) end })
end

local curves = { default = true }
local leaves = {}
for _, l in ipairs({ "global", "windows", "windowsIn", "windowsOut", "windowsMove", "layers", "layersIn", "layersOut",
    "fade", "fadeIn", "fadeOut", "fadeSwitch", "fadeShadow", "fadeDim", "fadeLayers", "fadeLayersIn", "fadeLayersOut",
    "fadePopups", "fadePopupsIn", "fadePopupsOut", "border", "borderangle", "workspaces", "workspacesIn", "workspacesOut",
    "specialWorkspace", "specialWorkspaceIn", "specialWorkspaceOut", "zoomFactor", "monitorAdded" }) do leaves[l] = true end

local started = {}
local counts = { config = 0, bind = 0, monitor = 0, rule = 0, exec = 0 }

hl = strict("hl", {
    config = function(t) counts.config = counts.config + 1; check_config(t) end,
    monitor = function(t)
        counts.monitor = counts.monitor + 1
        need(type(t.output) == "string", "hl.monitor: output must be a string")
        need(type(t.mode) == "string", "hl.monitor: mode must be a string")
        need(type(t.position) == "string", "hl.monitor: position must be a string")
        need(type(t.scale) == "number" or t.scale == "auto", "hl.monitor: scale must be a number or \"auto\"")
    end,
    env = function(k, v) need(type(k) == "string" and type(v) == "string", "hl.env: expects two strings") end,
    on = function(ev, fn)
        need(ev == "hyprland.start" or ev == "hyprland.shutdown", "hl.on: unknown event " .. tostring(ev))
        need(type(fn) == "function", "hl.on: expects a function")
        started[#started + 1] = fn
    end,
    exec_cmd = function(c) counts.exec = counts.exec + 1; need(type(c) == "string" and c ~= "", "hl.exec_cmd: empty command") end,
    curve = function(name, t)
        need(type(name) == "string", "hl.curve: name must be a string")
        need(t.type == "bezier" and #t.points == 2 and #t.points[1] == 2 and #t.points[2] == 2, "hl.curve: bad bezier " .. tostring(name))
        curves[name] = true
    end,
    animation = function(t)
        need(leaves[t.leaf], "hl.animation: unknown leaf " .. tostring(t.leaf))
        need(type(t.enabled) == "boolean", "hl.animation: enabled must be bool")
        need(type(t.speed) == "number" and t.speed > 0, "hl.animation: speed must be > 0")
        need(t.bezier == nil or curves[t.bezier], "hl.animation: undefined curve " .. tostring(t.bezier))
    end,
    gesture = function(t)
        need(math.type(t.fingers) == "integer", "hl.gesture: fingers must be an integer")
        need(t.direction ~= nil and t.action ~= nil, "hl.gesture: direction and action required")
    end,
    bind = function(keys, d, opts)
        counts.bind = counts.bind + 1
        need(type(keys) == "string" and keys ~= "", "hl.bind: keys must be a string")
        need(not keys:find("%$"), "hl.bind: unexpanded variable in " .. keys)
        need(getmetatable(d) == DSP or type(d) == "function", "hl.bind: second argument must be a dispatcher (" .. keys .. ")")
        need(opts == nil or type(opts) == "table", "hl.bind: options must be a table")
    end,
    window_rule = function(t)
        counts.rule = counts.rule + 1
        need(type(t.match) == "table", "hl.window_rule: match table required")
        for k, v in pairs(t.match or {}) do
            need(props[k], "hl.window_rule: unknown match prop " .. k)
            need(type(v) == "string" or type(v) == "boolean", "hl.window_rule: bad match value for " .. k)
        end
        for k, _ in pairs(t) do
            if k ~= "name" and k ~= "match" then need(effects[k], "hl.window_rule: unknown effect " .. k) end
        end
    end,
    dsp = strict("hl.dsp", {
        exec_cmd = dsp("exec_cmd", function(c) need(type(c) == "string" and c ~= "", "hl.dsp.exec_cmd: empty command") end),
        exit = dsp("exit"),
        layout = dsp("layout", function(m) need(type(m) == "string", "hl.dsp.layout: message must be a string") end),
        focus = dsp("focus", function(t)
            need(type(t) == "table", "hl.dsp.focus: table expected")
            need((t.direction and directions[t.direction]) or t.workspace ~= nil, "hl.dsp.focus: bad target")
        end),
        window = strict("hl.dsp.window", {
            close = dsp("close"),
            pseudo = dsp("pseudo"),
            drag = dsp("drag"),
            resize = dsp("resize"),
            float = dsp("float", function(t) need(t == nil or t.action == nil or t.action == "toggle" or t.action == "set" or t.action == "unset", "hl.dsp.window.float: bad action") end),
            fullscreen = dsp("fullscreen", function(t) need(t == nil or t.mode == nil or t.mode == "fullscreen" or t.mode == "maximized", "hl.dsp.window.fullscreen: bad mode") end),
            move = dsp("move", function(t)
                need(type(t) == "table", "hl.dsp.window.move: table expected")
                need((t.direction and directions[t.direction]) or t.workspace ~= nil, "hl.dsp.window.move: bad target")
            end),
        }),
        workspace = strict("hl.dsp.workspace", {
            toggle_special = dsp("toggle_special", function(n)
                need(type(n) == "string" and not n:find("^special:"), "hl.dsp.workspace.toggle_special: plain name expected")
            end),
        }),
    }),
})

local chunk, load_err = loadfile(config)
if not chunk then
    print("SYNTAX ERROR: " .. load_err)
    os.exit(1)
end
local ok, run_err = pcall(chunk)
if not ok then err("runtime error: " .. tostring(run_err)) end
for _, fn in ipairs(started) do
    local ok2, e = pcall(fn)
    if not ok2 then err("autostart error: " .. tostring(e)) end
end

if #errors > 0 then
    for _, e in ipairs(errors) do print("ERROR: " .. e) end
    os.exit(1)
end
print(("OK  %s  (config calls: %d, binds: %d, monitors: %d, rules: %d, autostart: %d)"):format(
    config, counts.config, counts.bind, counts.monitor, counts.rule, counts.exec))
