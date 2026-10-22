use crate::config::{Format, Settings};
use crate::i18n::Lang;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Marker present in the first line of every file we generate.
const MARKER: &str = "Easy Hyprland";

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
}

pub fn hypr_dir() -> PathBuf {
    config_dir().join("hypr")
}

fn app_dir() -> PathBuf {
    config_dir().join("easy-hyprland")
}

fn settings_path() -> PathBuf {
    app_dir().join("settings.json")
}

fn lang_path() -> PathBuf {
    app_dir().join("lang")
}

/// Loads saved settings. A missing file yields defaults; a broken file yields
/// defaults plus the error so the UI can tell the user.
pub fn load_settings(lang: Lang) -> (Settings, Option<String>) {
    match fs::read_to_string(settings_path()) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(s) => (s, None),
            Err(e) => (Settings::for_lang(lang), Some(e.to_string())),
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => (Settings::for_lang(lang), None),
        Err(e) => (Settings::for_lang(lang), Some(e.to_string())),
    }
}

pub fn load_lang() -> Option<Lang> {
    fs::read_to_string(lang_path()).ok().and_then(|s| Lang::from_code(&s))
}

pub fn save_lang(lang: Lang) {
    let _ = fs::create_dir_all(app_dir());
    let _ = fs::write(lang_path(), lang.code());
}

/// Parses `hyprctl version` output into (major, minor).
fn parse_version(out: &str) -> Option<(u32, u32)> {
    let start = out.find(|c: char| c.is_ascii_digit())?;
    let mut nums = out[start..].split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty());
    Some((nums.next()?.parse().ok()?, nums.next()?.parse().ok()?))
}

pub fn hyprland_version() -> Option<(u32, u32)> {
    let out = Command::new("hyprctl").arg("version").output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().find(|l| l.contains("Tag:") || l.starts_with("Hyprland"))?;
    let tag = line.split("Tag:").nth(1).unwrap_or(line);
    let tag = tag.split_once('v').map(|(_, rest)| rest).unwrap_or(tag);
    parse_version(tag)
}

/// Picks the format Hyprland will actually load:
/// hyprland.lua wins over hyprland.conf (0.55+); with neither, use the running version.
pub fn resolve_format(requested: Format) -> Format {
    resolve_format_in(requested, &hypr_dir(), hyprland_version)
}

fn resolve_format_in(requested: Format, dir: &Path, version: impl Fn() -> Option<(u32, u32)>) -> Format {
    if requested != Format::Auto {
        return requested;
    }
    if dir.join("hyprland.lua").exists() {
        return Format::Lua;
    }
    if dir.join("hyprland.conf").exists() {
        return Format::Hyprlang;
    }
    match version() {
        Some(v) if v < (0, 55) => Format::Hyprlang,
        _ => Format::Lua,
    }
}

pub fn generated_file_name(format: Format) -> &'static str {
    match format {
        Format::Lua => "easy-hyprland.lua",
        _ => "easy-hyprland.conf",
    }
}

pub struct SaveReport {
    pub path: PathBuf,
    pub backups: Vec<PathBuf>,
    /// Set when the main config can't be edited (e.g. managed by Nix/home-manager):
    /// (main config path, line the user has to add themselves).
    pub manual_include: Option<(PathBuf, String)>,
}

pub fn save(settings: &Settings, lang: Lang) -> io::Result<SaveReport> {
    save_in(settings, lang, &hypr_dir(), &settings_path(), resolve_format(settings.format))
}

/// Writes the settings JSON, the generated config and the include line.
fn save_in(settings: &Settings, lang: Lang, dir: &Path, settings_file: &Path, format: Format) -> io::Result<SaveReport> {
    if let Some(parent) = settings_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(settings_file, serde_json::to_string_pretty(settings)?)?;

    fs::create_dir_all(dir)?;
    let generated = dir.join(generated_file_name(format));
    fs::write(&generated, settings.generate(format, lang))?;

    let mut backups = Vec::new();
    let (main, include) = match format {
        Format::Lua => (dir.join("hyprland.lua"), "require(\"easy-hyprland\")".to_string()),
        // Absolute path so it also works with a custom $XDG_CONFIG_HOME.
        _ => (dir.join("hyprland.conf"), format!("source = {}", generated.display())),
    };
    let mut manual_include = None;
    if is_nix_managed(&main) {
        if !has_include(&main, generated_file_name(format)) {
            manual_include = Some((main.clone(), include.clone()));
        }
    } else {
        match ensure_include(&main, generated_file_name(format), &include) {
            Ok(Some(b)) => backups.push(b),
            Ok(None) => {}
            Err(e) if matches!(e.kind(), io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem) => {
                manual_include = Some((main.clone(), include.clone()));
            }
            Err(e) => return Err(e),
        }
    }

    if settings.wallpaper_enabled && !settings.wallpaper.trim().is_empty() {
        let paper = dir.join("hyprpaper.conf");
        match fs::read_to_string(&paper) {
            Ok(old) if !old.lines().next().unwrap_or("").contains(MARKER) => backups.push(backup(&paper)?),
            _ => {}
        }
        fs::write(paper, settings.generate_hyprpaper(lang))?;
    }

    Ok(SaveReport { path: generated, backups, manual_include })
}

