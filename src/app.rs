use crate::config::{
    effect_kind, Bind, EffectKind, EnvVar, Format, Monitor, Rgba, Settings, WindowRule, DISPATCHERS, RULE_EFFECTS,
    RULE_FIELDS,
};
use crate::hypr::{self, ApplyError};
use crate::i18n::Lang;
use crate::presets;
use eframe::egui::{
    self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, StrokeKind, TextStyle, Vec2,
};
use std::time::{Duration, Instant};

const ACCENT: Color32 = Color32::from_rgb(0xb4, 0x9c, 0xff);
const BG: Color32 = Color32::from_rgb(0x16, 0x16, 0x22);
const PANEL: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x2e);
const CARD: Color32 = Color32::from_rgb(0x26, 0x26, 0x3a);
const MUTED: Color32 = Color32::from_rgb(0x9a, 0x9a, 0xb8);
const OK: Color32 = Color32::from_rgb(0xa6, 0xe3, 0xa1);
const ERR: Color32 = Color32::from_rgb(0xf3, 0x8b, 0xa8);
const WARN: Color32 = Color32::from_rgb(0xf9, 0xe2, 0xaf);

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Themes,
    Windows,
    Effects,
    Animations,
    Input,
    Keys,
    Monitors,
    Autostart,
    Rules,
    Env,
    Wallpaper,
    Config,
}

const PAGES: &[(Page, &str, &str)] = &[
    (Page::Themes, "🎨", "page.themes"),
    (Page::Windows, "🗗", "page.windows"),
    (Page::Effects, "✨", "page.effects"),
    (Page::Animations, "🎞", "page.animations"),
    (Page::Input, "⌨", "page.input"),
    (Page::Keys, "🔑", "page.keys"),
    (Page::Monitors, "🖥", "page.monitors"),
    (Page::Autostart, "🚀", "page.autostart"),
    (Page::Rules, "📐", "page.rules"),
    (Page::Env, "🌿", "page.env"),
    (Page::Wallpaper, "🖼", "page.wallpaper"),
    (Page::Config, "📄", "page.config"),
];

pub struct App {
    settings: Settings,
    saved: Settings,
    lang: Lang,
    page: Page,
    status: Option<(String, bool, Instant)>,
    confirm_close: bool,
    allow_close: bool,
    /// Main config we may not edit (Nix-managed / read-only) and the line to add there.
    manual_include: Option<(std::path::PathBuf, String)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_style(&cc.egui_ctx);
        let lang = std::env::var("EASY_HYPRLAND_LANG")
            .ok()
            .and_then(|l| Lang::from_code(&l))
            .or_else(hypr::load_lang)
            .unwrap_or_else(Lang::detect);
        let (settings, err) = hypr::load_settings(lang);
        let mut app = Self {
            saved: settings.clone(),
            settings,
            lang,
            page: Page::Themes,
            status: None,
            confirm_close: false,
            allow_close: false,
            manual_include: None,
        };
        // Opens a given page on start (used for screenshots), e.g. EASY_HYPRLAND_PAGE=rules.
        if let Ok(page) = std::env::var("EASY_HYPRLAND_PAGE")
            && let Some(p) = PAGES.iter().find(|p| p.2.ends_with(page.as_str()))
        {
            app.page = p.0;
        }
        if let Some(e) = err {
            app.set_status(lang.t("status.load_error").replace("{}", &e), false);
        }
        app
    }

    fn set_status(&mut self, text: impl Into<String>, ok: bool) {
        self.status = Some((text.into(), ok, Instant::now()));
    }

    /// Returns true when the files were written.
    fn save(&mut self, apply: bool) -> bool {
        let l = self.lang;
        let report = match hypr::save(&self.settings, l) {
            Ok(r) => r,
            Err(e) => {
                self.set_status(l.t("status.save_error").replace("{}", &e.to_string()), false);
                return false;
            }
        };
        self.saved = self.settings.clone();
        self.manual_include = report.manual_include.clone();
        let mut msg = l.t("status.saved").replace("{}", &report.path.display().to_string());
        for b in &report.backups {
            msg.push_str(" · ");
            msg.push_str(&l.t("status.backup").replace("{}", &b.display().to_string()));
        }
        if !apply {
            self.set_status(msg, true);
            return true;
        }
        match hypr::reload(&self.settings) {
            Ok(errors) if errors.is_empty() => self.set_status(format!("{msg} · {}", l.t("status.reloaded")), true),
            Ok(errors) => self.set_status(l.t("status.config_errors").replace("{}", &errors.join(" | ")), false),
            Err(ApplyError::NoHyprctl) => self.set_status(format!("{msg} · {}", l.t("status.no_hyprctl")), false),
            Err(ApplyError::Failed(e)) => self.set_status(format!("{msg} · hyprctl: {e}"), false),
        }
        true
    }
}

