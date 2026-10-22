[English](README.md) · [Русский](README.ru.md) · **Deutsch**

# Easy Hyprland

Eine einfache, schöne grafische Oberfläche zum Konfigurieren von [Hyprland](https://hyprland.org).
Du musst die Konfig-Syntax nicht lernen: Regler verschieben, Farben wählen und **Speichern und anwenden** drücken.

Geschrieben in **Rust** mit [egui](https://github.com/emilk/egui). Die Oberfläche gibt es auf Deutsch, Englisch und Russisch.

![Screenshot](docs/screenshot-de.png)

## Funktionen

- 🎨 **Themes**: Catppuccin, Nord, Gruvbox, Neon und Minimal, jeweils mit einem Klick
- 🪟 **Fenster**: Abstände, Rahmen, Verlaufsrahmen, Eckenrundung, Layout (dwindle, master, scrolling, monocle)
- ✨ **Effekte**: Transparenz, Unschärfe (Blur), Schatten, Abdunkeln inaktiver Fenster
- 🎞 **Animationen**: an/aus, Geschwindigkeit, Stil (weich, zackig, federnd)
- ⌨ **Eingabe**: Tastaturlayouts und Layoutwechsel, Mausempfindlichkeit, Touchpad, Drei-Finger-Wischen zwischen Arbeitsflächen
- 🔑 **Tastenkürzel**: Standardprogramme und ein Kürzel-Editor
- 🖥 **Monitore**: Auflösung, Bildwiederholrate, Position, Skalierung
- 🚀 **Autostart** von Programmen
- 📐 **Fensterregeln**
- 🌿 **Umgebungsvariablen**
- 🖼 **Hintergrund** mit hyprpaper
- 📄 **Fertige Konfig**: ansehen, kopieren, eigene Zeilen hinzufügen
- 👀 **Live-Vorschau** deiner Fenster direkt in der App
- 🛟 **Sicher**: vor Änderungen an deiner Konfig wird eine Sicherung angelegt, und nach dem Anwenden werden Konfigfehler von Hyprland angezeigt

## Unterstützte Hyprland-Versionen

| Hyprland | Konfig-Format | Unterstützung |
|---|---|---|
| **0.55 und neuer** | Lua (`hyprland.lua`) | ✅ |
| **0.53 – 0.54** | hyprlang (`hyprland.conf`) | ✅ |
| älter als 0.53 | alte hyprlang-Syntax | ❌ |

Das Format wird automatisch erkannt. Seit 0.55 lädt Hyprland `hyprland.lua` und liest die alte `hyprland.conf` nur, wenn keine Lua-Datei existiert. Die App schreibt deshalb in die Datei, die dein Hyprland tatsächlich lädt. Auf der Seite **Themes** kannst du das Format auch selbst wählen.

## Installation

### 1. Rust installieren

```bash
# Arch Linux
sudo pacman -S rust
# jede Distribution
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Bauen und installieren

```bash
git clone https://github.com/NeXtagG/easy-hyprland.git
cd easy-hyprland
cargo install --path .
```

Das Programm landet in `~/.cargo/bin/easy-hyprland`. Achte darauf, dass `~/.cargo/bin` in deinem `PATH` ist.

Ohne Installation bauen:

```bash
cargo build --release
./target/release/easy-hyprland
```

### 3. (Optional) Zum App-Starter hinzufügen

```bash
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/easy-hyprland.desktop <<EOF
[Desktop Entry]
Name=Easy Hyprland
Comment=Hyprland konfigurieren
Exec=$HOME/.cargo/bin/easy-hyprland
Icon=preferences-desktop
Type=Application
Categories=Settings;
EOF
```

## Benutzung

1. Starte `easy-hyprland` im Terminal oder über deinen Starter (wofi, rofi, hyprlauncher…).
2. Wähle links einen Bereich, zum Beispiel **Themes** oder **Fenster**.
3. Ändere die Einstellungen. Die Vorschau rechts wird sofort aktualisiert.
4. Drücke **Speichern und anwenden** (oder **Strg+S**). Hyprland lädt sofort neu, ohne Neuanmeldung.

Die Sprache wird aus der Systemsprache übernommen. Du kannst sie unten in der Seitenleiste ändern.

| Schaltfläche | Was sie tut |
|---|---|
| ✔ Speichern und anwenden | schreibt die Konfig, führt `hyprctl reload` aus und zeigt eventuelle Konfigfehler |
| 💾 Nur speichern | schreibt nur die Dateien |
| ↩ Änderungen verwerfen | kehrt zu den zuletzt gespeicherten Einstellungen zurück |
| ⟲ Standardwerte | setzt alles auf die Standardwerte zurück |

Wenn du das Fenster mit ungespeicherten Änderungen schließt, fragt die App, ob sie gespeichert werden sollen.

## So funktioniert es

- Deine Einstellungen werden in eine eigene Datei geschrieben:
  - Lua: `~/.config/hypr/easy-hyprland.lua`
  - hyprlang: `~/.config/hypr/easy-hyprland.conf`
- Ans Ende deiner Hauptkonfig (`hyprland.lua` oder `hyprland.conf`) wird eine Zeile angehängt:
  ```lua
  require("easy-hyprland")                         -- Lua
  ```
  ```ini
  source = /home/du/.config/hypr/easy-hyprland.conf   # hyprlang
  ```
  Vorher wird eine Kopie `hyprland.lua.bak-<Zeit>` (oder `hyprland.conf.bak-<Zeit>`) angelegt. Deine bisherige Konfig bleibt erhalten, die Einstellungen der App werden darüber angewendet.
- Gibt es noch keine Hauptkonfig, wird sie erstellt.
- Der Zustand der App liegt in `~/.config/easy-hyprland/settings.json`.
- Mit aktiviertem Hintergrund wird `~/.config/hypr/hyprpaper.conf` geschrieben. Eine fremde hyprpaper-Konfig wird vorher gesichert.
- `$XDG_CONFIG_HOME` wird berücksichtigt.

### Deinstallieren

Entferne die Zeile `require("easy-hyprland")` bzw. `source = …easy-hyprland.conf` aus deiner Hauptkonfig und führe aus:

```bash
cargo uninstall easy-hyprland
rm -r ~/.config/easy-hyprland ~/.config/hypr/easy-hyprland.*
```

## Entwicklung

```bash
cargo run              # App starten
cargo test             # Unit-Tests
tests/validate.sh      # erzeugte Konfigs gegen Hyprland 0.56.2 prüfen (benötigt lua5.4 und python3)
```

`tests/validate.sh` erzeugt Beispielkonfigs in beiden Formaten und prüft jede Option, jeden Dispatcher und jeden Fensterregel-Effekt gegen Listen aus dem Quellcode von Hyprland 0.56.2 (`tests/hyprland-0.56.2/`). Die Lua-Konfigs werden gegen einen strikten Mock der `hl`-API ausgeführt.

Pull Requests sind willkommen!

## Lizenz

Dieses Projekt steht unter der **GNU GPL v3.0 oder neuer**. Der vollständige Text steht in [LICENSE](LICENSE).

Einfach gesagt:
- ✅ jeder darf das Programm **kostenlos nutzen**, auch beruflich;
- ✅ du darfst den Code **ändern** und **weitergeben**;
- ❗ wenn du eine geänderte Version weitergibst, muss ihr Quellcode ebenfalls unter der GPL-3.0 **offen** sein.
