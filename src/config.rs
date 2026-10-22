use crate::i18n::Lang;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub type Rgba = [u8; 4];

pub fn hypr_color(c: Rgba) -> String {
    format!("rgba({:02x}{:02x}{:02x}{:02x})", c[0], c[1], c[2], c[3])
}

/// Which config language to write. `Auto` is resolved by `hypr::resolve_format`.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub enum Format {
    #[default]
    Auto,
    Lua,
    Hyprlang,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Bind {
    pub mods: String,
    pub key: String,
    pub dispatcher: String,
    pub arg: String,
}

impl Bind {
    pub fn new(mods: &str, key: &str, dispatcher: &str, arg: &str) -> Self {
        Self {
            mods: mods.into(),
            key: key.into(),
            dispatcher: dispatcher.into(),
            arg: arg.into(),
        }
    }
}

/// Dispatchers offered in the UI (hyprlang names; translated for Lua).
pub const DISPATCHERS: &[&str] = &[
    "exec",
    "killactive",
    "exit",
    "togglefloating",
    "fullscreen",
    "pseudo",
    "togglesplit",
    "movefocus",
    "movewindow",
    "workspace",
    "movetoworkspace",
    "togglespecialworkspace",
    "layoutmsg",
];

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Monitor {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct WindowRule {
    pub field: String,
    pub pattern: String,
    pub effect: String,
    pub value: String,
}

impl Default for WindowRule {
    fn default() -> Self {
        Self {
            field: "class".into(),
            pattern: String::new(),
            effect: "float".into(),
            value: String::new(),
        }
    }
}

pub const RULE_FIELDS: &[&str] = &["class", "title", "initial_class", "initial_title"];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EffectKind {
    Bool,
    Int,
    Str,
}

/// Window rule effects offered in the UI, with the value type Hyprland expects.
pub const RULE_EFFECTS: &[(&str, EffectKind)] = &[
    ("float", EffectKind::Bool),
    ("tile", EffectKind::Bool),
    ("center", EffectKind::Bool),
    ("pin", EffectKind::Bool),
    ("fullscreen", EffectKind::Bool),
    ("maximize", EffectKind::Bool),
    ("no_blur", EffectKind::Bool),
    ("no_shadow", EffectKind::Bool),
    ("no_anim", EffectKind::Bool),
    ("no_initial_focus", EffectKind::Bool),
    ("opacity", EffectKind::Str),
    ("size", EffectKind::Str),
    ("move", EffectKind::Str),
    ("workspace", EffectKind::Str),
    ("monitor", EffectKind::Str),
    ("idle_inhibit", EffectKind::Str),
    ("rounding", EffectKind::Int),
    ("border_size", EffectKind::Int),
];

pub fn effect_kind(effect: &str) -> EffectKind {
    RULE_EFFECTS
        .iter()
        .find(|e| e.0 == effect)
        .map(|e| e.1)
        .unwrap_or(EffectKind::Str)
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub format: Format,

    // general
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub border_size: i32,
    pub active_border_a: Rgba,
    pub active_border_b: Rgba,
    pub border_gradient: bool,
    pub border_angle: i32,
    pub inactive_border: Rgba,
    pub layout: String,
    pub resize_on_border: bool,

    // decoration
    pub rounding: i32,
    pub active_opacity: f32,
    pub inactive_opacity: f32,
    pub shadow_enabled: bool,
    pub shadow_range: i32,
    pub shadow_power: i32,
    pub shadow_color: Rgba,
    pub blur_enabled: bool,
    pub blur_size: i32,
    pub blur_passes: i32,
    pub blur_vibrancy: f32,
    pub dim_inactive: bool,
    pub dim_strength: f32,

    // animations
    pub animations_enabled: bool,
    pub animation_speed: f32,
    pub animation_style: String,

    // input
    pub kb_layout: String,
    pub kb_variant: String,
    pub kb_options: String,
    pub follow_mouse: i32,
    pub sensitivity: f32,
    pub natural_scroll: bool,
    pub tap_to_click: bool,
    pub numlock_by_default: bool,
    pub workspace_gesture: bool,

    // misc
    pub disable_hyprland_logo: bool,
    pub disable_splash: bool,

    // programs and binds
    pub main_mod: String,
    pub terminal: String,
    pub file_manager: String,
    pub launcher: String,
    pub workspace_binds: bool,
    pub binds: Vec<Bind>,

    pub monitors: Vec<Monitor>,
    pub autostart: Vec<String>,
    pub window_rules: Vec<WindowRule>,
    pub env: Vec<EnvVar>,

    // hyprpaper
    pub wallpaper_enabled: bool,
    pub wallpaper: String,
    pub wallpaper_fit: String,

    pub extra: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            format: Format::Auto,

            gaps_in: 5,
            gaps_out: 15,
            border_size: 2,
            active_border_a: [0x89, 0xb4, 0xfa, 0xff],
            active_border_b: [0xcb, 0xa6, 0xf7, 0xff],
            border_gradient: true,
            border_angle: 45,
            inactive_border: [0x45, 0x47, 0x5a, 0xaa],
            layout: "dwindle".into(),
            resize_on_border: true,

            rounding: 10,
            active_opacity: 1.0,
            inactive_opacity: 0.95,
            shadow_enabled: true,
            shadow_range: 8,
            shadow_power: 3,
            shadow_color: [0x11, 0x11, 0x1b, 0xee],
            blur_enabled: true,
            blur_size: 6,
            blur_passes: 2,
            blur_vibrancy: 0.17,
            dim_inactive: false,
            dim_strength: 0.1,

            animations_enabled: true,
            animation_speed: 1.0,
            animation_style: "smooth".into(),

            kb_layout: "us".into(),
            kb_variant: String::new(),
            kb_options: String::new(),
            follow_mouse: 1,
            sensitivity: 0.0,
            natural_scroll: false,
            tap_to_click: true,
            numlock_by_default: true,
            workspace_gesture: true,

            disable_hyprland_logo: true,
            disable_splash: true,

            main_mod: "SUPER".into(),
            terminal: "kitty".into(),
            file_manager: "dolphin".into(),
            launcher: "wofi --show drun".into(),
            workspace_binds: true,
            binds: default_binds(),

            monitors: vec![Monitor {
                name: String::new(),
                resolution: "preferred".into(),
                position: "auto".into(),
                scale: "auto".into(),
            }],
            autostart: vec!["waybar".into()],
            window_rules: vec![WindowRule {
                field: "class".into(),
                pattern: "^(pavucontrol)$".into(),
                effect: "float".into(),
                value: String::new(),
            }],
            env: vec![
                EnvVar { name: "XCURSOR_SIZE".into(), value: "24".into() },
                EnvVar { name: "HYPRCURSOR_SIZE".into(), value: "24".into() },
            ],

            wallpaper_enabled: false,
            wallpaper: String::new(),
            wallpaper_fit: "cover".into(),

            extra: String::new(),
        }
    }
}

