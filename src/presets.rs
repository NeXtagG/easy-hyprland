use crate::config::Settings;

pub struct Preset {
    pub name: &'static str,
    pub apply: fn(&mut Settings),
}

pub const PRESETS: &[Preset] = &[
    Preset {
        name: "Catppuccin",
        apply: |s| {
            s.active_border_a = [0x89, 0xb4, 0xfa, 0xff];
            s.active_border_b = [0xcb, 0xa6, 0xf7, 0xff];
            s.inactive_border = [0x45, 0x47, 0x5a, 0xaa];
            s.rounding = 10;
            s.gaps_in = 5;
            s.gaps_out = 15;
            s.blur_enabled = true;
            s.animation_style = "smooth".into();
        },
    },
    Preset {
        name: "Nord",
        apply: |s| {
            s.active_border_a = [0x88, 0xc0, 0xd0, 0xff];
            s.active_border_b = [0x81, 0xa1, 0xc1, 0xff];
            s.inactive_border = [0x3b, 0x42, 0x52, 0xaa];
            s.rounding = 8;
            s.gaps_in = 4;
            s.gaps_out = 10;
            s.blur_enabled = true;
            s.animation_style = "smooth".into();
        },
    },
    Preset {
        name: "Gruvbox",
        apply: |s| {
            s.active_border_a = [0xfa, 0xbd, 0x2f, 0xff];
            s.active_border_b = [0xfe, 0x80, 0x19, 0xff];
            s.inactive_border = [0x50, 0x49, 0x45, 0xaa];
            s.rounding = 4;
            s.gaps_in = 4;
            s.gaps_out = 8;
            s.blur_enabled = false;
            s.animation_style = "snappy".into();
        },
    },
    Preset {
        name: "Неон",
        apply: |s| {
            s.active_border_a = [0xff, 0x00, 0x99, 0xff];
            s.active_border_b = [0x00, 0xe5, 0xff, 0xff];
            s.inactive_border = [0x2a, 0x2a, 0x3a, 0xaa];
            s.border_size = 3;
            s.rounding = 14;
            s.gaps_in = 6;
            s.gaps_out = 18;
            s.blur_enabled = true;
            s.animation_style = "bouncy".into();
        },
    },
    Preset {
        name: "Минимализм",
        apply: |s| {
            s.active_border_a = [0xdd, 0xdd, 0xdd, 0xff];
            s.border_gradient = false;
            s.inactive_border = [0x33, 0x33, 0x33, 0xaa];
            s.border_size = 1;
            s.rounding = 0;
            s.gaps_in = 0;
            s.gaps_out = 0;
            s.blur_enabled = false;
            s.shadow_enabled = false;
            s.animations_enabled = false;
        },
    },
];
