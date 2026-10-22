use crate::config::{Bind, EnvVar, Monitor, Rgba, Settings, WindowRule};
use crate::{hypr, presets};
use eframe::egui::{
    self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, StrokeKind,
    TextStyle, Vec2,
};
use std::time::Instant;

const ACCENT: Color32 = Color32::from_rgb(0xb4, 0x9c, 0xff);
const BG: Color32 = Color32::from_rgb(0x16, 0x16, 0x22);
const PANEL: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x2e);
const CARD: Color32 = Color32::from_rgb(0x26, 0x26, 0x3a);
const MUTED: Color32 = Color32::from_rgb(0x9a, 0x9a, 0xb8);

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
    (Page::Themes, "🎨", "Темы"),
    (Page::Windows, "🗗", "Окна"),
    (Page::Effects, "✨", "Эффекты"),
    (Page::Animations, "🎞", "Анимации"),
    (Page::Input, "⌨", "Ввод"),
    (Page::Keys, "🔑", "Горячие клавиши"),
    (Page::Monitors, "🖥", "Мониторы"),
    (Page::Autostart, "🚀", "Автозапуск"),
    (Page::Rules, "📐", "Правила окон"),
    (Page::Env, "🌿", "Переменные"),
    (Page::Wallpaper, "🖼", "Обои"),
    (Page::Config, "📄", "Готовый конфиг"),
];

pub struct App {
    settings: Settings,
    saved: Settings,
    page: Page,
    status: Option<(String, bool, Instant)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_style(&cc.egui_ctx);
        let settings = hypr::load_settings();
        Self {
            saved: settings.clone(),
            settings,
            page: Page::Themes,
            status: None,
        }
    }

    fn set_status(&mut self, text: impl Into<String>, ok: bool) {
        self.status = Some((text.into(), ok, Instant::now()));
    }

    fn save(&mut self, apply: bool) {
        match hypr::save(&self.settings) {
            Ok(msg) => {
                self.saved = self.settings.clone();
                if apply {
                    match hypr::reload() {
                        Ok(r) => self.set_status(format!("{msg} · {r}"), true),
                        Err(e) => self.set_status(format!("{msg} · {e}"), false),
                    }
                } else {
                    self.set_status(msg, true);
                }
            }
            Err(e) => self.set_status(format!("Ошибка сохранения: {e}"), false),
        }
    }
}

fn setup_style(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = PANEL;
    v.window_fill = PANEL;
    v.extreme_bg_color = BG;
    v.faint_bg_color = CARD;
    v.selection.bg_fill = ACCENT.linear_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0_f32, ACCENT);
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