impl Settings {
    /// Applies language-specific defaults (keyboard layouts) to fresh settings.
    pub fn for_lang(lang: Lang) -> Self {
        let mut s = Self::default();
        match lang {
            Lang::Ru => {
                s.kb_layout = "us,ru".into();
                s.kb_options = "grp:alt_shift_toggle".into();
            }
            Lang::De => s.kb_layout = "de".into(),
            Lang::En => {}
        }
        s
    }
}

pub fn default_binds() -> Vec<Bind> {
    vec![
        Bind::new("$mainMod", "Return", "exec", "$terminal"),
        Bind::new("$mainMod", "Q", "killactive", ""),
        Bind::new("$mainMod SHIFT", "M", "exit", ""),
        Bind::new("$mainMod", "E", "exec", "$fileManager"),
        Bind::new("$mainMod", "R", "exec", "$menu"),
        Bind::new("$mainMod", "V", "togglefloating", ""),
        Bind::new("$mainMod", "F", "fullscreen", "0"),
        Bind::new("$mainMod", "P", "pseudo", ""),
        Bind::new("$mainMod", "J", "togglesplit", ""),
        Bind::new("$mainMod", "left", "movefocus", "l"),
        Bind::new("$mainMod", "right", "movefocus", "r"),
        Bind::new("$mainMod", "up", "movefocus", "u"),
        Bind::new("$mainMod", "down", "movefocus", "d"),
        Bind::new("$mainMod", "S", "togglespecialworkspace", "magic"),
        Bind::new("$mainMod SHIFT", "S", "movetoworkspace", "special:magic"),
        Bind::new("", "Print", "exec", "grim -g \"$(slurp)\" - | wl-copy"),
    ]
}

