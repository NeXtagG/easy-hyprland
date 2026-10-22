**English** · [Русский](README.ru.md) · [Deutsch](README.de.md)

# Easy Hyprland

An easy, good-looking GUI for configuring [Hyprland](https://hyprland.org).
You don't need to learn the config syntax: move the sliders, pick colors and press **Save and apply**.

Written in **Rust** with [egui](https://github.com/emilk/egui). The interface is available in English, Russian and German.

![Easy Hyprland demo](docs/demo.gif)

## Features

- 🎨 **Themes**: Catppuccin, Nord, Gruvbox, Neon and Minimal, one click each
- 🪟 **Windows**: gaps, borders, gradient borders, rounding, layout (dwindle, master, scrolling, monocle)
- ✨ **Effects**: opacity, blur, shadows, dimming of inactive windows
- 🎞 **Animations**: on/off, speed, style (smooth, snappy, bouncy)
- ⌨ **Input**: keyboard layouts and layout switching, mouse sensitivity, touchpad, three-finger workspace swipe
- 🔑 **Keybinds**: default programs and a shortcut editor
- 🖥 **Monitors**: resolution, refresh rate, position, scale
- 🚀 **Autostart** programs
- 📐 **Window rules**
- 🌿 **Environment variables**
- 🖼 **Wallpaper** with hyprpaper
- 📄 **Generated config**: view, copy, add your own lines
- 👀 **Live preview** of your windows inside the app
- 🛟 **Safe**: a backup is made before your config is touched, and Hyprland's config errors are shown after applying

## Screenshots

The five built-in themes, as shown in the live preview:

![Themes](docs/themes.png)

Every section of the app:

<table>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/themes.png" alt="Themes and config format"><br><sub>Themes and config format</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/windows.png" alt="Windows: gaps, borders, layout"><br><sub>Windows: gaps, borders, layout</sub></td>
  </tr>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/effects.png" alt="Effects: opacity, blur, shadows"><br><sub>Effects: opacity, blur, shadows</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/animations.png" alt="Animations"><br><sub>Animations</sub></td>
  </tr>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/input.png" alt="Input: keyboard, mouse, touchpad"><br><sub>Input: keyboard, mouse, touchpad</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/keys.png" alt="Keybinds"><br><sub>Keybinds</sub></td>
  </tr>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/monitors.png" alt="Monitors"><br><sub>Monitors</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/autostart.png" alt="Autostart"><br><sub>Autostart</sub></td>
  </tr>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/rules.png" alt="Window rules"><br><sub>Window rules</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/env.png" alt="Environment variables"><br><sub>Environment variables</sub></td>
  </tr>
  <tr>
    <td width="50%" align="center"><img src="docs/screenshots/en/wallpaper.png" alt="Wallpaper (hyprpaper)"><br><sub>Wallpaper (hyprpaper)</sub></td>
    <td width="50%" align="center"><img src="docs/screenshots/en/config.png" alt="Generated config"><br><sub>Generated config</sub></td>
  </tr>
</table>

## Supported Hyprland versions

| Hyprland | Config format | Support |
|---|---|---|
| **0.55 and newer** | Lua (`hyprland.lua`) | ✅ |
| **0.53 – 0.54** | hyprlang (`hyprland.conf`) | ✅ |
| older than 0.53 | old hyprlang syntax | ❌ |

The format is detected automatically: since 0.55, Hyprland loads `hyprland.lua` and only falls back to the old `hyprland.conf` when there is no Lua file, so the app writes to whichever one your Hyprland actually loads. You can also choose the format by hand on the **Themes** page.

## Installation

### Arch Linux

A `PKGBUILD` is included. It builds the latest version from git:

```bash
git clone https://github.com/NeXtagG/easy-hyprland.git
cd easy-hyprland/packaging/arch
makepkg -si
```

The app then appears in your launcher as **Easy Hyprland**.

### NixOS

Try it without installing:

```bash
nix run github:NeXtagG/easy-hyprland
```

Install it with flakes (`flake.nix` of your system):

```nix
{
  inputs.easy-hyprland.url = "github:NeXtagG/easy-hyprland";

  outputs = { nixpkgs, easy-hyprland, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [
            easy-hyprland.packages.${pkgs.stdenv.hostPlatform.system}.default
          ];
        })
      ];
    };
  };
}
```

Without flakes:

```bash
nix-env -f https://github.com/NeXtagG/easy-hyprland/archive/main.tar.gz -i
```

#### If your Hyprland config is managed by home-manager

With home-manager, `hyprland.lua` / `hyprland.conf` is a read-only link into `/nix/store`. Easy Hyprland never touches it: it writes its own file and shows the one line you need to add yourself. Add it once and rebuild:

Lua config (Hyprland 0.55+), if you manage the file through home-manager:

```nix
xdg.configFile."hypr/hyprland.lua".text = ''
  -- your config
  require("easy-hyprland")
'';
```

hyprlang config (Hyprland 0.53–0.54):

```nix
wayland.windowManager.hyprland.extraConfig = ''
  source = ~/.config/hypr/easy-hyprland.conf
'';
```

After that, all changes are made in the app as usual: **Save and apply** rewrites only `easy-hyprland.lua` / `easy-hyprland.conf`.

### Other distributions

Install Rust (for example with [rustup](https://rustup.rs)), then:

```bash
git clone https://github.com/NeXtagG/easy-hyprland.git
cd easy-hyprland
cargo install --locked --path .
```

The binary is installed to `~/.cargo/bin/easy-hyprland`. Make sure `~/.cargo/bin` is in your `PATH`. To add it to your launcher, copy `packaging/easy-hyprland.desktop` to `~/.local/share/applications/`.

## Usage

1. Start `easy-hyprland` from a terminal or from your launcher (wofi, rofi, hyprlauncher…).
2. Pick a section on the left, for example **Themes** or **Windows**.
3. Change the settings. The preview on the right updates immediately.
4. Press **Save and apply** (or **Ctrl+S**). Hyprland reloads right away, no re-login needed.

The language is picked from your system locale. You can change it at the bottom of the sidebar.

| Button | What it does |
|---|---|
| ✔ Save and apply | writes the config, runs `hyprctl reload` and shows any config errors |
| 💾 Save only | only writes the files |
| ↩ Revert changes | goes back to the last saved settings |
| ⟲ Defaults | resets everything to the default values |

If you close the window with unsaved changes, the app asks whether to save them.

## How it works

- Your settings are written to a separate file:
  - Lua: `~/.config/hypr/easy-hyprland.lua`
  - hyprlang: `~/.config/hypr/easy-hyprland.conf`
- One line is appended to the end of your main config (`hyprland.lua` or `hyprland.conf`):
  ```lua
  require("easy-hyprland")                         -- Lua
  ```
  ```ini
  source = /home/you/.config/hypr/easy-hyprland.conf   # hyprlang
  ```
  Before that, a copy named `hyprland.lua.bak-<time>` (or `hyprland.conf.bak-<time>`) is created. Your existing config stays in place, and the app's settings are applied on top of it.
- If there is no main config yet, it is created.
- The app's own state lives in `~/.config/easy-hyprland/settings.json`.
- With the wallpaper enabled, `~/.config/hypr/hyprpaper.conf` is written. A foreign hyprpaper config is backed up first.
- `$XDG_CONFIG_HOME` is respected.

### Uninstall

Remove the `require("easy-hyprland")` or `source = …easy-hyprland.conf` line from your main config, then:

```bash
cargo uninstall easy-hyprland
rm -r ~/.config/easy-hyprland ~/.config/hypr/easy-hyprland.*
```

## Development

```bash
cargo run              # start the app
cargo test             # unit tests
tests/validate.sh      # check generated configs against Hyprland 0.56.2 (needs lua5.4 and python3)
```

`tests/validate.sh` generates sample configs in both formats and checks every option, dispatcher and window rule effect against lists taken from the Hyprland 0.56.2 sources (`tests/hyprland-0.56.2/`). The Lua configs are executed against a strict mock of the `hl` API.

Pull requests are welcome!

## License

This project is licensed under the **GNU GPL v3.0 or later**. The full text is in [LICENSE](LICENSE).

In plain words:
- ✅ anyone can **use** the program for free, including at work;
- ✅ you may **change** the code and **share** it;
- ❗ if you share a modified version, its source code must also be **open** under the same GPL-3.0 license.