/// A list of editable rows with delete buttons and an "add" button.
fn edit_list<T>(
    ui: &mut egui::Ui,
    items: &mut Vec<T>,
    new: impl Fn() -> T,
    mut row: impl FnMut(&mut egui::Ui, &mut T),
) {
    let mut remove = None;
    for (i, item) in items.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            row(ui, item);
            if ui.button("🗑").on_hover_text("Удалить").clicked() {
                remove = Some(i);
            }
        });
    }
    if let Some(i) = remove {
        items.remove(i);
    }
    if ui.button("➕ Добавить").clicked() {
        items.push(new());
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dirty = self.settings != self.saved;

        egui::SidePanel::left("nav")
            .resizable(false)
            .exact_width(220.0)
            .frame(egui::Frame::new().fill(BG).inner_margin(Margin::same(14)))
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.label(RichText::new("Easy Hyprland").size(22.0).strong().color(ACCENT));
                hint(ui, "настройка без боли");
                ui.add_space(14.0);
                for &(page, icon, name) in PAGES {
                    let selected = self.page == page;
                    let text = RichText::new(format!("{icon}  {name}")).size(15.5);
                    let text = if selected { text.color(Color32::WHITE).strong() } else { text };
                    let btn = egui::Button::new(text)
                        .fill(if selected { ACCENT.linear_multiply(0.35) } else { Color32::TRANSPARENT })
                        .stroke(Stroke::NONE)
                        .min_size(Vec2::new(ui.available_width(), 36.0));
                    if ui.add(btn).clicked() {
                        self.page = page;
                    }
                }
            });

        egui::TopBottomPanel::bottom("actions")
            .frame(egui::Frame::new().fill(BG).inner_margin(Margin::symmetric(16, 10)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let primary = egui::Button::new(RichText::new("✔ Сохранить и применить").strong().color(BG))
                        .fill(ACCENT);
                    if ui.add(primary).clicked() {
                        self.save(true);
                    }
                    if ui.button("💾 Только сохранить").clicked() {
                        self.save(false);
                    }
                    if ui.add_enabled(dirty, egui::Button::new("↩ Отменить изменения")).clicked() {
                        self.settings = self.saved.clone();
                    }
                    if ui.button("⟲ По умолчанию").clicked() {
                        self.settings = Settings::default();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some((text, ok, at)) = &self.status {
                            if at.elapsed().as_secs() < 8 {
                                let color = if *ok { Color32::from_rgb(0xa6, 0xe3, 0xa1) } else { Color32::from_rgb(0xf3, 0x8b, 0xa8) };
                                ui.label(RichText::new(text).color(color));
                                ctx.request_repaint_after(std::time::Duration::from_secs(1));
                                return;
                            }
                        }
                        if dirty {
                            ui.label(RichText::new("● есть несохранённые изменения").color(Color32::from_rgb(0xf9, 0xe2, 0xaf)));
                        }
                    });
                });
            });

        let show_preview = matches!(self.page, Page::Themes | Page::Windows | Page::Effects);
        if show_preview {
            egui::SidePanel::right("preview")
                .resizable(false)
                .exact_width(380.0)
                .frame(egui::Frame::new().fill(PANEL).inner_margin(Margin::same(16)))
                .show(ctx, |ui| {
                    ui.label(RichText::new("Предпросмотр").size(17.0).strong().color(ACCENT));
                    let size = Vec2::new(ui.available_width(), ui.available_width() * 0.62);
                    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                    draw_preview(ui.painter(), rect, &self.settings);
                    hint(ui, "Так примерно будут выглядеть окна: отступы, рамки, скругления и прозрачность.");
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(PANEL).inner_margin(Margin::same(20)))
            .show(ctx, |ui| {
                let title = PAGES.iter().find(|p| p.0 == self.page).map(|p| p.2).unwrap_or("");
                ui.heading(RichText::new(title).strong());
                ui.add_space(6.0);
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    self.page_ui(ui);
                });
            });
    }
}

