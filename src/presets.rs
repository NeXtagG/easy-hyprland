use crate::config::Settings;

pub struct Preset {
    /// Display name, or a translation key when `translated` is true.
    pub name: &'static str,
    pub translated: bool,
    pub apply: fn(&mut Settings),
}

pub const PRESETS: &[Preset] = &[
    Preset {
        name: "Catppuccin",
        translated: false,
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
        translated: false,
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
        translated: false,
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
        name: "themes.neon",
        translated: true,
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
        name: "themes.minimal",
        translated: true,
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

impl Preset {
    /// Resets every look-related setting before applying the theme, so themes
    /// don't inherit leftovers from a previously applied one.
    pub fn apply_to(&self, s: &mut Settings) {
        let d = Settings::default();
        s.gaps_in = d.gaps_in;
        s.gaps_out = d.gaps_out;
        s.border_size = d.border_size;
        s.border_gradient = d.border_gradient;
        s.border_angle = d.border_angle;
        s.rounding = d.rounding;
        s.shadow_enabled = d.shadow_enabled;
        s.shadow_range = d.shadow_range;
        s.shadow_power = d.shadow_power;
        s.shadow_color = d.shadow_color;
        s.blur_enabled = d.blur_enabled;
        s.blur_size = d.blur_size;
        s.blur_passes = d.blur_passes;
        s.animations_enabled = d.animations_enabled;
        s.animation_style = d.animation_style;
        (self.apply)(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themes_do_not_inherit_from_each_other() {
        let minimal = PRESETS.iter().find(|p| p.name == "themes.minimal").unwrap();
        let mut s = Settings::default();
        minimal.apply_to(&mut s);
        assert!(!s.animations_enabled && !s.shadow_enabled && !s.border_gradient);
        PRESETS[0].apply_to(&mut s);
        assert!(s.animations_enabled && s.shadow_enabled && s.border_gradient);
        assert_eq!(s.border_size, Settings::default().border_size);
    }
}
