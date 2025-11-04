use eframe::egui;
use std::sync::Arc;
use std::process;

mod http_client;
mod network_utils;
mod ui;
mod cli;

use http_client::HttpClient;
use ui::PeekApp;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // First try parsing CLI args. If a CLI subcommand was given, handle it and exit.
    let code = cli::run_from_args();
    if code != 127 {
        // Either succeeded or failed with an exit code.
        process::exit(code);
    }

    // No CLI subcommand given -> start GUI (default behavior).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([960.0, 600.0]),
        ..Default::default()
    };

    let http_client = Arc::new(std::sync::Mutex::new(HttpClient::new()));

    if let Err(e) = eframe::run_native(
        "peek",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(PeekApp::new(http_client))
        }),
    ) {
        eprintln!("GUI error: {}", e);
        process::exit(1);
    }
}