impl App {
    fn page_ui(&mut self, ui: &mut egui::Ui) {
        let s = &mut self.settings;
        match self.page {
            Page::Themes => {
                card(ui, "Готовые темы", |ui| {
                    hint(ui, "Нажми на тему, чтобы применить её цвета и стиль. Потом всё можно подкрутить.");
                    ui.horizontal_wrapped(|ui| {
                        for p in presets::PRESETS {
                            if ui.add(egui::Button::new(RichText::new(p.name).size(16.0)).min_size(Vec2::new(140.0, 44.0))).clicked() {
                                (p.apply)(s);
                            }
                        }
                    });
                });
                ui.add_space(8.0);
                card(ui, "Как это работает", |ui| {
                    hint(ui, "1. Выбери настройки в разделах слева.");
                    hint(ui, "2. Нажми «Сохранить и применить» — Hyprland сразу обновится.");
                    hint(ui, "Настройки пишутся в ~/.config/hypr/easy-hyprland.conf, а твой hyprland.conf просто подключает его. Перед изменениями делается резервная копия.");
                });
            }
            Page::Windows => {
                card(ui, "Отступы и рамки", |ui| {
                    ui.add(egui::Slider::new(&mut s.gaps_in, 0..=40).text("Отступ между окнами"));
                    ui.add(egui::Slider::new(&mut s.gaps_out, 0..=80).text("Отступ от краёв экрана"));
                    ui.add(egui::Slider::new(&mut s.border_size, 0..=10).text("Толщина рамки"));
                    ui.add(egui::Slider::new(&mut s.rounding, 0..=30).text("Скругление углов"));
                    ui.checkbox(&mut s.resize_on_border, "Менять размер окна, потянув за рамку");
                });
                ui.add_space(8.0);
                card(ui, "Цвета рамок", |ui| {
                    color_row(ui, "Активное окно", &mut s.active_border_a);
                    ui.checkbox(&mut s.border_gradient, "Градиент");
                    if s.border_gradient {
                        color_row(ui, "Второй цвет градиента", &mut s.active_border_b);
                        ui.add(egui::Slider::new(&mut s.border_angle, 0..=360).text("Угол градиента°"));
                    }
                    color_row(ui, "Неактивное окно", &mut s.inactive_border);
                });
                ui.add_space(8.0);
                card(ui, "Раскладка", |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut s.layout, "dwindle".into(), "Dwindle (спираль)");
                        ui.selectable_value(&mut s.layout, "master".into(), "Master (главное окно)");
                    });
                });
            }
            Page::Effects => {
                card(ui, "Прозрачность", |ui| {
                    ui.add(egui::Slider::new(&mut s.active_opacity, 0.3..=1.0).text("Активное окно"));
                    ui.add(egui::Slider::new(&mut s.inactive_opacity, 0.3..=1.0).text("Неактивные окна"));
                    ui.checkbox(&mut s.dim_inactive, "Затемнять неактивные окна");
                    if s.dim_inactive {
                        ui.add(egui::Slider::new(&mut s.dim_strength, 0.0..=1.0).text("Сила затемнения"));
                    }
                });
                ui.add_space(8.0);
                card(ui, "Размытие (blur)", |ui| {
                    ui.checkbox(&mut s.blur_enabled, "Включить размытие");
                    ui.add_enabled_ui(s.blur_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.blur_size, 1..=20).text("Радиус"));
                        ui.add(egui::Slider::new(&mut s.blur_passes, 1..=6).text("Проходы (качество)"));
                        ui.add(egui::Slider::new(&mut s.blur_vibrancy, 0.0..=1.0).text("Насыщенность"));
                    });
                    hint(ui, "Больше проходов — красивее, но тяжелее для видеокарты.");
                });
                ui.add_space(8.0);
                card(ui, "Тени", |ui| {
                    ui.checkbox(&mut s.shadow_enabled, "Включить тени");
                    ui.add_enabled_ui(s.shadow_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.shadow_range, 1..=40).text("Размер"));
                        ui.add(egui::Slider::new(&mut s.shadow_power, 1..=4).text("Резкость"));
                        color_row(ui, "Цвет тени", &mut s.shadow_color);
                    });
                });
            }
            Page::Animations => {
                card(ui, "Анимации", |ui| {
                    ui.checkbox(&mut s.animations_enabled, "Включить анимации");
                    ui.add_enabled_ui(s.animations_enabled, |ui| {
                        ui.add(egui::Slider::new(&mut s.animation_speed, 0.25..=3.0).text("Скорость (×)"));
                        ui.label("Стиль движения:");
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut s.animation_style, "smooth".into(), "🌊 Плавный");
                            ui.selectable_value(&mut s.animation_style, "snappy".into(), "⚡ Быстрый");
                            ui.selectable_value(&mut s.animation_style, "bouncy".into(), "🏀 Пружинистый");
                        });
                    });
                });
            }
            Page::Input => {
                card(ui, "Клавиатура", |ui| {
                    egui::Grid::new("kb").num_columns(2).spacing([12.0, 10.0]).show(ui, |ui| {
                        ui.label("Раскладки");
                        ui.text_edit_singleline(&mut s.kb_layout);
                        ui.end_row();
                        ui.label("Варианты");
                        ui.text_edit_singleline(&mut s.kb_variant);
                        ui.end_row();
                        ui.label("Опции");
                        ui.text_edit_singleline(&mut s.kb_options);
                        ui.end_row();
                    });
                    hint(ui, "Например: раскладки «us,ru», опция «grp:alt_shift_toggle» — переключение по Alt+Shift.");
                    ui.horizontal(|ui| {
                        ui.label("Переключение раскладки:");
                        for (opt, name) in [("grp:alt_shift_toggle", "Alt+Shift"), ("grp:ctrl_shift_toggle", "Ctrl+Shift"), ("grp:win_space_toggle", "Win+Space"), ("grp:caps_toggle", "CapsLock")] {
                            if ui.selectable_label(s.kb_options == opt, name).clicked() {
                                s.kb_options = opt.into();
                            }
                        }
                    });
                    ui.checkbox(&mut s.numlock_by_default, "NumLock включён при старте");
                });
                ui.add_space(8.0);
                card(ui, "Мышь и тачпад", |ui| {
                    ui.add(egui::Slider::new(&mut s.sensitivity, -1.0..=1.0).text("Чувствительность"));
                    ui.horizontal(|ui| {
                        ui.label("Фокус за мышью:");
                        ui.selectable_value(&mut s.follow_mouse, 0, "Выкл");
                        ui.selectable_value(&mut s.follow_mouse, 1, "Всегда");
                        ui.selectable_value(&mut s.follow_mouse, 2, "Только клавиатура");
                    });
                    ui.checkbox(&mut s.natural_scroll, "Естественная прокрутка на тачпаде");
                    ui.checkbox(&mut s.tap_to_click, "Касание = клик");
                });
            }
            Page::Keys => {
                card(ui, "Программы", |ui| {
                    egui::Grid::new("apps").num_columns(2).spacing([12.0, 10.0]).show(ui, |ui| {
                        ui.label("Главная клавиша ($mainMod)");
                        egui::ComboBox::from_id_salt("mod").selected_text(&s.main_mod).show_ui(ui, |ui| {
                            for m in ["SUPER", "ALT", "CTRL", "SUPER SHIFT"] {
                                ui.selectable_value(&mut s.main_mod, m.into(), m);
                            }
                        });
                        ui.end_row();
                        ui.label("Терминал ($terminal)");
                        ui.text_edit_singleline(&mut s.terminal);
                        ui.end_row();
                        ui.label("Файлы ($fileManager)");
                        ui.text_edit_singleline(&mut s.file_manager);
                        ui.end_row();
                        ui.label("Меню запуска ($menu)");
                        ui.text_edit_singleline(&mut s.launcher);
                        ui.end_row();
                    });
                });
                ui.add_space(8.0);
                card(ui, "Сочетания клавиш", |ui| {
                    hint(ui, "Модификаторы · Клавиша · Действие · Аргумент");
                    edit_list(ui, &mut s.binds, || Bind::new("$mainMod", "", "exec", ""), |ui, b| {
                        ui.add(egui::TextEdit::singleline(&mut b.mods).desired_width(130.0));
                        ui.add(egui::TextEdit::singleline(&mut b.key).desired_width(80.0).hint_text("клавиша"));
                        egui::ComboBox::from_id_salt(b as *const Bind as usize).width(140.0).selected_text(&b.dispatcher).show_ui(ui, |ui| {
                            for d in ["exec", "killactive", "exit", "togglefloating", "fullscreen", "pseudo", "togglesplit", "movefocus", "movewindow", "workspace", "movetoworkspace", "togglespecialworkspace"] {
                                ui.selectable_value(&mut b.dispatcher, d.into(), d);
                            }
                        });
                        ui.add(egui::TextEdit::singleline(&mut b.arg).desired_width(ui.available_width() - 50.0));
                    });
                    ui.separator();
                    ui.checkbox(&mut s.workspace_binds, "Добавить клавиши рабочих столов ($mainMod + 1…0, перенос с Shift, мышь)");
                });
            }
            Page::Monitors => {
                card(ui, "Мониторы", |ui| {
                    hint(ui, "Имя пустое = любой монитор. Имена можно узнать командой «hyprctl monitors».");
                    hint(ui, "Разрешение: preferred / highres / 1920x1080@144 · Позиция: auto / 0x0 · Масштаб: 1, 1.25, 2");
                    edit_list(ui, &mut s.monitors, || Monitor { name: "HDMI-A-1".into(), resolution: "preferred".into(), position: "auto".into(), scale: "1".into() }, |ui, m| {
                        ui.add(egui::TextEdit::singleline(&mut m.name).desired_width(120.0).hint_text("имя"));
                        ui.add(egui::TextEdit::singleline(&mut m.resolution).desired_width(150.0));
                        ui.add(egui::TextEdit::singleline(&mut m.position).desired_width(90.0));
                        ui.add(egui::TextEdit::singleline(&mut m.scale).desired_width(50.0));
                    });
                });
            }
            Page::Autostart => {
                card(ui, "Запускать вместе с Hyprland", |ui| {
                    hint(ui, "Каждая строка — команда (exec-once). Например: waybar, nm-applet, dunst.");
                    edit_list(ui, &mut s.autostart, String::new, |ui, cmd| {
                        ui.add(egui::TextEdit::singleline(cmd).desired_width(ui.available_width() - 50.0));
                    });
                });
            }
            Page::Rules => {
                card(ui, "Правила окон", |ui| {
                    hint(ui, "Правило · Для каких окон. Например: «float» для «class:^(pavucontrol)$», «opacity 0.9» для «class:^(kitty)$».");
                    edit_list(ui, &mut s.window_rules, || WindowRule { rule: "float".into(), matcher: "class:^()$".into() }, |ui, r| {
                        ui.add(egui::TextEdit::singleline(&mut r.rule).desired_width(160.0));
                        ui.add(egui::TextEdit::singleline(&mut r.matcher).desired_width(ui.available_width() - 50.0));
                    });
                });
            }
            Page::Env => {
                card(ui, "Переменные окружения", |ui| {
                    hint(ui, "Например: XCURSOR_SIZE = 24, QT_QPA_PLATFORMTHEME = qt6ct.");
                    edit_list(ui, &mut s.env, || EnvVar { name: String::new(), value: String::new() }, |ui, e| {
                        ui.add(egui::TextEdit::singleline(&mut e.name).desired_width(220.0).hint_text("ИМЯ"));
                        ui.label("=");
                        ui.add(egui::TextEdit::singleline(&mut e.value).desired_width(ui.available_width() - 50.0));
                    });
                });
            }
            Page::Wallpaper => {
                card(ui, "Обои (hyprpaper)", |ui| {
                    ui.checkbox(&mut s.wallpaper_enabled, "Управлять обоями через hyprpaper");
                    ui.add_enabled_ui(s.wallpaper_enabled, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Путь к картинке");
                            ui.add(egui::TextEdit::singleline(&mut s.wallpaper).desired_width(ui.available_width()).hint_text("~/Pictures/wallpaper.png"));
                        });
                    });
                    hint(ui, "Нужен установленный пакет hyprpaper. Будет создан ~/.config/hypr/hyprpaper.conf.");
                });
            }
            Page::Config => {
                let text = s.generate();
                card(ui, "Свои строки", |ui| {
                    hint(ui, "Всё, что не нашлось в программе, — сюда. Добавится в конец конфига как есть.");
                    ui.add(egui::TextEdit::multiline(&mut s.extra).code_editor().desired_rows(5).desired_width(f32::INFINITY));
                });
                ui.add_space(8.0);
                card(ui, "easy-hyprland.conf", |ui| {
                    if ui.button("📋 Скопировать").clicked() {
                        ui.ctx().copy_text(text.clone());
                    }
                    let mut shown = text.as_str();
                    ui.add(egui::TextEdit::multiline(&mut shown).code_editor().desired_width(f32::INFINITY));
                });
            }
        }
    }
}

