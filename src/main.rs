mod app;
mod constants;
mod polygon;
mod render;
mod utils;

use app::App;
use constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use eframe::{self, NativeOptions};
use eframe::egui::{Vec2, ViewportBuilder};

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::new(DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32)),
            resizable: Some(false),
            maximize_button: Some(false),
            minimize_button: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Fillpoly",
        native_options,
        Box::new(|_| {
            Ok(Box::new(App::default()))
        }
    ))
}
