use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Ru,
    En,
    De,
}

impl Lang {
    pub const ALL: [Lang; 3] = [Lang::En, Lang::Ru, Lang::De];

    pub fn code(self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en",
            Lang::De => "de",
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Lang::Ru => "Русский",
            Lang::En => "English",
            Lang::De => "Deutsch",
        }
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        let code = code.trim().to_lowercase();
        Lang::ALL.into_iter().find(|l| code.starts_with(l.code()))
    }

    /// Picks the language from the usual locale environment variables, English otherwise.
    pub fn detect() -> Lang {
        ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"]
            .iter()
            .filter_map(|v| std::env::var(v).ok())
            .filter(|v| !v.is_empty() && v != "C" && v != "POSIX")
            .find_map(|v| Lang::from_code(&v))
            .unwrap_or(Lang::En)
    }

    pub fn t(self, key: &str) -> &'static str {
        static MAP: OnceLock<HashMap<&'static str, [&'static str; 3]>> = OnceLock::new();
        let map = MAP.get_or_init(|| STRINGS.iter().map(|(k, ru, en, de)| (*k, [*ru, *en, *de])).collect());
        match map.get(key) {
            Some(v) => v[self as usize],
            None => {
                debug_assert!(false, "missing translation key {key}");
                "???"
            }
        }
    }
}