/// Draws a small fake desktop with three tiled windows using the current settings.
fn draw_preview(p: &egui::Painter, rect: Rect, s: &Settings) {
    let scale = 0.5;
    // Wallpaper: vertical gradient.
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

    // Bar on top.
    let bar = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 14.0));
    p.rect_filled(bar, CornerRadius::ZERO, Color32::from_black_alpha(140));

    let area = Rect::from_min_max(Pos2::new(rect.min.x, bar.max.y), rect.max)
        .shrink(s.gaps_out as f32 * scale);
    let gi = s.gaps_in as f32 * scale;
    let left = Rect::from_min_max(area.min, Pos2::new(area.center().x - gi, area.max.y));
    let rt = Rect::from_min_max(Pos2::new(area.center().x + gi, area.min.y), Pos2::new(area.max.x, area.center().y - gi));
    let rb = Rect::from_min_max(Pos2::new(area.center().x + gi, area.center().y + gi), area.max);

    let radius = CornerRadius::same((s.rounding as f32 * scale).min(40.0) as u8);
    for (i, w) in [left, rt, rb].into_iter().enumerate() {
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
        let fill = Color32::from_rgba_unmultiplied(0x1e, 0x1e, 0x2e, (opacity * 255.0) as u8);
        p.rect_filled(w, radius, fill);

        // Fake title text lines.
        let line = Color32::from_white_alpha((60.0 * opacity) as u8);
        for n in 0..3 {
            let y = w.min.y + 12.0 + n as f32 * 9.0;
            if y + 4.0 < w.max.y {
                let len = (w.width() - 24.0) * [0.7, 0.5, 0.6][n];
                p.rect_filled(Rect::from_min_size(Pos2::new(w.min.x + 12.0, y), Vec2::new(len.max(0.0), 4.0)), CornerRadius::same(2), line);
            }
        }

        if s.border_size > 0 {
            let width = (s.border_size as f32 * scale).max(1.0);
            let color = if active {
                if s.border_gradient {
                    c32(s.active_border_a).lerp_to_gamma(c32(s.active_border_b), 0.5)
                } else {
                    c32(s.active_border_a)
                }
            } else {
                c32(s.inactive_border)
            };
            p.rect_stroke(w, radius, Stroke::new(width, color), StrokeKind::Inside);
            if active && s.border_gradient {
                // Hint at the gradient with corner accents.
                p.rect_stroke(w.shrink(width), radius, Stroke::new(width * 0.6, c32(s.active_border_b).linear_multiply(0.5)), StrokeKind::Inside);
            }
        }
    }
}
