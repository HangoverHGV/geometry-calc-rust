use std::path::Path;
use winprint::printer::{FilePrinter, PdfiumPrinter, PrinterDevice};

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

pub fn print_files(printer_name: &str, files: Vec<String>) -> Result<usize, String> {
    let devices = PrinterDevice::all()
        .map_err(|e| format!("Failed to fetch Windows printers: {}", e))?;

    let selected_device = devices
        .into_iter()
        .find(|dev| dev.name() == printer_name)
        .ok_or_else(|| format!("Printer '{}' not found in Windows devices.", printer_name))?;

    let pdf_printer = PdfiumPrinter::new(selected_device);
    let mut failed_count = 0;
    let total_files = files.len();

    for file in files {
        let path = Path::new(&file);
        match pdf_printer.print(path, Default::default()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to print {}: {:?}", file, e);
                failed_count += 1;
            }
        }
    }

    if failed_count > 0 {
        Err(format!("Completed with errors: {} out of {} files failed to print.", failed_count, total_files))
    } else {
        Ok(total_files)
    }
}