/// Lua string literal.
fn q(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A Lua number if the text is numeric, otherwise a Lua string.
fn num_or_str(s: &str) -> String {
    let t = s.trim();
    if !t.is_empty() && t.parse::<f64>().is_ok() { t.to_string() } else { q(t) }
}

/// hyprlang treats `#` as a comment start; `##` is a literal `#`. Newlines would end the value.
fn h(s: &str) -> String {
    s.replace('#', "##").replace(['\n', '\r'], " ")
}

fn lua_dir(d: &str) -> &str {
    match d.trim() {
        "l" | "left" => "left",
        "r" | "right" => "right",
        "u" | "t" | "up" => "up",
        "d" | "b" | "down" => "down",
        other => other,
    }
}

fn b(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

impl Settings {
    pub fn generate(&self, format: Format, lang: Lang) -> String {
        match format {
            Format::Lua => self.generate_lua(lang),
            _ => self.generate_hyprlang(lang),
        }
    }

    fn expand_vars(&self, s: &str) -> String {
        s.replace("$terminal", &self.terminal)
            .replace("$fileManager", &self.file_manager)
            .replace("$menu", &self.launcher)
            .replace("$mainMod", &self.main_mod)
    }

    fn anim_curve(&self) -> (&'static str, [f32; 4]) {
        match self.animation_style.as_str() {
            "bouncy" => ("easyBounce", [0.68, -0.55, 0.265, 1.55]),
            "snappy" => ("easySnap", [0.2, 1.0, 0.3, 1.0]),
            _ => ("easySmooth", [0.05, 0.9, 0.1, 1.05]),
        }
    }

    /// (leaf, base speed in deciseconds, use custom curve, style)
    fn anim_leaves(&self) -> Vec<(&'static str, f32, bool, &'static str)> {
        let k = self.animation_speed.clamp(0.1, 10.0);
        [
            ("windows", 6.0, true, "popin 80%"),
            ("windowsOut", 5.0, false, "popin 80%"),
            ("border", 10.0, false, ""),
            ("fade", 5.0, false, ""),
            ("workspaces", 5.0, true, "slide"),
        ]
        .into_iter()
        .map(|(leaf, base, custom, style)| (leaf, ((base / k) * 100.0).round() / 100.0, custom, style))
        .collect()
    }

    fn workspace_bind_list(&self) -> Vec<Bind> {
        let mut v = Vec::new();
        if self.workspace_binds {
            for i in 1..=10 {
                let key = (i % 10).to_string();
                v.push(Bind::new("$mainMod", &key, "workspace", &i.to_string()));
                v.push(Bind::new("$mainMod SHIFT", &key, "movetoworkspace", &i.to_string()));
            }
            v.push(Bind::new("$mainMod", "mouse_down", "workspace", "e+1"));
            v.push(Bind::new("$mainMod", "mouse_up", "workspace", "e-1"));
        }
        v
    }

    fn active_binds(&self) -> impl Iterator<Item = Bind> + '_ {
        self.binds
            .iter()
            .filter(|b| !b.key.trim().is_empty())
            .cloned()
            .chain(self.workspace_bind_list())
    }

    fn wallpaper_active(&self) -> bool {
        self.wallpaper_enabled && !self.wallpaper.trim().is_empty()
    }

    /// hyprpaper must be started once: only add it when autostart doesn't already.
    fn needs_hyprpaper_autostart(&self) -> bool {
        self.wallpaper_active()
            && !self.autostart.iter().any(|c| c.split_whitespace().next().is_some_and(|p| p.rsplit('/').next() == Some("hyprpaper")))
    }

    // ---------------------------------------------------------------- hyprlang

    fn generate_hyprlang(&self, lang: Lang) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "# {} (https://github.com/NeXtagG/easy-hyprland)", lang.t("gen.header1"));
        let _ = writeln!(s, "# {}\n", lang.t("gen.header2"));

        for m in &self.monitors {
            let _ = writeln!(s, "monitor = {},{},{},{}", h(&m.name), h(&m.resolution), h(&m.position), h(&m.scale));
        }
        s.push('\n');

        let _ = writeln!(s, "$terminal = {}", h(&self.terminal));
        let _ = writeln!(s, "$fileManager = {}", h(&self.file_manager));
        let _ = writeln!(s, "$menu = {}", h(&self.launcher));
        let _ = writeln!(s, "$mainMod = {}\n", h(&self.main_mod));

        for e in self.env.iter().filter(|e| !e.name.trim().is_empty()) {
            let _ = writeln!(s, "env = {},{}", h(e.name.trim()), h(&e.value));
        }
        s.push('\n');

        for cmd in self.autostart.iter().filter(|c| !c.trim().is_empty()) {
            let _ = writeln!(s, "exec-once = {}", h(cmd));
        }
        if self.needs_hyprpaper_autostart() {
            s.push_str("exec-once = hyprpaper\n");
        }
        s.push('\n');

        let active = if self.border_gradient {
            format!("{} {} {}deg", hypr_color(self.active_border_a), hypr_color(self.active_border_b), self.border_angle)
        } else {
            hypr_color(self.active_border_a)
        };
        let _ = writeln!(
            s,
            "general {{\n    gaps_in = {}\n    gaps_out = {}\n    border_size = {}\n    col.active_border = {}\n    col.inactive_border = {}\n    resize_on_border = {}\n    layout = {}\n}}\n",
            self.gaps_in,
            self.gaps_out,
            self.border_size,
            active,
            hypr_color(self.inactive_border),
            b(self.resize_on_border),
            h(&self.layout)
        );

        let _ = writeln!(
            s,
            "decoration {{\n    rounding = {}\n    active_opacity = {:.2}\n    inactive_opacity = {:.2}\n    dim_inactive = {}\n    dim_strength = {:.2}\n\n    shadow {{\n        enabled = {}\n        range = {}\n        render_power = {}\n        color = {}\n    }}\n\n    blur {{\n        enabled = {}\n        size = {}\n        passes = {}\n        vibrancy = {:.4}\n    }}\n}}\n",
            self.rounding,
            self.active_opacity,
            self.inactive_opacity,
            b(self.dim_inactive),
            self.dim_strength,
            b(self.shadow_enabled),
            self.shadow_range,
            self.shadow_power,
            hypr_color(self.shadow_color),
            b(self.blur_enabled),
            self.blur_size,
            self.blur_passes,
            self.blur_vibrancy
        );

        if self.animations_enabled {
            let (name, p) = self.anim_curve();
            let _ = writeln!(s, "animations {{\n    enabled = true\n    bezier = {name}, {}, {}, {}, {}", p[0], p[1], p[2], p[3]);
            for (leaf, speed, custom, style) in self.anim_leaves() {
                let curve = if custom { name } else { "default" };
                let style = if style.is_empty() { String::new() } else { format!(", {style}") };
                let _ = writeln!(s, "    animation = {leaf}, 1, {speed}, {curve}{style}");
            }
            s.push_str("}\n\n");
        } else {
            s.push_str("animations {\n    enabled = false\n}\n\n");
        }

        let _ = writeln!(
            s,
            "input {{\n    kb_layout = {}\n    kb_variant = {}\n    kb_options = {}\n    follow_mouse = {}\n    sensitivity = {:.2}\n    numlock_by_default = {}\n\n    touchpad {{\n        natural_scroll = {}\n        tap-to-click = {}\n    }}\n}}\n",
            h(&self.kb_layout),
            h(&self.kb_variant),
            h(&self.kb_options),
            self.follow_mouse,
            self.sensitivity,
            b(self.numlock_by_default),
            b(self.natural_scroll),
            b(self.tap_to_click)
        );
        if self.workspace_gesture {
            s.push_str("gesture = 3, horizontal, workspace\n\n");
        }

        let _ = writeln!(
            s,
            "misc {{\n    disable_hyprland_logo = {}\n    disable_splash_rendering = {}\n}}\n",
            b(self.disable_hyprland_logo),
            b(self.disable_splash)
        );

        for bind in self.active_binds() {
            let (disp, arg) = match bind.dispatcher.as_str() {
                // `togglesplit` is a layout message since Hyprland 0.53.
                "togglesplit" => ("layoutmsg".to_string(), "togglesplit".to_string()),
                d => (d.to_string(), bind.arg.clone()),
            };
            let _ = writeln!(s, "bind = {}, {}, {}, {}", h(&bind.mods), h(bind.key.trim()), disp, h(&arg));
        }
        if self.workspace_binds {
            s.push_str("bindm = $mainMod, mouse:272, movewindow\n");
            s.push_str("bindm = $mainMod, mouse:273, resizewindow\n");
        }
        s.push('\n');

        for (i, r) in self.window_rules.iter().enumerate().filter(|(_, r)| !r.effect.trim().is_empty()) {
            let value = match (effect_kind(&r.effect), r.value.trim()) {
                (EffectKind::Bool, "") => "true".to_string(),
                (_, v) => h(v),
            };
            let _ = writeln!(
                s,
                "windowrule {{\n    name = easy-hyprland-{}\n    match:{} = {}\n    {} = {}\n}}\n",
                i + 1,
                r.field,
                h(&r.pattern),
                r.effect.trim(),
                value
            );
        }

        if !self.extra.trim().is_empty() {
            let _ = writeln!(s, "# {}\n{}", lang.t("gen.extra"), self.extra);
        }
        s
    }

    // --------------------------------------------------------------------- Lua

    fn lua_keys(&self, bind: &Bind) -> String {
        let mods = self.expand_vars(&bind.mods);
        let mut parts: Vec<&str> = mods.split(|c: char| c.is_whitespace() || c == '+').filter(|p| !p.is_empty()).collect();
        parts.push(bind.key.trim());
        q(&parts.join(" + "))
    }

    fn lua_dispatch(&self, bind: &Bind) -> String {
        let arg = self.expand_vars(bind.arg.trim());
        let ws = |a: &str| num_or_str(a);
        match bind.dispatcher.as_str() {
            "exec" => format!("hl.dsp.exec_cmd({})", q(&arg)),
            "killactive" => "hl.dsp.window.close()".into(),
            "exit" => "hl.dsp.exit()".into(),
            "togglefloating" => "hl.dsp.window.float({ action = \"toggle\" })".into(),
            "fullscreen" => {
                let mode = if arg == "1" { "maximized" } else { "fullscreen" };
                format!("hl.dsp.window.fullscreen({{ mode = \"{mode}\" }})")
            }
            "pseudo" => "hl.dsp.window.pseudo()".into(),
            "togglesplit" => "hl.dsp.layout(\"togglesplit\")".into(),
            "layoutmsg" => format!("hl.dsp.layout({})", q(&arg)),
            "movefocus" => format!("hl.dsp.focus({{ direction = {} }})", q(lua_dir(&arg))),
            "movewindow" => format!("hl.dsp.window.move({{ direction = {} }})", q(lua_dir(&arg))),
            "workspace" => format!("hl.dsp.focus({{ workspace = {} }})", ws(&arg)),
            "movetoworkspace" => format!("hl.dsp.window.move({{ workspace = {} }})", ws(&arg)),
            "togglespecialworkspace" => {
                let name = arg.strip_prefix("special:").unwrap_or(&arg);
                format!("hl.dsp.workspace.toggle_special({})", q(if name.is_empty() { "magic" } else { name }))
            }
            other => format!("hl.dsp.exec_cmd({})", q(&format!("hyprctl dispatch {other} {arg}"))),
        }
    }

    fn generate_lua(&self, lang: Lang) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "-- {} (https://github.com/NeXtagG/easy-hyprland)", lang.t("gen.header1"));
        let _ = writeln!(s, "-- {}\n", lang.t("gen.header2"));

        for m in &self.monitors {
            let _ = writeln!(
                s,
                "hl.monitor({{ output = {}, mode = {}, position = {}, scale = {} }})",
                q(m.name.trim()),
                q(m.resolution.trim()),
                q(m.position.trim()),
                num_or_str(&m.scale)
            );
        }
        s.push('\n');

        for e in self.env.iter().filter(|e| !e.name.trim().is_empty()) {
            let _ = writeln!(s, "hl.env({}, {})", q(e.name.trim()), q(&e.value));
        }
        s.push('\n');

        let autostart: Vec<String> = self
            .autostart
            .iter()
            .filter(|c| !c.trim().is_empty())
            .map(|c| self.expand_vars(c))
            .chain(self.needs_hyprpaper_autostart().then(|| "hyprpaper".to_string()))
            .collect();
        if !autostart.is_empty() {
            s.push_str("hl.on(\"hyprland.start\", function()\n");
            for cmd in &autostart {
                let _ = writeln!(s, "    hl.exec_cmd({})", q(cmd));
            }
            s.push_str("end)\n\n");
        }

        let active = if self.border_gradient {
            format!(
                "{{ colors = {{ {}, {} }}, angle = {} }}",
                q(&hypr_color(self.active_border_a)),
                q(&hypr_color(self.active_border_b)),
                self.border_angle
            )
        } else {
            q(&hypr_color(self.active_border_a))
        };

        let _ = writeln!(
            s,
            "hl.config({{
    general = {{
        gaps_in = {},
        gaps_out = {},
        border_size = {},
        col = {{
            active_border = {},
            inactive_border = {},
        }},
        resize_on_border = {},
        layout = {},
    }},

    decoration = {{
        rounding = {},
        active_opacity = {:.2},
        inactive_opacity = {:.2},
        dim_inactive = {},
        dim_strength = {:.2},

        shadow = {{
            enabled = {},
            range = {},
            render_power = {},
            color = {},
        }},

        blur = {{
            enabled = {},
            size = {},
            passes = {},
            vibrancy = {:.4},
        }},
    }},

    animations = {{
        enabled = {},
    }},

    input = {{
        kb_layout = {},
        kb_variant = {},
        kb_options = {},
        follow_mouse = {},
        sensitivity = {:.2},
        numlock_by_default = {},

        touchpad = {{
            natural_scroll = {},
            tap_to_click = {},
        }},
    }},

    misc = {{
        disable_hyprland_logo = {},
        disable_splash_rendering = {},
    }},
}})
",
            self.gaps_in,
            self.gaps_out,
            self.border_size,
            active,
            q(&hypr_color(self.inactive_border)),
            b(self.resize_on_border),
            q(&self.layout),
            self.rounding,
            self.active_opacity,
            self.inactive_opacity,
            b(self.dim_inactive),
            self.dim_strength,
            b(self.shadow_enabled),
            self.shadow_range,
            self.shadow_power,
            q(&hypr_color(self.shadow_color)),
            b(self.blur_enabled),
            self.blur_size,
            self.blur_passes,
            self.blur_vibrancy,
            b(self.animations_enabled),
            q(&self.kb_layout),
            q(&self.kb_variant),
            q(&self.kb_options),
            self.follow_mouse,
            self.sensitivity,
            b(self.numlock_by_default),
            b(self.natural_scroll),
            b(self.tap_to_click),
            b(self.disable_hyprland_logo),
            b(self.disable_splash),
        );

        if self.animations_enabled {
            let (name, p) = self.anim_curve();
            let _ = writeln!(
                s,
                "hl.curve({}, {{ type = \"bezier\", points = {{ {{{}, {}}}, {{{}, {}}} }} }})",
                q(name),
                p[0],
                p[1],
                p[2],
                p[3]
            );
            for (leaf, speed, custom, style) in self.anim_leaves() {
                let curve = if custom { name } else { "default" };
                let style = if style.is_empty() { String::new() } else { format!(", style = {}", q(style)) };
                let _ = writeln!(
                    s,
                    "hl.animation({{ leaf = {}, enabled = true, speed = {}, bezier = {}{} }})",
                    q(leaf),
                    speed,
                    q(curve),
                    style
                );
            }
            s.push('\n');
        }

        if self.workspace_gesture {
            s.push_str("hl.gesture({ fingers = 3, direction = \"horizontal\", action = \"workspace\" })\n\n");
        }

        for bind in self.active_binds() {
            let _ = writeln!(s, "hl.bind({}, {})", self.lua_keys(&bind), self.lua_dispatch(&bind));
        }
        if self.workspace_binds {
            let m = q(&format!("{} + mouse:272", self.main_mod.split_whitespace().collect::<Vec<_>>().join(" + ")));
            let r = q(&format!("{} + mouse:273", self.main_mod.split_whitespace().collect::<Vec<_>>().join(" + ")));
            let _ = writeln!(s, "hl.bind({m}, hl.dsp.window.drag(), {{ mouse = true }})");
            let _ = writeln!(s, "hl.bind({r}, hl.dsp.window.resize(), {{ mouse = true }})");
        }
        s.push('\n');

        for (i, r) in self.window_rules.iter().enumerate().filter(|(_, r)| !r.effect.trim().is_empty()) {
            let value = match (effect_kind(&r.effect), r.value.trim()) {
                (EffectKind::Bool, "" | "true" | "on" | "1" | "yes") => "true".to_string(),
                (EffectKind::Bool, _) => "false".to_string(),
                (EffectKind::Int, v) if v.parse::<i64>().is_ok() => v.to_string(),
                (_, v) => q(v),
            };
            let _ = writeln!(
                s,
                "hl.window_rule({{\n    name = \"easy-hyprland-{}\",\n    match = {{ {} = {} }},\n    {} = {},\n}})\n",
                i + 1,
                r.field,
                q(&r.pattern),
                r.effect.trim(),
                value
            );
        }

        if !self.extra.trim().is_empty() {
            let _ = writeln!(s, "-- {}\n{}", lang.t("gen.extra"), self.extra);
        }
        s
    }

    pub fn generate_hyprpaper(&self, lang: Lang) -> String {
        format!(
            "# {}\n\nwallpaper {{\n    monitor =\n    path = {}\n    fit_mode = {}\n}}\n\nsplash = false\n",
            lang.t("gen.header1"),
            h(self.wallpaper.trim()),
            self.wallpaper_fit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_settings() -> Settings {
        let mut s = Settings::for_lang(Lang::Ru);
        s.window_rules = vec![
            WindowRule { field: "class".into(), pattern: "^(pavucontrol)$".into(), effect: "float".into(), value: String::new() },
            WindowRule { field: "title".into(), pattern: "^(Picture-in-Picture)$".into(), effect: "pin".into(), value: "true".into() },
            WindowRule { field: "class".into(), pattern: "^(kitty)$".into(), effect: "opacity".into(), value: "0.9 0.8".into() },
            WindowRule { field: "class".into(), pattern: "^(firefox)$".into(), effect: "rounding".into(), value: "0".into() },
        ];
        s.autostart.push("notify-send \"hi #1\"".into());
        s.monitors.push(Monitor { name: "DP-1".into(), resolution: "2560x1440@144".into(), position: "0x0".into(), scale: "1.25".into() });
        s.wallpaper_enabled = true;
        s.wallpaper = "~/Pictures/wall.png".into();
        s
    }

    #[test]
    fn hyprlang_has_all_sections() {
        let conf = full_settings().generate(Format::Hyprlang, Lang::En);
        for needle in [
            "general {",
            "decoration {",
            "animations {",
            "input {",
            "misc {",
            "monitor = ,preferred,auto,auto",
            "bind = $mainMod, Q, killactive, ",
            "bind = $mainMod, J, layoutmsg, togglesplit",
            "exec-once = waybar",
            "exec-once = hyprpaper",
            "gesture = 3, horizontal, workspace",
            "match:class = ^(pavucontrol)$",
            "tap-to-click = true",
            "notify-send \"hi ##1\"",
        ] {
            assert!(conf.contains(needle), "missing {needle}\n{conf}");
        }
        assert!(!conf.contains("windowrulev2"));
        assert!(!conf.contains("vfr"));
        assert_eq!(conf.matches('{').count(), conf.matches('}').count());
    }

    #[test]
    fn lua_has_all_sections() {
        let lua = full_settings().generate(Format::Lua, Lang::En);
        for needle in [
            "hl.config({",
            "hl.monitor({ output = \"\", mode = \"preferred\", position = \"auto\", scale = \"auto\" })",
            "hl.monitor({ output = \"DP-1\", mode = \"2560x1440@144\", position = \"0x0\", scale = 1.25 })",
            "hl.bind(\"SUPER + Q\", hl.dsp.window.close())",
            "hl.bind(\"SUPER + Return\", hl.dsp.exec_cmd(\"kitty\"))",
            "hl.bind(\"SUPER + SHIFT + 1\", hl.dsp.window.move({ workspace = 1 }))",
            "hl.bind(\"SUPER + left\", hl.dsp.focus({ direction = \"left\" }))",
            "hl.bind(\"Print\", hl.dsp.exec_cmd(\"grim -g \\\"$(slurp)\\\" - | wl-copy\"))",
            "hl.dsp.workspace.toggle_special(\"magic\")",
            "hl.exec_cmd(\"hyprpaper\")",
            "tap_to_click = true",
            "match = { class = \"^(pavucontrol)$\" },\n    float = true,",
            "opacity = \"0.9 0.8\",",
            "rounding = 0,",
        ] {
            assert!(lua.contains(needle), "missing {needle}\n{lua}");
        }
        assert!(!lua.contains("$mainMod") && !lua.contains("$terminal"));
    }

    #[test]
    fn hyprpaper_is_started_only_once() {
        let mut s = full_settings();
        for format in [Format::Lua, Format::Hyprlang] {
            let count = |s: &Settings| {
                s.generate(format, Lang::En)
                    .lines()
                    .filter(|l| (l.contains("exec-once") || l.contains("exec_cmd")) && l.contains("hyprpaper"))
                    .count()
            };
            s.autostart = vec!["waybar".into()];
            assert_eq!(count(&s), 1, "{format:?}: added when missing");
            s.autostart.push("/usr/bin/hyprpaper".into());
            assert_eq!(count(&s), 1, "{format:?}: not duplicated");
        }
    }

    #[test]
    fn hyprpaper_uses_block_syntax() {
        let p = full_settings().generate_hyprpaper(Lang::En);
        assert!(p.contains("wallpaper {") && p.contains("path = ~/Pictures/wall.png") && p.contains("fit_mode = cover"));
        assert!(!p.contains("preload"));
    }

    #[test]
    fn lua_strings_are_escaped() {
        assert_eq!(q("a\"b\\c\nd"), "\"a\\\"b\\\\c\\nd\"");
        assert_eq!(num_or_str("1.5"), "1.5");
        assert_eq!(num_or_str("auto"), "\"auto\"");
    }

    #[test]
    fn colors_are_hyprland_format() {
        assert_eq!(hypr_color([0x89, 0xb4, 0xfa, 0xff]), "rgba(89b4faff)");
    }

    /// Writes both formats for the external validation script (`tests/validate.sh`).
    #[test]
    fn write_samples_for_validation() {
        if let Ok(dir) = std::env::var("EASY_HYPRLAND_SAMPLES") {
            let dir = std::path::Path::new(&dir);
            std::fs::create_dir_all(dir).unwrap();
            for (name, s) in [("default", Settings::default()), ("full", full_settings())] {
                std::fs::write(dir.join(format!("{name}.lua")), s.generate(Format::Lua, Lang::En)).unwrap();
                std::fs::write(dir.join(format!("{name}.conf")), s.generate(Format::Hyprlang, Lang::En)).unwrap();
            }
            let mut off = full_settings();
            off.animations_enabled = false;
            off.border_gradient = false;
            off.workspace_binds = false;
            off.workspace_gesture = false;
            std::fs::write(dir.join("off.lua"), off.generate(Format::Lua, Lang::En)).unwrap();
            std::fs::write(dir.join("off.conf"), off.generate(Format::Hyprlang, Lang::En)).unwrap();
        }
    }
}
