use crate::config::Settings;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const SOURCE_LINE: &str = "source = ~/.config/hypr/easy-hyprland.conf";

fn config_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"))
}

pub fn hypr_dir() -> PathBuf {
    config_dir().join("hypr")
}

fn settings_path() -> PathBuf {
    config_dir().join("easy-hyprland").join("settings.json")
}

pub fn load_settings() -> Settings {
    fs::read_to_string(settings_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Saves settings, writes easy-hyprland.conf (and hyprpaper.conf) and makes sure
/// hyprland.conf sources it. Existing files are backed up before being modified.
pub fn save(settings: &Settings) -> io::Result<String> {
    let sp = settings_path();
    fs::create_dir_all(sp.parent().unwrap())?;
    fs::write(&sp, serde_json::to_string_pretty(settings)?)?;

    let dir = hypr_dir();
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("easy-hyprland.conf"), settings.generate())?;

    let main = dir.join("hyprland.conf");
    let mut notes = Vec::new();
    match fs::read_to_string(&main) {
        Ok(content) if content.contains("easy-hyprland.conf") => {}
        Ok(content) => {
            let backup = backup(&main)?;
            fs::write(&main, format!("{content}\n\n# Easy Hyprland\n{SOURCE_LINE}\n"))?;
            notes.push(format!("копия старого конфига: {}", backup.display()));
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::write(&main, format!("# Easy Hyprland\n{SOURCE_LINE}\n"))?;
        }
        Err(e) => return Err(e),
    }

    if settings.wallpaper_enabled && !settings.wallpaper.is_empty() {
        let paper = dir.join("hyprpaper.conf");
        if paper.exists() {
            backup(&paper)?;
        }
        fs::write(paper, settings.generate_hyprpaper())?;
    }

    let mut msg = format!("Сохранено в {}", dir.join("easy-hyprland.conf").display());
    if !notes.is_empty() {
        msg.push_str(&format!(" ({})", notes.join(", ")));
    }
    Ok(msg)
}

fn backup(path: &PathBuf) -> io::Result<PathBuf> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut name = path.file_name().unwrap().to_os_string();
    name.push(format!(".bak-{ts}"));
    let dest = path.with_file_name(name);
    fs::copy(path, &dest)?;
    Ok(dest)
}

pub fn reload() -> Result<String, String> {
    match Command::new("hyprctl").arg("reload").output() {
        Ok(out) if out.status.success() => Ok("Hyprland перезагружен".into()),
        Ok(out) => Err(format!("hyprctl: {}", String::from_utf8_lossy(&out.stderr).trim())),
        Err(_) => Err("hyprctl не найден — Hyprland запущен?".into()),
    }
}