/// True when the file is a symlink into the Nix store (home-manager, NixOS modules).
/// Such files are read-only and get replaced on the next rebuild anyway.
fn is_nix_managed(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
        && fs::canonicalize(path).is_ok_and(|p| p.starts_with("/nix/store"))
}

fn has_include(main: &Path, marker: &str) -> bool {
    let marker_stem = marker.trim_end_matches(".lua").trim_end_matches(".conf");
    fs::read_to_string(main).is_ok_and(|c| c.lines().any(|l| !is_comment(l) && l.contains(marker_stem)))
}

fn is_comment(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('#') || t.starts_with("--")
}

/// Makes sure `main` contains an active (uncommented) line mentioning `marker`.
/// Returns the backup path if the file had to be modified.
fn ensure_include(main: &Path, marker: &str, include: &str) -> io::Result<Option<PathBuf>> {
    match fs::read_to_string(main) {
        Ok(_) if has_include(main, marker) => Ok(None),
        Ok(content) => {
            let b = backup(main)?;
            let comment = if main.extension().is_some_and(|e| e == "lua") { "--" } else { "#" };
            let sep = if content.ends_with('\n') || content.is_empty() { "" } else { "\n" };
            fs::write(main, format!("{content}{sep}\n{comment} {MARKER}\n{include}\n"))?;
            Ok(Some(b))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let comment = if main.extension().is_some_and(|e| e == "lua") { "--" } else { "#" };
            fs::write(main, format!("{comment} {MARKER}\n{include}\n"))?;
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

fn backup(path: &Path) -> io::Result<PathBuf> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut n = 0;
    loop {
        let mut name = path.file_name().unwrap_or_default().to_os_string();
        name.push(if n == 0 { format!(".bak-{ts}") } else { format!(".bak-{ts}-{n}") });
        let dest = path.with_file_name(name);
        if !dest.exists() {
            fs::copy(path, &dest)?;
            return Ok(dest);
        }
        n += 1;
    }
}

fn expand_home(p: &str) -> String {
    match (p.strip_prefix("~/"), dirs::home_dir()) {
        (Some(rest), Some(home)) => home.join(rest).display().to_string(),
        _ => p.to_string(),
    }
}

pub enum ApplyError {
    NoHyprctl,
    Failed(String),
}

/// Reloads Hyprland, then returns any config errors it reports.
pub fn reload(settings: &Settings) -> Result<Vec<String>, ApplyError> {
    let out = Command::new("hyprctl").arg("reload").output().map_err(|_| ApplyError::NoHyprctl)?;
    if !out.status.success() {
        let mut msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if msg.is_empty() {
            msg = String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
        return Err(ApplyError::Failed(msg));
    }

    if settings.wallpaper_enabled && !settings.wallpaper.trim().is_empty() {
        let arg = format!(",{},{}", expand_home(settings.wallpaper.trim()), settings.wallpaper_fit);
        let _ = Command::new("hyprctl").args(["hyprpaper", "wallpaper", &arg]).output();
    }

    let errors = Command::new("hyprctl")
        .arg("configerrors")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    Ok(errors
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.eq_ignore_ascii_case("ok") && !l.contains("no errors"))
        .map(String::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("easy-hyprland-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn parses_versions() {
        assert_eq!(parse_version("0.56.2"), Some((0, 56)));
        assert_eq!(parse_version("0.54.0 based on branch"), Some((0, 54)));
        assert_eq!(parse_version("nothing"), None);
    }

    #[test]
    fn auto_format_follows_existing_files() {
        let d = tmp("detect");
        assert_eq!(resolve_format_in(Format::Auto, &d, || Some((0, 54))), Format::Hyprlang);
        assert_eq!(resolve_format_in(Format::Auto, &d, || Some((0, 56))), Format::Lua);
        assert_eq!(resolve_format_in(Format::Auto, &d, || None), Format::Lua);
        fs::write(d.join("hyprland.conf"), "").unwrap();
        assert_eq!(resolve_format_in(Format::Auto, &d, || Some((0, 56))), Format::Hyprlang);
        fs::write(d.join("hyprland.lua"), "").unwrap();
        assert_eq!(resolve_format_in(Format::Auto, &d, || Some((0, 54))), Format::Lua);
        assert_eq!(resolve_format_in(Format::Hyprlang, &d, || None), Format::Hyprlang);
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn save_adds_include_once_and_backs_up() {
        for format in [Format::Lua, Format::Hyprlang] {
            let d = tmp(&format!("save-{format:?}"));
            let main = d.join(if format == Format::Lua { "hyprland.lua" } else { "hyprland.conf" });
            let original = if format == Format::Lua { "hl.config({})" } else { "general {\n}\n" };
            fs::write(&main, original).unwrap();
            let mut s = Settings::default();
            s.wallpaper_enabled = true;
            s.wallpaper = "~/w.png".into();

            let r = save_in(&s, Lang::En, &d, &d.join("settings.json"), format).unwrap();
            assert_eq!(r.backups.len(), 1);
            let content = fs::read_to_string(&main).unwrap();
            assert!(content.starts_with(original));
            let include = if format == Format::Lua { "require(\"easy-hyprland\")" } else { "source = " };
            assert_eq!(content.matches(include).count(), 1);
            assert_eq!(fs::read_to_string(&r.backups[0]).unwrap(), original);

            // Saving again must not add a second include or more backups.
            let r2 = save_in(&s, Lang::En, &d, &d.join("settings.json"), format).unwrap();
            assert!(r2.backups.is_empty(), "{:?}", r2.backups);
            assert_eq!(fs::read_to_string(&main).unwrap(), content);
            assert!(r.path.exists());

            let saved: Settings = serde_json::from_str(&fs::read_to_string(d.join("settings.json")).unwrap()).unwrap();
            assert!(saved == s);
            fs::remove_dir_all(d).unwrap();
        }
    }

    #[test]
    fn commented_include_does_not_count() {
        let d = tmp("commented");
        let main = d.join("hyprland.conf");
        fs::write(&main, "# source = ~/.config/hypr/easy-hyprland.conf\n").unwrap();
        save_in(&Settings::default(), Lang::En, &d, &d.join("s.json"), Format::Hyprlang).unwrap();
        let content = fs::read_to_string(&main).unwrap();
        assert!(content.lines().any(|l| l.starts_with("source = ") && l.contains("easy-hyprland.conf")));
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn read_only_main_config_asks_for_manual_include() {
        use std::os::unix::fs::PermissionsExt;
        let d = tmp("readonly");
        let main = d.join("hyprland.lua");
        fs::write(&main, "hl.config({})\n").unwrap();
        fs::set_permissions(&main, fs::Permissions::from_mode(0o444)).unwrap();
        // Root ignores file permissions, so only check the outcome when the write really fails.
        let writable = fs::OpenOptions::new().append(true).open(&main).is_ok();
        let r = save_in(&Settings::default(), Lang::En, &d, &d.join("s.json"), Format::Lua).unwrap();
        assert!(r.path.exists());
        if !writable {
            let (path, line) = r.manual_include.expect("manual include expected");
            assert_eq!(path, main);
            assert_eq!(line, "require(\"easy-hyprland\")");
            assert_eq!(fs::read_to_string(&main).unwrap(), "hl.config({})\n");
        }
        fs::set_permissions(&main, fs::Permissions::from_mode(0o644)).unwrap();
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn nix_store_symlink_is_never_modified() {
        // Simulates a home-manager link; the target only lives in /nix/store on NixOS.
        let d = tmp("nixlink");
        let target = d.join("store-hyprland.conf");
        fs::write(&target, "general {\n}\n").unwrap();
        std::os::unix::fs::symlink(&target, d.join("hyprland.conf")).unwrap();
        assert!(!is_nix_managed(&d.join("hyprland.conf")));
        assert!(!is_nix_managed(&target));
        if let Some(store_file) = fs::read_dir("/nix/store").ok().and_then(|mut it| it.find_map(|e| e.ok()).map(|e| e.path())) {
            let link = d.join("hm-link");
            std::os::unix::fs::symlink(&store_file, &link).unwrap();
            assert!(is_nix_managed(&link));
        }
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn creates_main_config_when_missing() {
        let d = tmp("missing");
        save_in(&Settings::default(), Lang::En, &d, &d.join("s.json"), Format::Lua).unwrap();
        assert!(fs::read_to_string(d.join("hyprland.lua")).unwrap().contains("require(\"easy-hyprland\")"));
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn foreign_hyprpaper_is_backed_up_only_once() {
        let d = tmp("paper");
        fs::write(d.join("hyprpaper.conf"), "preload = old\n").unwrap();
        let mut s = Settings::default();
        s.wallpaper_enabled = true;
        s.wallpaper = "/w.png".into();
        let r1 = save_in(&s, Lang::En, &d, &d.join("s.json"), Format::Lua).unwrap();
        let r2 = save_in(&s, Lang::En, &d, &d.join("s.json"), Format::Lua).unwrap();
        assert_eq!(r1.backups.iter().filter(|b| b.to_string_lossy().contains("hyprpaper")).count(), 1);
        assert!(r2.backups.is_empty());
        fs::remove_dir_all(d).unwrap();
    }
}