fn setup_style(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = PANEL;
    v.window_fill = PANEL;
    v.extreme_bg_color = BG;
    v.faint_bg_color = CARD;
    v.selection.bg_fill = ACCENT.linear_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
    v.hyperlink_color = ACCENT;
    let r = CornerRadius::same(8);
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = r;
    }
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(0x32, 0x32, 0x4a);
    v.widgets.inactive.bg_fill = Color32::from_rgb(0x32, 0x32, 0x4a);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(0x3e, 0x3e, 0x5a);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, ACCENT.linear_multiply(0.6));
    v.widgets.active.weak_bg_fill = ACCENT.linear_multiply(0.5);
    v.slider_trailing_fill = true;
    ctx.set_visuals(v);

    ctx.style_mut(|s| {
        s.spacing.item_spacing = Vec2::new(10.0, 10.0);
        s.spacing.button_padding = Vec2::new(12.0, 6.0);
        s.spacing.slider_width = 220.0;
        s.spacing.interact_size.y = 28.0;
        s.text_styles.insert(TextStyle::Body, FontId::proportional(15.0));
        s.text_styles.insert(TextStyle::Button, FontId::proportional(15.0));
        s.text_styles.insert(TextStyle::Heading, FontId::proportional(24.0));
        s.text_styles.insert(TextStyle::Monospace, FontId::monospace(13.5));
    });
}

fn card<R>(ui: &mut egui::Ui, title: &str, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(CARD)
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(16))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).size(17.0).strong().color(ACCENT));
            ui.add_space(4.0);
            add(ui)
        })
        .inner
}

fn hint(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).color(MUTED).size(13.0));
}

fn color_row(ui: &mut egui::Ui, label: &str, c: &mut Rgba) {
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba_unmultiplied(c);
        ui.label(label);
    });
}

fn c32(c: Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3])
}

/// Width left for a text field after reserving room for the delete button.
fn rest_width(ui: &egui::Ui) -> f32 {
    (ui.available_width() - 56.0).max(80.0)
}

