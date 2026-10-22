mod app;
mod config;
mod hypr;
mod i18n;
mod presets;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Easy Hyprland")
            .with_app_id("easy-hyprland")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([820.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Easy Hyprland",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
