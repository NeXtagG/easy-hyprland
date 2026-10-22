[English](README.md) · **Русский** · [Deutsch](README.de.md)

# Easy Hyprland

Программа для простой настройки [Hyprland](https://hyprland.org) с понятным графическим интерфейсом.
Не нужно разбираться в синтаксисе конфигов: двигай ползунки, выбирай цвета и нажимай **«Сохранить и применить»**.

Написано на **Rust** с [egui](https://github.com/emilk/egui). Интерфейс есть на русском, английском и немецком.

![Скриншот](docs/screenshot-ru.png)

## Возможности

- 🎨 **Темы**: Catppuccin, Nord, Gruvbox, Неон, Минимализм, каждая в один клик
- 🪟 **Окна**: отступы, рамки, градиент рамки, скругление, раскладка (dwindle, master, scrolling, monocle)
- ✨ **Эффекты**: прозрачность, размытие (blur), тени, затемнение неактивных окон
- 🎞 **Анимации**: включение, скорость, стиль (плавный, быстрый, пружинистый)
- ⌨ **Ввод**: раскладки и их переключение, чувствительность мыши, тачпад, свайп тремя пальцами между рабочими столами
- 🔑 **Горячие клавиши**: программы по умолчанию и редактор сочетаний
- 🖥 **Мониторы**: разрешение, частота, позиция, масштаб
- 🚀 **Автозапуск** программ
- 📐 **Правила окон**
- 🌿 **Переменные окружения**
- 🖼 **Обои** через hyprpaper
- 📄 **Готовый конфиг**: просмотр, копирование и свои строки
- 👀 **Живой предпросмотр** окон прямо в программе
- 🛟 **Безопасно**: перед изменением конфига делается резервная копия, а после применения показываются ошибки конфига от Hyprland

## Поддерживаемые версии Hyprland

| Hyprland | Формат конфига | Поддержка |
|---|---|---|
| **0.55 и новее** | Lua (`hyprland.lua`) | ✅ |
| **0.53 – 0.54** | hyprlang (`hyprland.conf`) | ✅ |
| старше 0.53 | старый синтаксис hyprlang | ❌ |

Формат определяется автоматически. С версии 0.55 Hyprland загружает `hyprland.lua`, а старый `hyprland.conf` читает только если Lua-файла нет. Поэтому программа пишет в тот файл, который твой Hyprland реально загружает. Формат можно выбрать и вручную на странице **«Темы»**.

## Установка

### Arch Linux

В репозитории есть `PKGBUILD`, он собирает последнюю версию из git:

```bash
git clone https://github.com/NeXtagG/easy-hyprland.git
cd easy-hyprland/packaging/arch
makepkg -si
```

После этого программа появится в меню приложений как **Easy Hyprland**.

### NixOS

Попробовать без установки:

```bash
nix run github:NeXtagG/easy-hyprland
```

Установить через flakes (в `flake.nix` твоей системы):

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

Без flakes:

```bash
nix-env -f https://github.com/NeXtagG/easy-hyprland/archive/main.tar.gz -i
```

#### Если конфиг Hyprland создаёт home-manager

С home-manager файл `hyprland.lua` или `hyprland.conf` — это ссылка только для чтения в `/nix/store`. Easy Hyprland его никогда не трогает: программа пишет свой файл и показывает строку, которую нужно добавить самому. Добавь её один раз и пересобери систему:

Конфиг на Lua (Hyprland 0.55+), если файл задаётся через home-manager:

```nix
xdg.configFile."hypr/hyprland.lua".text = ''
  -- твой конфиг
  require("easy-hyprland")
'';
```

Конфиг hyprlang (Hyprland 0.53–0.54):

```nix
wayland.windowManager.hyprland.extraConfig = ''
  source = ~/.config/hypr/easy-hyprland.conf
'';
```

Дальше все изменения делаются в программе как обычно: **«Сохранить и применить»** перезаписывает только `easy-hyprland.lua` или `easy-hyprland.conf`.

### Другие дистрибутивы

Установи Rust (например, через [rustup](https://rustup.rs)), затем:

```bash
git clone https://github.com/NeXtagG/easy-hyprland.git
cd easy-hyprland
cargo install --locked --path .
```

Программа установится в `~/.cargo/bin/easy-hyprland`. Проверь, что `~/.cargo/bin` есть в `PATH`. Чтобы добавить её в меню приложений, скопируй `packaging/easy-hyprland.desktop` в `~/.local/share/applications/`.

## Использование

1. Запусти `easy-hyprland` из терминала или из меню (wofi, rofi, hyprlauncher…).
2. Слева выбери раздел, например **«Темы»** или **«Окна»**.
3. Меняй настройки. Предпросмотр справа обновляется сразу.
4. Нажми **«Сохранить и применить»** (или **Ctrl+S**). Hyprland обновится сразу, без перезахода.

Язык выбирается по языку системы. Поменять его можно внизу боковой панели.

| Кнопка | Что делает |
|---|---|
| ✔ Сохранить и применить | записывает конфиг, выполняет `hyprctl reload` и показывает ошибки конфига, если они есть |
| 💾 Только сохранить | только записывает файлы |
| ↩ Отменить изменения | возвращает последние сохранённые настройки |
| ⟲ По умолчанию | сбрасывает всё к стандартным значениям |

Если закрыть окно с несохранёнными изменениями, программа спросит, сохранить ли их.

## Как это работает

- Настройки записываются в отдельный файл:
  - Lua: `~/.config/hypr/easy-hyprland.lua`
  - hyprlang: `~/.config/hypr/easy-hyprland.conf`
- В конец основного конфига (`hyprland.lua` или `hyprland.conf`) добавляется одна строка:
  ```lua
  require("easy-hyprland")                         -- Lua
  ```
  ```ini
  source = /home/ты/.config/hypr/easy-hyprland.conf   # hyprlang
  ```
  Перед этим создаётся копия `hyprland.lua.bak-<время>` (или `hyprland.conf.bak-<время>`). Твой старый конфиг остаётся на месте, а настройки из программы применяются поверх него.
- Если основного конфига нет, он будет создан.
- Состояние программы хранится в `~/.config/easy-hyprland/settings.json`.
- Если включены обои, записывается `~/.config/hypr/hyprpaper.conf`. Чужой конфиг hyprpaper сначала сохраняется в копию.
- Учитывается `$XDG_CONFIG_HOME`.

### Как удалить

Удали строку `require("easy-hyprland")` или `source = …easy-hyprland.conf` из основного конфига и выполни:

```bash
cargo uninstall easy-hyprland
rm -r ~/.config/easy-hyprland ~/.config/hypr/easy-hyprland.*
```

## Разработка

```bash
cargo run              # запуск
cargo test             # юнит-тесты
tests/validate.sh      # проверка конфигов по Hyprland 0.56.2 (нужны lua5.4 и python3)
```

`tests/validate.sh` генерирует примеры конфигов в обоих форматах и сверяет каждую опцию, команду и эффект правил окон со списками из исходников Hyprland 0.56.2 (`tests/hyprland-0.56.2/`). Lua-конфиги выполняются на строгой заглушке API `hl`.

Pull request'ы приветствуются!

## Лицензия

Проект распространяется под лицензией **GNU GPL v3.0 или новее**, полный текст в файле [LICENSE](LICENSE).

Простыми словами:
- ✅ любой может **бесплатно использовать** программу, в том числе для работы;
- ✅ можно **изменять** код и **распространять** его;
- ❗ если ты распространяешь изменённую версию, её код тоже должен быть **открытым** под той же лицензией GPL-3.0.