/// A list of editable rows with delete buttons and an "add" button.
fn edit_list<T>(
    ui: &mut egui::Ui,
    l: Lang,
    items: &mut Vec<T>,
    new: impl Fn() -> T,
    mut row: impl FnMut(&mut egui::Ui, usize, &mut T),
) {
    let mut remove = None;
    for (i, item) in items.iter_mut().enumerate() {
        ui.push_id(i, |ui| {
            ui.horizontal(|ui| {
                row(ui, i, item);
                if ui.button("🗑").on_hover_text(l.t("btn.delete")).clicked() {
                    remove = Some(i);
                }
            });
        });
    }
    if let Some(i) = remove {
        items.remove(i);
    }
    if ui.button(l.t("btn.add")).clicked() {
        items.push(new());
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let l = self.lang;
        let dirty = self.settings != self.saved;

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            self.save(true);
        }

        if ctx.input(|i| i.viewport().close_requested()) && dirty && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_close = true;
        }

        egui::SidePanel::left("nav")
            .resizable(false)
            .exact_width(230.0)
            .frame(egui::Frame::new().fill(BG).inner_margin(Margin::same(14)))
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.label(RichText::new("Easy Hyprland").size(22.0).strong().color(ACCENT));
                hint(ui, l.t("app.tagline"));
                ui.add_space(14.0);
                for &(page, icon, key) in PAGES {
                    let selected = self.page == page;
                    let text = RichText::new(format!("{icon}  {}", l.t(key))).size(15.5);
                    let text = if selected { text.color(Color32::WHITE).strong() } else { text };
                    let btn = egui::Button::new(text)
                        .fill(if selected { ACCENT.linear_multiply(0.35) } else { Color32::TRANSPARENT })
                        .stroke(Stroke::NONE)
                        .min_size(Vec2::new(ui.available_width(), 34.0));
                    if ui.add(btn).clicked() {
                        self.page = page;
                    }
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    egui::ComboBox::from_id_salt("lang")
                        .selected_text(format!("🌐 {}", self.lang.native_name()))
                        .show_ui(ui, |ui| {
                            for lang in Lang::ALL {
                                if ui.selectable_value(&mut self.lang, lang, lang.native_name()).changed() {
                                    hypr::save_lang(lang);
                                }
                            }
                        });
                });
            });

        egui::TopBottomPanel::bottom("actions")
            .frame(egui::Frame::new().fill(BG).inner_margin(Margin::symmetric(16, 10)))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let primary = egui::Button::new(RichText::new(l.t("btn.save_apply")).strong().color(BG)).fill(ACCENT);
                    if ui.add(primary).on_hover_text("Ctrl+S").clicked() {
                        self.save(true);
                    }
                    if ui.button(l.t("btn.save")).clicked() {
                        self.save(false);
                    }
                    if ui.add_enabled(dirty, egui::Button::new(l.t("btn.revert"))).clicked() {
                        self.settings = self.saved.clone();
                    }
                    if ui.button(l.t("btn.defaults")).clicked() {
                        let format = self.settings.format;
                        self.settings = Settings::for_lang(l);
                        self.settings.format = format;
                    }
                });
                if let Some((path, line)) = &self.manual_include {
                    ui.label(RichText::new(l.t("status.manual_include").replace("{}", &path.display().to_string())).color(WARN));
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(line).monospace().color(Color32::WHITE));
                        if ui.button(l.t("btn.copy")).clicked() {
                            ui.ctx().copy_text(line.clone());
                        }
                    });
                }
                let shown = match &self.status {
                    Some((text, ok, at)) if at.elapsed() < Duration::from_secs(10) => {
                        ui.label(RichText::new(text).color(if *ok { OK } else { ERR }));
                        ctx.request_repaint_after(Duration::from_secs(1));
                        true
                    }
                    _ => false,
                };
                if !shown && dirty {
                    ui.label(RichText::new(l.t("status.unsaved")).color(WARN));
                }
            });

        if matches!(self.page, Page::Themes | Page::Windows | Page::Effects) && ctx.screen_rect().width() > 1000.0 {
            egui::SidePanel::right("preview")
                .resizable(false)
                .exact_width(360.0)
                .frame(egui::Frame::new().fill(PANEL).inner_margin(Margin::same(16)))
                .show(ctx, |ui| {
                    ui.label(RichText::new(l.t("preview.title")).size(17.0).strong().color(ACCENT));
                    let size = Vec2::new(ui.available_width(), ui.available_width() * 0.62);
                    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                    draw_preview(ui.painter(), rect, &self.settings);
                    hint(ui, l.t("preview.hint"));
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(PANEL).inner_margin(Margin::same(20)))
            .show(ctx, |ui| {
                let title = PAGES.iter().find(|p| p.0 == self.page).map(|p| l.t(p.2)).unwrap_or("");
                ui.heading(RichText::new(title).strong());
                ui.add_space(6.0);
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    self.page_ui(ui);
                });
            });

        if self.confirm_close {
            egui::Modal::new(egui::Id::new("confirm_close")).show(ctx, |ui| {
                ui.set_width(360.0);
                ui.heading(l.t("close.title"));
                ui.label(l.t("close.text"));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(RichText::new(l.t("close.save")).color(BG)).fill(ACCENT)).clicked() {
                        self.confirm_close = false;
                        if self.save(false) {
                            self.allow_close = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                    if ui.button(l.t("close.discard")).clicked() {
                        self.allow_close = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button(l.t("close.cancel")).clicked() {
                        self.confirm_close = false;
                    }
                });
            });
        }
    }
}

