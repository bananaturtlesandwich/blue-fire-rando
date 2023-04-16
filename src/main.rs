#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use blue_fire_rando::Rando;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::epaint::Vec2::new(400.0, 360.0)),
            resizable: false,
            icon_data: Some(eframe::IconData {
                rgba: include_bytes!("umby.rgba").to_vec(),
                width: 32,
                height: 32,
            }),
            ..Default::default()
        },
        Box::new(|ctx| Box::new(Rando::new(ctx))),
    )
}
