use std::path::Path;
use winprint::printer::{FilePrinter, PdfiumPrinter, PrinterDevice};

/// Fetches the list of printers directly using winprint to ensure perfect name matching
pub fn get_all_printers() -> Result<Option<Vec<String>>, String> {
    let devices = match PrinterDevice::all() {
        Ok(devs) => devs,
        Err(e) => return Err(format!("Failed to fetch Windows printers: {}", e)),
    };

    if devices.is_empty() {
        return Ok(None);
    }

    let printer_names: Vec<String> = devices
        .into_iter()
        .map(|device| device.name().to_string())
        .collect();

    Ok(Some(printer_names))
}

pub fn print_files(printer_name: &str, files: Vec<String>) {
    // 1. Get all Windows printer devices and find the one that matches the UI selection
    let devices = match PrinterDevice::all() {
        Ok(devs) => devs,
        Err(e) => {
            eprintln!("Failed to fetch Windows printers: {}", e);
            return;
        }
    };

    let selected_device = devices.into_iter().find(|dev| dev.name() == printer_name);

    if let Some(device) = selected_device {
        // 2. Wrap the device in a PdfiumPrinter to handle PDFs natively
        let pdf_printer = PdfiumPrinter::new(device);

        // 3. Print each file
        for file in files {
            println!("Attempting to print PDF using winprint: {}", file);
            let path = Path::new(&file);

            // Print with default ticket (default Windows printing preferences)
            match pdf_printer.print(path, Default::default()) {
                Ok(_) => println!("Successfully queued: {}", file),
                Err(e) => eprintln!("Failed to print {}: {:?}", e),
            }
        }
    } else {
        eprintln!("Printer '{}' not found in Windows devices.", printer_name);
    }
}