impl App {
    fn page_ui(&mut self, ui: &mut egui::Ui) {
        let l = self.lang;
        let s = &mut self.settings;
        match self.page {
            Page::Themes => {
                card(ui, l.t("themes.card"), |ui| {
                    hint(ui, l.t("themes.hint"));
                    ui.horizontal_wrapped(|ui| {
                        for p in presets::PRESETS {
                            let name = if p.translated { l.t(p.name) } else { p.name };
                            if ui.add(egui::Button::new(RichText::new(name).size(16.0)).min_size(Vec2::new(140.0, 44.0))).clicked() {
                                p.apply_to(s);
                            }
                        }
                    });
                });
                ui.add_space(8.0);
                card(ui, l.t("format.card"), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.selectable_value(&mut s.format, Format::Auto, l.t("format.auto"));
                        ui.selectable_value(&mut s.format, Format::Lua, l.t("format.lua"));
                        ui.selectable_value(&mut s.format, Format::Hyprlang, l.t("format.conf"));
                    });
                    hint(ui, l.t("format.hint"));
                    let resolved = hypr::resolve_format(s.format);
                    let path = hypr::hypr_dir().join(hypr::generated_file_name(resolved));
                    ui.label(RichText::new(l.t("format.detected").replace("{}", &path.display().to_string())).color(OK).size(13.0));
                });
                ui.add_space(8.0);
                card(ui, l.t("howto.card"), |ui| {
                    hint(ui, l.t("howto.1"));
                    hint(ui, l.t("howto.2"));
                    hint(ui, l.t("howto.3"));
                });
            }
            Page::Windows => {
                card(ui, l.t("win.gaps_card"), |ui| {
                    ui.add(egui::Slider::new(&mut s.gaps_in, 0..=40).text(l.t("win.gaps_in")));
                    ui.add(egui::Slider::new(&mut s.gaps_out, 0..=80).text(l.t("win.gaps_out")));
                    ui.add(egui::Slider::new(&mut s.border_size, 0..=10).text(l.t("win.border_size")));
                    ui.add(egui::Slider::new(&mut s.rounding, 0..=30).text(l.t("win.rounding")));
                    ui.checkbox(&mut s.resize_on_border, l.t("win.resize_on_border"));
                });
                ui.add_space(8.0);
                card(ui, l.t("win.colors_card"), |ui| {
                    color_row(ui, l.t("win.active"), &mut s.active_border_a);
                    ui.checkbox(&mut s.border_gradient, l.t("win.gradient"));
                    if s.border_gradient {
                        color_row(ui, l.t("win.second_color"), &mut s.active_border_b);
                        ui.add(egui::Slider::new(&mut s.border_angle, 0..=360).text(l.t("win.angle")));
                    }
                    color_row(ui, l.t("win.inactive"), &mut s.inactive_border);
                });
                ui.add_space(8.0);
                card(ui, l.t("win.layout_card"), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for (id, key) in [("dwindle", "win.dwindle"), ("master", "win.master"), ("scrolling", "win.scrolling"), ("monocle", "win.monocle")] {
                            ui.selectable_value(&mut s.layout, id.to_string(), l.t(key));
                        }
                    });
                    hint(ui, l.t("win.layout_hint"));
                });
            }
            Page::Effects => {
                card(ui, l.t("fx.opacity_card"), |ui| {
                    ui.add(egui::Slider::new(&mut s.active_opacity, 0.3..=1.0).text(l.t("fx.active")));
                    ui.add(egui::Slider::new(&mut s.inactive_opacity, 0.3..=1.0).text(l.t("fx.inactive")));
                    ui.checkbox(&mut s.dim_inactive, l.t("fx.dim"));
                    if s.dim_inactive {
                        ui.add(egui::Slider::new(&mut s.dim_strength, 0.0..=1.0).text(l.t("fx.dim_strength")));
                    }
                });
                ui.add_space(8.0);
                card(ui, l.t("fx.blur_card"), |ui| {
                    ui.checkbox(&mut s.blur_enabled, l.t("fx.blur_enable"));
                    ui.add_enabled_ui(s.blur_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.blur_size, 1..=20).text(l.t("fx.blur_size")));
                        ui.add(egui::Slider::new(&mut s.blur_passes, 1..=6).text(l.t("fx.blur_passes")));
                        ui.add(egui::Slider::new(&mut s.blur_vibrancy, 0.0..=1.0).text(l.t("fx.blur_vibrancy")));
                    });
                    hint(ui, l.t("fx.blur_hint"));
                });
                ui.add_space(8.0);
                card(ui, l.t("fx.shadow_card"), |ui| {
                    ui.checkbox(&mut s.shadow_enabled, l.t("fx.shadow_enable"));
                    ui.add_enabled_ui(s.shadow_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.shadow_range, 1..=40).text(l.t("fx.shadow_range")));
                        ui.add(egui::Slider::new(&mut s.shadow_power, 1..=4).text(l.t("fx.shadow_power")));
                        color_row(ui, l.t("fx.shadow_color"), &mut s.shadow_color);
                    });
                });
            }
            Page::Animations => {
                card(ui, l.t("anim.card"), |ui| {
                    ui.checkbox(&mut s.animations_enabled, l.t("anim.enable"));
                    ui.add_enabled_ui(s.animations_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.animation_speed, 0.25..=3.0).text(l.t("anim.speed")));
                        ui.label(l.t("anim.style"));
                        ui.horizontal_wrapped(|ui| {
                            ui.selectable_value(&mut s.animation_style, "smooth".into(), l.t("anim.smooth"));
                            ui.selectable_value(&mut s.animation_style, "snappy".into(), l.t("anim.snappy"));
                            ui.selectable_value(&mut s.animation_style, "bouncy".into(), l.t("anim.bouncy"));
                        });
                    });
                });
            }
            Page::Input => {
                card(ui, l.t("input.kb_card"), |ui| {
                    egui::Grid::new("kb").num_columns(2).spacing([12.0, 10.0]).show(ui, |ui| {
                        ui.label(l.t("input.layouts"));
                        ui.text_edit_singleline(&mut s.kb_layout);
                        ui.end_row();
                        ui.label(l.t("input.variants"));
                        ui.text_edit_singleline(&mut s.kb_variant);
                        ui.end_row();
                        ui.label(l.t("input.options"));
                        ui.text_edit_singleline(&mut s.kb_options);
                        ui.end_row();
                    });
                    hint(ui, l.t("input.kb_hint"));
                    ui.horizontal_wrapped(|ui| {
                        ui.label(l.t("input.switch"));
                        for (opt, name) in [
                            ("grp:alt_shift_toggle", "Alt+Shift"),
                            ("grp:ctrl_shift_toggle", "Ctrl+Shift"),
                            ("grp:win_space_toggle", "Win+Space"),
                            ("grp:caps_toggle", "CapsLock"),
                        ] {
                            // Keep other xkb options (like caps:escape) while switching the group toggle.
                            let mut opts: Vec<String> = s.kb_options.split(',').map(|o| o.trim().to_string()).filter(|o| !o.is_empty()).collect();
                            let active = opts.iter().any(|o| o == opt);
                            if ui.selectable_label(active, name).clicked() {
                                opts.retain(|o| !o.starts_with("grp:"));
                                if !active {
                                    opts.push(opt.to_string());
                                }
                                s.kb_options = opts.join(",");
                            }
                        }
                    });
                    ui.checkbox(&mut s.numlock_by_default, l.t("input.numlock"));
                });
                ui.add_space(8.0);
                card(ui, l.t("input.mouse_card"), |ui| {
                    ui.add(egui::Slider::new(&mut s.sensitivity, -1.0..=1.0).text(l.t("input.sensitivity")));
                    ui.horizontal_wrapped(|ui| {
                        ui.label(l.t("input.follow"));
                        ui.selectable_value(&mut s.follow_mouse, 0, l.t("input.follow_off"));
                        ui.selectable_value(&mut s.follow_mouse, 1, l.t("input.follow_always"));
                        ui.selectable_value(&mut s.follow_mouse, 2, l.t("input.follow_loose"));
                    });
                    ui.checkbox(&mut s.natural_scroll, l.t("input.natural"));
                    ui.checkbox(&mut s.tap_to_click, l.t("input.tap"));
                    ui.checkbox(&mut s.workspace_gesture, l.t("input.gesture"));
                });
            }
            Page::Keys => {
                card(ui, l.t("keys.apps_card"), |ui| {
                    egui::Grid::new("apps").num_columns(2).spacing([12.0, 10.0]).show(ui, |ui| {
                        ui.label(l.t("keys.main_mod"));
                        egui::ComboBox::from_id_salt("mod").selected_text(&s.main_mod).show_ui(ui, |ui| {
                            for m in ["SUPER", "ALT", "CTRL", "SUPER SHIFT"] {
                                ui.selectable_value(&mut s.main_mod, m.into(), m);
                            }
                        });
                        ui.end_row();
                        ui.label(l.t("keys.terminal"));
                        ui.text_edit_singleline(&mut s.terminal);
                        ui.end_row();
                        ui.label(l.t("keys.files"));
                        ui.text_edit_singleline(&mut s.file_manager);
                        ui.end_row();
                        ui.label(l.t("keys.menu"));
                        ui.text_edit_singleline(&mut s.launcher);
                        ui.end_row();
                    });
                });
                ui.add_space(8.0);
                card(ui, l.t("keys.binds_card"), |ui| {
                    hint(ui, l.t("keys.binds_hint"));
                    edit_list(ui, l, &mut s.binds, || Bind::new("$mainMod", "", "exec", ""), |ui, i, b| {
                        ui.add(egui::TextEdit::singleline(&mut b.mods).desired_width(130.0));
                        ui.add(egui::TextEdit::singleline(&mut b.key).desired_width(80.0).hint_text(l.t("keys.key")));
                        egui::ComboBox::from_id_salt(("disp", i)).width(150.0).selected_text(&b.dispatcher).show_ui(ui, |ui| {
                            for d in DISPATCHERS {
                                ui.selectable_value(&mut b.dispatcher, d.to_string(), *d);
                            }
                        });
                        ui.add(egui::TextEdit::singleline(&mut b.arg).desired_width(rest_width(ui)));
                    });
                    ui.separator();
                    ui.checkbox(&mut s.workspace_binds, l.t("keys.workspaces"));
                });
            }
            Page::Monitors => {
                card(ui, l.t("mon.card"), |ui| {
                    hint(ui, l.t("mon.hint1"));
                    hint(ui, l.t("mon.hint2"));
                    edit_list(
                        ui,
                        l,
                        &mut s.monitors,
                        || Monitor { name: "HDMI-A-1".into(), resolution: "preferred".into(), position: "auto".into(), scale: "1".into() },
                        |ui, _, m| {
                            ui.add(egui::TextEdit::singleline(&mut m.name).desired_width(120.0).hint_text(l.t("mon.name")));
                            ui.add(egui::TextEdit::singleline(&mut m.resolution).desired_width(150.0));
                            ui.add(egui::TextEdit::singleline(&mut m.position).desired_width(90.0));
                            ui.add(egui::TextEdit::singleline(&mut m.scale).desired_width(60.0));
                        },
                    );
                });
            }
            Page::Autostart => {
                card(ui, l.t("auto.card"), |ui| {
                    hint(ui, l.t("auto.hint"));
                    edit_list(ui, l, &mut s.autostart, String::new, |ui, _, cmd| {
                        ui.add(egui::TextEdit::singleline(cmd).desired_width(rest_width(ui)));
                    });
                });
            }
            Page::Rules => {
                card(ui, l.t("rules.card"), |ui| {
                    hint(ui, l.t("rules.hint"));
                    edit_list(ui, l, &mut s.window_rules, WindowRule::default, |ui, i, r| {
                        egui::ComboBox::from_id_salt(("field", i)).width(110.0).selected_text(&r.field).show_ui(ui, |ui| {
                            for f in RULE_FIELDS {
                                ui.selectable_value(&mut r.field, f.to_string(), *f);
                            }
                        });
                        ui.add(egui::TextEdit::singleline(&mut r.pattern).desired_width(180.0).hint_text("^(app)$"));
                        ui.label("->");
                        egui::ComboBox::from_id_salt(("effect", i)).width(130.0).selected_text(&r.effect).show_ui(ui, |ui| {
                            for (e, _) in RULE_EFFECTS {
                                ui.selectable_value(&mut r.effect, e.to_string(), *e);
                            }
                        });
                        if effect_kind(&r.effect) == EffectKind::Bool {
                            let mut on = !matches!(r.value.trim(), "false" | "off" | "0" | "no");
                            if ui.checkbox(&mut on, l.t("rules.value_bool")).changed() {
                                r.value = if on { String::new() } else { "false".into() };
                            }
                        } else {
                            ui.add(egui::TextEdit::singleline(&mut r.value).desired_width(rest_width(ui)).hint_text(l.t("rules.value")));
                        }
                    });
                });
            }
            Page::Env => {
                card(ui, l.t("env.card"), |ui| {
                    hint(ui, l.t("env.hint"));
                    edit_list(ui, l, &mut s.env, || EnvVar { name: String::new(), value: String::new() }, |ui, _, e| {
                        ui.add(egui::TextEdit::singleline(&mut e.name).desired_width(220.0).hint_text(l.t("env.name")));
                        ui.label("=");
                        ui.add(egui::TextEdit::singleline(&mut e.value).desired_width(rest_width(ui)));
                    });
                });
            }
            Page::Wallpaper => {
                card(ui, l.t("wp.card"), |ui| {
                    ui.checkbox(&mut s.wallpaper_enabled, l.t("wp.enable"));
                    ui.add_enabled_ui(s.wallpaper_enabled, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(l.t("wp.path"));
                            ui.add(egui::TextEdit::singleline(&mut s.wallpaper).desired_width(ui.available_width()).hint_text("~/Pictures/wallpaper.png"));
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.label(l.t("wp.fit"));
                            for fit in ["cover", "contain", "fill", "tile"] {
                                ui.selectable_value(&mut s.wallpaper_fit, fit.to_string(), fit);
                            }
                        });
                    });
                    hint(ui, l.t("wp.hint"));
                });
            }
            Page::Config => {
                let format = hypr::resolve_format(s.format);
                let name = hypr::generated_file_name(format);
                card(ui, l.t("cfg.extra_card"), |ui| {
                    hint(ui, &l.t("cfg.extra_hint").replace("{}", if format == Format::Lua { "Lua" } else { "hyprlang" }));
                    ui.add(egui::TextEdit::multiline(&mut s.extra).code_editor().desired_rows(5).desired_width(f32::INFINITY));
                });
                ui.add_space(8.0);
                let text = s.generate(format, l);
                let mut copied = false;
                card(ui, name, |ui| {
                    if ui.button(l.t("btn.copy")).clicked() {
                        ui.ctx().copy_text(text.clone());
                        copied = true;
                    }
                    let mut shown = text.as_str();
                    ui.add(egui::TextEdit::multiline(&mut shown).code_editor().desired_width(f32::INFINITY));
                });
                if copied {
                    self.set_status(l.t("btn.copied"), true);
                }
            }
        }
    }
}

