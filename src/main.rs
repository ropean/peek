use eframe::egui;
use std::sync::Arc;

mod http_client;
mod network_utils;
mod ui;

use http_client::HttpClient;
use ui::PeekApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([960.0, 600.0]),
        ..Default::default()
    };

    let http_client = Arc::new(std::sync::Mutex::new(HttpClient::new()));

    eframe::run_native(
        "peek",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(PeekApp::new(http_client))
        }),
    )
}
