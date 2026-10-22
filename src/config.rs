use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub type Rgba = [u8; 4];

pub fn hypr_color(c: Rgba) -> String {
    format!("rgba({:02x}{:02x}{:02x}{:02x})", c[0], c[1], c[2], c[3])
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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Monitor {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowRule {
    pub rule: String,
    pub matcher: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
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

    // misc
    pub disable_hyprland_logo: bool,
    pub disable_splash: bool,
    pub vfr: bool,

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

    pub extra: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
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

            kb_layout: "us,ru".into(),
            kb_variant: String::new(),
            kb_options: "grp:alt_shift_toggle".into(),
            follow_mouse: 1,
            sensitivity: 0.0,
            natural_scroll: false,
            tap_to_click: true,
            numlock_by_default: true,

            disable_hyprland_logo: true,
            disable_splash: true,
            vfr: true,

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
                scale: "1".into(),
            }],
            autostart: vec!["waybar".into()],
            window_rules: vec![WindowRule {
                rule: "float".into(),
                matcher: "class:^(pavucontrol)$".into(),
            }],
            env: vec![
                EnvVar { name: "XCURSOR_SIZE".into(), value: "24".into() },
                EnvVar { name: "HYPRCURSOR_SIZE".into(), value: "24".into() },
            ],

            wallpaper_enabled: false,
            wallpaper: String::new(),

            extra: String::new(),
        }
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
        Bind::new("", "Print", "exec", "grim -g \"$(slurp)\" - | wl-copy"),
    ]
}

impl Settings {
    /// Builds the contents of easy-hyprland.conf.
    pub fn generate(&self) -> String {
        let mut s = String::new();
        let b = |v: bool| if v { "true" } else { "false" };

        s.push_str("# Сгенерировано Easy Hyprland — https://github.com/NeXtagG/easy-hyprland\n");
        s.push_str("# Не редактируйте вручную: изменения перезапишутся при следующем сохранении.\n\n");

        for m in &self.monitors {
            let _ = writeln!(s, "monitor = {},{},{},{}", m.name, m.resolution, m.position, m.scale);
        }
        s.push('\n');

        let _ = writeln!(s, "$terminal = {}", self.terminal);
        let _ = writeln!(s, "$fileManager = {}", self.file_manager);
        let _ = writeln!(s, "$menu = {}", self.launcher);
        let _ = writeln!(s, "$mainMod = {}\n", self.main_mod);

        for e in self.env.iter().filter(|e| !e.name.is_empty()) {
            let _ = writeln!(s, "env = {},{}", e.name, e.value);
        }
        s.push('\n');

        for cmd in self.autostart.iter().filter(|c| !c.trim().is_empty()) {
            let _ = writeln!(s, "exec-once = {cmd}");
        }
        if self.wallpaper_enabled && !self.wallpaper.is_empty() {
            s.push_str("exec-once = hyprpaper\n");
        }
        s.push('\n');

        let active = if self.border_gradient {
            format!(
                "{} {} {}deg",
                hypr_color(self.active_border_a),
                hypr_color(self.active_border_b),
                self.border_angle
            )
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
            self.layout
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

        s.push_str(&self.generate_animations());

        let _ = writeln!(
            s,
            "input {{\n    kb_layout = {}\n    kb_variant = {}\n    kb_options = {}\n    follow_mouse = {}\n    sensitivity = {:.2}\n    numlock_by_default = {}\n\n    touchpad {{\n        natural_scroll = {}\n        tap-to-click = {}\n    }}\n}}\n",
            self.kb_layout,
            self.kb_variant,
            self.kb_options,
            self.follow_mouse,
            self.sensitivity,
            b(self.numlock_by_default),
            b(self.natural_scroll),
            b(self.tap_to_click)
        );

        let _ = writeln!(
            s,
            "misc {{\n    disable_hyprland_logo = {}\n    disable_splash_rendering = {}\n    vfr = {}\n}}\n",
            b(self.disable_hyprland_logo),
            b(self.disable_splash),
            b(self.vfr)
        );

        for bind in self.binds.iter().filter(|b| !b.key.is_empty()) {
            let _ = writeln!(s, "bind = {}, {}, {}, {}", bind.mods, bind.key, bind.dispatcher, bind.arg);
        }
        if self.workspace_binds {
            s.push('\n');
            for i in 1..=10 {
                let key = i % 10;
                let _ = writeln!(s, "bind = $mainMod, {key}, workspace, {i}");
                let _ = writeln!(s, "bind = $mainMod SHIFT, {key}, movetoworkspace, {i}");
            }
            s.push_str("bind = $mainMod, mouse_down, workspace, e+1\n");
            s.push_str("bind = $mainMod, mouse_up, workspace, e-1\n");
            s.push_str("bindm = $mainMod, mouse:272, movewindow\n");
            s.push_str("bindm = $mainMod, mouse:273, resizewindow\n");
        }
        s.push('\n');

        for r in self.window_rules.iter().filter(|r| !r.rule.is_empty()) {
            let _ = writeln!(s, "windowrulev2 = {}, {}", r.rule, r.matcher);
        }

        if !self.extra.trim().is_empty() {
            s.push_str("\n# Свои строки\n");
            s.push_str(&self.extra);
            s.push('\n');
        }
        s
    }

    fn generate_animations(&self) -> String {
        if !self.animations_enabled {
            return "animations {\n    enabled = false\n}\n\n".into();
        }
        let (bezier, curve) = match self.animation_style.as_str() {
            "bouncy" => ("bounce", "0.68, -0.55, 0.265, 1.55"),
            "snappy" => ("snap", "0.2, 1, 0.3, 1"),
            _ => ("smooth", "0.05, 0.9, 0.1, 1.05"),
        };
        // Hyprland speed is in deciseconds: larger means slower.
        let d = |base: f32| ((base / self.animation_speed.max(0.1)).round() as i32).max(1);
        format!(
            "animations {{\n    enabled = true\n    bezier = {bezier}, {curve}\n    animation = windows, 1, {}, {bezier}, popin 80%\n    animation = windowsOut, 1, {}, default, popin 80%\n    animation = border, 1, {}, default\n    animation = fade, 1, {}, default\n    animation = workspaces, 1, {}, {bezier}, slide\n}}\n\n",
            d(6.0),
            d(5.0),
            d(10.0),
            d(5.0),
            d(5.0)
        )
    }

    pub fn generate_hyprpaper(&self) -> String {
        format!(
            "# Сгенерировано Easy Hyprland\npreload = {0}\nwallpaper = ,{0}\nsplash = false\n",
            self.wallpaper
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_all_sections() {
        let conf = Settings::default().generate();
        for section in ["general {", "decoration {", "animations {", "input {", "misc {", "monitor = ", "bind = $mainMod, Q, killactive", "exec-once = waybar"] {
            assert!(conf.contains(section), "missing {section}");
        }
        assert_eq!(conf.matches('{').count(), conf.matches('}').count());
    }

    #[test]
    fn colors_are_hyprland_format() {
        assert_eq!(hypr_color([0x89, 0xb4, 0xfa, 0xff]), "rgba(89b4faff)");
    }
}