/// (key, ru, en, de)
pub const STRINGS: &[(&str, &str, &str, &str)] = &[
    // navigation
    ("page.themes", "Темы", "Themes", "Themes"),
    ("page.windows", "Окна", "Windows", "Fenster"),
    ("page.effects", "Эффекты", "Effects", "Effekte"),
    ("page.animations", "Анимации", "Animations", "Animationen"),
    ("page.input", "Ввод", "Input", "Eingabe"),
    ("page.keys", "Горячие клавиши", "Keybinds", "Tastenkürzel"),
    ("page.monitors", "Мониторы", "Monitors", "Monitore"),
    ("page.autostart", "Автозапуск", "Autostart", "Autostart"),
    ("page.rules", "Правила окон", "Window rules", "Fensterregeln"),
    ("page.env", "Переменные", "Environment", "Umgebung"),
    ("page.wallpaper", "Обои", "Wallpaper", "Hintergrund"),
    ("page.config", "Готовый конфиг", "Generated config", "Fertige Konfig"),
    ("app.tagline", "настройка без боли", "painless configuration", "Konfiguration ohne Schmerz"),
    ("app.language", "Язык", "Language", "Sprache"),
    // bottom bar
    ("btn.save_apply", "✔ Сохранить и применить", "✔ Save and apply", "✔ Speichern und anwenden"),
    ("btn.save", "💾 Только сохранить", "💾 Save only", "💾 Nur speichern"),
    ("btn.revert", "↩ Отменить изменения", "↩ Revert changes", "↩ Änderungen verwerfen"),
    ("btn.defaults", "⟲ По умолчанию", "⟲ Defaults", "⟲ Standardwerte"),
    ("btn.add", "➕ Добавить", "➕ Add", "➕ Hinzufügen"),
    ("btn.delete", "Удалить", "Delete", "Löschen"),
    ("btn.copy", "📋 Скопировать", "📋 Copy", "📋 Kopieren"),
    ("btn.copied", "Скопировано", "Copied", "Kopiert"),
    ("status.unsaved", "• есть несохранённые изменения", "• unsaved changes", "• ungespeicherte Änderungen"),
    ("status.saved", "Сохранено: {}", "Saved: {}", "Gespeichert: {}"),
    ("status.backup", "копия старого файла: {}", "backup of the old file: {}", "Sicherung der alten Datei: {}"),
    ("status.save_error", "Ошибка сохранения: {}", "Save failed: {}", "Speichern fehlgeschlagen: {}"),
    ("status.load_error", "Не удалось прочитать настройки ({}), загружены значения по умолчанию", "Could not read settings ({}), defaults loaded", "Einstellungen konnten nicht gelesen werden ({}), Standardwerte geladen"),
    ("status.manual_include", "Основной конфиг {} доступен только для чтения (его создаёт Nix/home-manager). Добавь эту строку в него через свою конфигурацию Nix и пересобери систему:", "Your main config {} is read-only (it is managed by Nix/home-manager). Add this line to it through your Nix configuration and rebuild:", "Deine Hauptkonfig {} ist schreibgeschützt (sie wird von Nix/home-manager verwaltet). Füge diese Zeile über deine Nix-Konfiguration hinzu und baue neu:"),
    ("status.reloaded", "Hyprland перезагружен", "Hyprland reloaded", "Hyprland neu geladen"),
    ("status.no_hyprctl", "hyprctl не найден: Hyprland запущен?", "hyprctl not found: is Hyprland running?", "hyprctl nicht gefunden: läuft Hyprland?"),
    ("status.config_errors", "Hyprland сообщает об ошибках в конфиге: {}", "Hyprland reports config errors: {}", "Hyprland meldet Konfigurationsfehler: {}"),
    // close dialog
    ("close.title", "Несохранённые изменения", "Unsaved changes", "Ungespeicherte Änderungen"),
    ("close.text", "Сохранить изменения перед выходом?", "Save your changes before quitting?", "Änderungen vor dem Beenden speichern?"),
    ("close.save", "Сохранить и выйти", "Save and quit", "Speichern und beenden"),
    ("close.discard", "Выйти без сохранения", "Quit without saving", "Ohne Speichern beenden"),
    ("close.cancel", "Отмена", "Cancel", "Abbrechen"),
    // preview
    ("preview.title", "Предпросмотр", "Preview", "Vorschau"),
    ("preview.hint", "Так примерно будут выглядеть окна: отступы, рамки, скругления и прозрачность.", "Roughly how your windows will look: gaps, borders, rounding and opacity.", "So ungefähr sehen deine Fenster aus: Abstände, Rahmen, Rundungen und Transparenz."),
    // themes
    ("themes.card", "Готовые темы", "Ready-made themes", "Fertige Themes"),
    ("themes.hint", "Нажми на тему, чтобы применить её цвета и стиль. Потом всё можно подкрутить.", "Click a theme to apply its colors and style. You can fine-tune everything afterwards.", "Klicke auf ein Theme, um Farben und Stil zu übernehmen. Danach kannst du alles anpassen."),
    ("themes.minimal", "Минимализм", "Minimal", "Minimal"),
    ("themes.neon", "Неон", "Neon", "Neon"),
    ("howto.card", "Как это работает", "How it works", "So funktioniert es"),
    ("howto.1", "1. Выбери настройки в разделах слева.", "1. Pick your settings in the sections on the left.", "1. Wähle deine Einstellungen in den Bereichen links."),
    ("howto.2", "2. Нажми «Сохранить и применить» (или Ctrl+S): Hyprland сразу обновится.", "2. Press “Save and apply” (or Ctrl+S): Hyprland updates right away.", "2. Drücke „Speichern und anwenden“ (oder Strg+S): Hyprland wird sofort aktualisiert."),
    ("howto.3", "Настройки пишутся в отдельный файл, а твой основной конфиг только подключает его. Перед изменением основного конфига делается резервная копия.", "Settings go into a separate file that your main config simply includes. A backup is made before your main config is changed.", "Die Einstellungen landen in einer eigenen Datei, die deine Hauptkonfig nur einbindet. Vor Änderungen an der Hauptkonfig wird eine Sicherung angelegt."),
    ("format.card", "Формат конфига", "Config format", "Konfig-Format"),
    ("format.auto", "Автоматически", "Automatic", "Automatisch"),
    ("format.lua", "Lua (Hyprland 0.55+)", "Lua (Hyprland 0.55+)", "Lua (Hyprland 0.55+)"),
    ("format.conf", "hyprlang .conf (0.53–0.54)", "hyprlang .conf (0.53–0.54)", "hyprlang .conf (0.53–0.54)"),
    ("format.hint", "С версии 0.55 Hyprland читает hyprland.lua, а если его нет, то старый hyprland.conf. «Автоматически» выбирает тот файл, который реально загружает твой Hyprland.", "Since 0.55 Hyprland reads hyprland.lua, or the old hyprland.conf if there is no Lua file. “Automatic” picks whichever file your Hyprland actually loads.", "Seit 0.55 liest Hyprland hyprland.lua, oder die alte hyprland.conf, wenn keine Lua-Datei existiert. „Automatisch“ wählt die Datei, die dein Hyprland wirklich lädt."),
    ("format.detected", "Сейчас будет использован: {}", "Will be written as: {}", "Wird geschrieben als: {}"),
    // windows
    ("win.gaps_card", "Отступы и рамки", "Gaps and borders", "Abstände und Rahmen"),
    ("win.gaps_in", "Отступ между окнами", "Gap between windows", "Abstand zwischen Fenstern"),
    ("win.gaps_out", "Отступ от краёв экрана", "Gap to screen edges", "Abstand zum Bildschirmrand"),
    ("win.border_size", "Толщина рамки", "Border width", "Rahmenbreite"),
    ("win.rounding", "Скругление углов", "Corner rounding", "Eckenrundung"),
    ("win.resize_on_border", "Менять размер окна, потянув за рамку", "Resize windows by dragging their border", "Fenstergröße durch Ziehen am Rahmen ändern"),
    ("win.colors_card", "Цвета рамок", "Border colors", "Rahmenfarben"),
    ("win.active", "Активное окно", "Active window", "Aktives Fenster"),
    ("win.gradient", "Градиент", "Gradient", "Verlauf"),
    ("win.second_color", "Второй цвет градиента", "Second gradient color", "Zweite Verlaufsfarbe"),
    ("win.angle", "Угол градиента°", "Gradient angle°", "Verlaufswinkel°"),
    ("win.inactive", "Неактивное окно", "Inactive window", "Inaktives Fenster"),
    ("win.layout_card", "Раскладка", "Layout", "Layout"),
    ("win.dwindle", "Dwindle (спираль)", "Dwindle (spiral)", "Dwindle (Spirale)"),
    ("win.master", "Master (главное окно)", "Master (main window)", "Master (Hauptfenster)"),
    ("win.scrolling", "Scrolling (лента)", "Scrolling (strip)", "Scrolling (Streifen)"),
    ("win.monocle", "Monocle (одно окно)", "Monocle (one window)", "Monocle (ein Fenster)"),
    ("win.layout_hint", "Scrolling и Monocle есть только в новых версиях Hyprland (0.54+).", "Scrolling and Monocle only exist in newer Hyprland versions (0.54+).", "Scrolling und Monocle gibt es nur in neueren Hyprland-Versionen (0.54+)."),
    // effects
    ("fx.opacity_card", "Прозрачность", "Opacity", "Transparenz"),
    ("fx.active", "Активное окно", "Active window", "Aktives Fenster"),
    ("fx.inactive", "Неактивные окна", "Inactive windows", "Inaktive Fenster"),
    ("fx.dim", "Затемнять неактивные окна", "Dim inactive windows", "Inaktive Fenster abdunkeln"),
    ("fx.dim_strength", "Сила затемнения", "Dim strength", "Abdunkelungsstärke"),
    ("fx.blur_card", "Размытие (blur)", "Blur", "Unschärfe (Blur)"),
    ("fx.blur_enable", "Включить размытие", "Enable blur", "Unschärfe aktivieren"),
    ("fx.blur_size", "Радиус", "Radius", "Radius"),
    ("fx.blur_passes", "Проходы (качество)", "Passes (quality)", "Durchgänge (Qualität)"),
    ("fx.blur_vibrancy", "Насыщенность", "Vibrancy", "Sättigung"),
    ("fx.blur_hint", "Больше проходов — красивее, но тяжелее для видеокарты.", "More passes look nicer but cost more GPU time.", "Mehr Durchgänge sehen schöner aus, belasten aber die Grafikkarte."),
    ("fx.shadow_card", "Тени", "Shadows", "Schatten"),
    ("fx.shadow_enable", "Включить тени", "Enable shadows", "Schatten aktivieren"),
    ("fx.shadow_range", "Размер", "Size", "Größe"),
    ("fx.shadow_power", "Резкость", "Falloff", "Abfall"),
    ("fx.shadow_color", "Цвет тени", "Shadow color", "Schattenfarbe"),
    // animations
    ("anim.card", "Анимации", "Animations", "Animationen"),
    ("anim.enable", "Включить анимации", "Enable animations", "Animationen aktivieren"),
    ("anim.speed", "Скорость (×)", "Speed (×)", "Geschwindigkeit (×)"),
    ("anim.style", "Стиль движения:", "Motion style:", "Bewegungsstil:"),
    ("anim.smooth", "🌊 Плавный", "🌊 Smooth", "🌊 Weich"),
    ("anim.snappy", "⚡ Быстрый", "⚡ Snappy", "⚡ Zackig"),
    ("anim.bouncy", "🏀 Пружинистый", "🏀 Bouncy", "🏀 Federnd"),
    // input
    ("input.kb_card", "Клавиатура", "Keyboard", "Tastatur"),
    ("input.layouts", "Раскладки", "Layouts", "Layouts"),
    ("input.variants", "Варианты", "Variants", "Varianten"),
    ("input.options", "Опции", "Options", "Optionen"),
    ("input.kb_hint", "Например: раскладки «us,ru», опция «grp:alt_shift_toggle» — переключение по Alt+Shift.", "For example: layouts “us,de”, option “grp:alt_shift_toggle” switches with Alt+Shift.", "Zum Beispiel: Layouts „de,us“, Option „grp:alt_shift_toggle“ wechselt mit Alt+Umschalt."),
    ("input.switch", "Переключение раскладки:", "Layout switching:", "Layout wechseln:"),
    ("input.numlock", "NumLock включён при старте", "NumLock on at startup", "NumLock beim Start an"),
    ("input.mouse_card", "Мышь и тачпад", "Mouse and touchpad", "Maus und Touchpad"),
    ("input.sensitivity", "Чувствительность", "Sensitivity", "Empfindlichkeit"),
    ("input.follow", "Фокус за мышью:", "Focus follows mouse:", "Fokus folgt der Maus:"),
    ("input.follow_off", "Выкл", "Off", "Aus"),
    ("input.follow_always", "Всегда", "Always", "Immer"),
    ("input.follow_loose", "Только клавиатура", "Keyboard only", "Nur Tastatur"),
    ("input.natural", "Естественная прокрутка на тачпаде", "Natural touchpad scrolling", "Natürliches Scrollen am Touchpad"),
    ("input.tap", "Касание = клик", "Tap to click", "Tippen zum Klicken"),
    ("input.gesture", "Смена рабочих столов свайпом тремя пальцами", "Switch workspaces with a three-finger swipe", "Arbeitsflächen mit Drei-Finger-Wischen wechseln"),
    // keys
    ("keys.apps_card", "Программы", "Programs", "Programme"),
    ("keys.main_mod", "Главная клавиша ($mainMod)", "Main modifier ($mainMod)", "Haupttaste ($mainMod)"),
    ("keys.terminal", "Терминал ($terminal)", "Terminal ($terminal)", "Terminal ($terminal)"),
    ("keys.files", "Файлы ($fileManager)", "Files ($fileManager)", "Dateien ($fileManager)"),
    ("keys.menu", "Меню запуска ($menu)", "Launcher ($menu)", "Starter ($menu)"),
    ("keys.binds_card", "Сочетания клавиш", "Shortcuts", "Tastenkürzel"),
    ("keys.binds_hint", "Модификаторы · Клавиша · Действие · Аргумент", "Modifiers · Key · Action · Argument", "Modifikatoren · Taste · Aktion · Argument"),
    ("keys.key", "клавиша", "key", "Taste"),
    ("keys.workspaces", "Добавить клавиши рабочих столов ($mainMod + 1…0, перенос с Shift, мышь)", "Add workspace keys ($mainMod + 1…0, move with Shift, mouse)", "Arbeitsflächen-Tasten hinzufügen ($mainMod + 1…0, verschieben mit Umschalt, Maus)"),
    // monitors
    ("mon.card", "Мониторы", "Monitors", "Monitore"),
    ("mon.hint1", "Имя пустое = любой монитор. Имена можно узнать командой «hyprctl monitors».", "Empty name = any monitor. Find names with “hyprctl monitors”.", "Leerer Name = jeder Monitor. Namen zeigt „hyprctl monitors“."),
    ("mon.hint2", "Режим: preferred / highres / 1920x1080@144 · Позиция: auto / 0x0 · Масштаб: 1, 1.25, 2, auto", "Mode: preferred / highres / 1920x1080@144 · Position: auto / 0x0 · Scale: 1, 1.25, 2, auto", "Modus: preferred / highres / 1920x1080@144 · Position: auto / 0x0 · Skalierung: 1, 1.25, 2, auto"),
    ("mon.name", "имя", "name", "Name"),
    // autostart
    ("auto.card", "Запускать вместе с Hyprland", "Start together with Hyprland", "Zusammen mit Hyprland starten"),
    ("auto.hint", "Каждая строка — команда. Например: waybar, nm-applet, dunst.", "One command per line. For example: waybar, nm-applet, dunst.", "Ein Befehl pro Zeile. Zum Beispiel: waybar, nm-applet, dunst."),
    // rules
    ("rules.card", "Правила окон", "Window rules", "Fensterregeln"),
    ("rules.hint", "Для каких окон (поле и регулярное выражение) · Эффект · Значение. Например: class «^(pavucontrol)$» -> float.", "Which windows (field and regex) · Effect · Value. For example: class “^(pavucontrol)$” -> float.", "Welche Fenster (Feld und Regex) · Effekt · Wert. Zum Beispiel: class „^(pavucontrol)$“ -> float."),
    ("rules.value_bool", "вкл", "on", "an"),
    ("rules.value", "значение", "value", "Wert"),
    // env
    ("env.card", "Переменные окружения", "Environment variables", "Umgebungsvariablen"),
    ("env.hint", "Например: XCURSOR_SIZE = 24, QT_QPA_PLATFORMTHEME = qt6ct.", "For example: XCURSOR_SIZE = 24, QT_QPA_PLATFORMTHEME = qt6ct.", "Zum Beispiel: XCURSOR_SIZE = 24, QT_QPA_PLATFORMTHEME = qt6ct."),
    ("env.name", "ИМЯ", "NAME", "NAME"),
    // wallpaper
    ("wp.card", "Обои (hyprpaper)", "Wallpaper (hyprpaper)", "Hintergrund (hyprpaper)"),
    ("wp.enable", "Управлять обоями через hyprpaper", "Manage the wallpaper with hyprpaper", "Hintergrund mit hyprpaper verwalten"),
    ("wp.path", "Картинка или папка", "Image or folder", "Bild oder Ordner"),
    ("wp.fit", "Масштабирование:", "Fit mode:", "Anpassung:"),
    ("wp.hint", "Нужен пакет hyprpaper. Будет создан ~/.config/hypr/hyprpaper.conf, а при применении обои сменятся сразу.", "Requires hyprpaper. ~/.config/hypr/hyprpaper.conf will be written and the wallpaper changes immediately on apply.", "Benötigt hyprpaper. ~/.config/hypr/hyprpaper.conf wird geschrieben und der Hintergrund beim Anwenden sofort gewechselt."),
    // config page
    ("cfg.extra_card", "Свои строки", "Custom lines", "Eigene Zeilen"),
    ("cfg.extra_hint", "Всё, что не нашлось в программе. Добавится в конец как есть, поэтому пиши в формате своего конфига ({}).", "Anything the app doesn't cover. Appended as-is, so write it in your config's format ({}).", "Alles, was die App nicht abdeckt. Wird unverändert angehängt, also im Format deiner Konfig schreiben ({})."),
    // generated file header
    ("gen.header1", "Сгенерировано Easy Hyprland", "Generated by Easy Hyprland", "Erzeugt von Easy Hyprland"),
    ("gen.header2", "Не редактируйте вручную: файл перезаписывается при каждом сохранении.", "Do not edit by hand: this file is overwritten on every save.", "Nicht von Hand bearbeiten: die Datei wird bei jedem Speichern überschrieben."),
    ("gen.extra", "Свои строки", "Custom lines", "Eigene Zeilen"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_unique_and_filled() {
        let mut seen = std::collections::HashSet::new();
        for (k, ru, en, de) in STRINGS {
            assert!(seen.insert(*k), "duplicate key {k}");
            assert!(!ru.is_empty() && !en.is_empty() && !de.is_empty(), "empty translation for {k}");
            let ph = |s: &str| s.matches("{}").count();
            assert!(ph(ru) == ph(en) && ph(en) == ph(de), "placeholder mismatch in {k}");
        }
    }

    #[test]
    fn every_used_key_exists() {
        let keys: std::collections::HashSet<_> = STRINGS.iter().map(|s| s.0).collect();
        for src in [include_str!("app.rs"), include_str!("config.rs")] {
            for part in src.split(".t(\"").skip(1) {
                let key = &part[..part.find('"').unwrap()];
                assert!(keys.contains(key), "missing translation key {key}");
            }
        }
    }

    #[test]
    fn detects_language_codes() {
        assert_eq!(Lang::from_code("ru_RU.UTF-8"), Some(Lang::Ru));
        assert_eq!(Lang::from_code("de_DE.UTF-8"), Some(Lang::De));
        assert_eq!(Lang::from_code("en_US"), Some(Lang::En));
        assert_eq!(Lang::from_code("fr_FR"), None);
    }
}
