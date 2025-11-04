use eframe::egui;
use std::sync::Arc;
use std::process;

mod http_client;
mod network_utils;
mod ui;
mod cli;

use http_client::HttpClient;
use ui::PeekApp;

#[cfg(windows)]
fn attach_console() {
    use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

fn load_icon_from_ico(ico_bytes: &[u8]) -> egui::IconData {
    use image::GenericImageView;
    
    let image = image::load_from_memory(ico_bytes)
        .expect("Failed to load icon");
    
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw();
    
    egui::IconData {
        rgba,
        width,
        height,
    }
}

fn main() {
    // On Windows with windows_subsystem = "windows", we need to manually attach
    // to console if running with CLI arguments
    #[cfg(windows)]
    {
        let args: Vec<String> = std::env::args().collect();
        // If there are CLI arguments (beyond just the program name), attach console
        if args.len() > 1 {
            attach_console();
        }
    }
    
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // First try parsing CLI args. If a CLI subcommand was given, handle it and exit.
    let code = cli::run_from_args();
    if code != 127 {
        // Either succeeded or failed with an exit code.
        process::exit(code);
    }

    // No CLI subcommand given -> start GUI (default behavior).
    
    // Load the icon from .ico file
    let icon_bytes = include_bytes!("../assets/pk.ico");
    let icon = load_icon_from_ico(icon_bytes);
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([960.0, 600.0])
            .with_icon(icon),
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