/// Draws a small fake desktop with three tiled windows using the current settings.
fn draw_preview(p: &egui::Painter, rect: Rect, s: &Settings) {
    let scale = 0.5;
    let mut mesh = egui::Mesh::default();
    let top = Color32::from_rgb(0x3a, 0x2a, 0x6a);
    let bottom = Color32::from_rgb(0x12, 0x4a, 0x6a);
    mesh.colored_vertex(rect.left_top(), top);
    mesh.colored_vertex(rect.right_top(), top);
    mesh.colored_vertex(rect.right_bottom(), bottom);
    mesh.colored_vertex(rect.left_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    p.add(mesh);

    let bar = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 14.0));
    p.rect_filled(bar, CornerRadius::ZERO, Color32::from_black_alpha(140));

    let outer = (s.gaps_out as f32 * scale).min(rect.height() * 0.25);
    let area = Rect::from_min_max(Pos2::new(rect.min.x, bar.max.y), rect.max).shrink(outer);
    let gi = (s.gaps_in as f32 * scale).min(area.height() * 0.2);
    let c = area.center();
    let windows: Vec<Rect> = if s.layout == "monocle" {
        vec![area]
    } else {
        vec![
            Rect::from_min_max(area.min, Pos2::new(c.x - gi, area.max.y)),
            Rect::from_min_max(Pos2::new(c.x + gi, area.min.y), Pos2::new(area.max.x, c.y - gi)),
            Rect::from_min_max(Pos2::new(c.x + gi, c.y + gi), area.max),
        ]
    };

    let radius = CornerRadius::same((s.rounding as f32 * scale).min(40.0) as u8);
    for (i, w) in windows.into_iter().enumerate() {
        let active = i == 0;
        if s.shadow_enabled {
            let range = s.shadow_range as f32 * scale;
            let steps = 6;
            for k in (1..=steps).rev() {
                let t = k as f32 / steps as f32;
                let alpha = (s.shadow_color[3] as f32 * (1.0 - t) * 0.35) as u8;
                p.rect_filled(w.expand(range * t), radius, Color32::from_rgba_unmultiplied(s.shadow_color[0], s.shadow_color[1], s.shadow_color[2], alpha));
            }
        }
        let mut opacity = if active { s.active_opacity } else { s.inactive_opacity };
        if s.dim_inactive && !active {
            opacity *= 1.0 - s.dim_strength * 0.5;
        }
        p.rect_filled(w, radius, Color32::from_rgba_unmultiplied(0x1e, 0x1e, 0x2e, (opacity * 255.0) as u8));

        let line = Color32::from_white_alpha((60.0 * opacity) as u8);
        for (n, frac) in [0.7, 0.5, 0.6].into_iter().enumerate() {
            let y = w.min.y + 12.0 + n as f32 * 9.0;
            if y + 4.0 < w.max.y {
                let len = (w.width() - 24.0) * frac;
                p.rect_filled(Rect::from_min_size(Pos2::new(w.min.x + 12.0, y), Vec2::new(len.max(0.0), 4.0)), CornerRadius::same(2), line);
            }
        }

        if s.border_size > 0 {
            // Borders are drawn a bit thicker than the scaled preview so their colors stay readable.
            let width = (s.border_size as f32 * 0.9).max(1.0);
            if active && s.border_gradient {
                gradient_border(p, w, radius.nw as f32, width, c32(s.active_border_a), c32(s.active_border_b), s.border_angle as f32);
            } else {
                let color = if active { c32(s.active_border_a) } else { c32(s.inactive_border) };
                p.rect_stroke(w, radius, Stroke::new(width, color), StrokeKind::Inside);
            }
        }
    }
}

