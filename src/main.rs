#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use blue_fire_rando::Rando;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "",
        eframe::NativeOptions {
            // initial_window_size: Some(eframe::epaint::Vec2::new(500.0, 390.0)),
            viewport: eframe::egui::ViewportBuilder::default()
                .with_resizable(false)
                .with_icon(eframe::egui::IconData {
                    rgba: include_bytes!("umby.rgba").to_vec(),
                    width: 32,
                    height: 32,
                }),
            ..Default::default()
        },
        Box::new(|ctx| Ok(Box::new(Rando::new(ctx)))),
    )
}
