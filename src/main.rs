#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

// Declare the folder-module. Rust looks for object_calculator/mod.rs automatically!
mod object_calculator;
mod triangle_calculator;
mod force_calculator;
mod scraper;



slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Init
    force_calculator::setup_ui_dropdowns(&ui);

    std::thread::spawn(|| {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(e) => {
                eprintln!("Failed to create Tokio runtime for scraper: {}", e);
                return;
            }
        };
        rt.block_on(async {
            scraper::scraper::run(scraper::constants::START_URL).await;
        })
    });

    // Call the re-exported function from your module
    object_calculator::setup_callbacks(&ui);
    triangle_calculator::setup_callbacks(&ui);
    force_calculator::setup_callback(&ui);

    ui.run()?;
    Ok(())
}