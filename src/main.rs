// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(e) = mtorrent_egui::run() {
        eprintln!("Error: {}", e);
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("mtorrent")
            .set_description(e.to_string())
            .show();
        std::process::exit(1);
    }
}
