#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

// Declare the folder-module. Rust looks for object_calculator/mod.rs automatically!
mod object_calculator;
mod triangle_calculator;
mod force_calculator;
mod scraper;
mod batch_printing;
mod voronoi_calculator;
mod bom_calculator;
mod schedule_shutdown;
mod sheet_generator;
mod oferta;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Print jobs run in a second copy of this exe (`--print-worker`) so
    // pdfium and the spooler never block the UI. No window in that mode.
    if std::env::args_os().nth(1).is_some_and(|arg| arg == batch_printing::WORKER_ARG) {
        std::process::exit(batch_printing::run_worker());
    }

    let ui = AppWindow::new()?;

    // Init
    force_calculator::setup_ui_dropdowns(&ui);

    ui.on_sync_metal_data(move || {
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
    });

    object_calculator::setup_callbacks(&ui);
    triangle_calculator::setup_callbacks(&ui);
    force_calculator::setup_callback(&ui);
    batch_printing::setup_ui_printers(&ui);
    batch_printing::open_file_dialog(&ui);
    voronoi_calculator::setup_callbacks(&ui);
    bom_calculator::upload_spreadsheet(&ui);
    schedule_shutdown::setup_callbacks(&ui);
    sheet_generator::open_directory_dialog(&ui);
    oferta::setup_callbacks(&ui);

    ui.run()?;
    Ok(())
}