/// Points along a rounded rectangle, clockwise from the top-left corner.
/// Every call with the same `steps` returns the same number of points.
fn rounded_rect_points(rect: Rect, radius: f32, steps: usize) -> Vec<Pos2> {
    let r = radius.clamp(0.0, rect.width().min(rect.height()) / 2.0);
    let corners = [
        (Pos2::new(rect.min.x + r, rect.min.y + r), 180.0_f32),
        (Pos2::new(rect.max.x - r, rect.min.y + r), 270.0),
        (Pos2::new(rect.max.x - r, rect.max.y - r), 0.0),
        (Pos2::new(rect.min.x + r, rect.max.y - r), 90.0),
    ];
    let mut pts = Vec::with_capacity(4 * (steps + 1));
    for (center, start) in corners {
        for i in 0..=steps {
            let a = (start + 90.0 * i as f32 / steps as f32).to_radians();
            pts.push(center + Vec2::new(a.cos(), a.sin()) * r);
        }
    }
    pts
}

/// Draws a border whose color runs from `a` to `b` along `angle_deg`, like Hyprland's gradient borders.
fn gradient_border(p: &egui::Painter, rect: Rect, radius: f32, width: f32, a: Color32, b: Color32, angle_deg: f32) {
    const STEPS: usize = 8;
    let outer = rounded_rect_points(rect, radius, STEPS);
    let inner = rounded_rect_points(rect.shrink(width), (radius - width).max(0.0), STEPS);
    // Hyprland measures the angle clockwise from "left to right".
    let dir = Vec2::angled(angle_deg.to_radians());
    let half = (rect.width() * dir.x.abs() + rect.height() * dir.y.abs()) / 2.0;
    let color = |pt: Pos2| {
        let t = ((pt - rect.center()).dot(dir) / half.max(1.0) * 0.5 + 0.5).clamp(0.0, 1.0);
        a.lerp_to_gamma(b, t)
    };
    let mut mesh = egui::Mesh::default();
    for (o, i) in outer.iter().zip(&inner) {
        mesh.colored_vertex(*o, color(*o));
        mesh.colored_vertex(*i, color(*i));
    }
    let n = outer.len() as u32;
    for k in 0..n {
        let (o0, i0) = (2 * k, 2 * k + 1);
        let (o1, i1) = (2 * ((k + 1) % n), 2 * ((k + 1) % n) + 1);
        mesh.add_triangle(o0, i0, o1);
        mesh.add_triangle(i0, i1, o1);
    }
    p.add(mesh);
}
