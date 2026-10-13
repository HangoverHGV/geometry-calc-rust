use std::path::Path;
use winprint::printer::{FilePrinter, PdfiumPrinter, PrinterDevice};
use winprint::ticket::{
    FeatureOptionPackWithPredefined, PredefinedPageOrientation, PrintCapabilities,
    PrintTicket, PrintTicketBuilder,
};
use crate::PrintOrientation;

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

fn build_ticket(
    device: &PrinterDevice,
    orientation: PrintOrientation,
) -> Result<PrintTicket, String> {
    let wanted = match orientation {
        PrintOrientation::Portrait => PredefinedPageOrientation::Portrait,
        PrintOrientation::Landscape => PredefinedPageOrientation::Landscape,
    };

    let capabilities = PrintCapabilities::fetch(device)
        .map_err(|e| format!("Failed to read printer capabilities: {:?}", e))?;

    let orientation_option = capabilities
        .page_orientations()
        .find(|o| o.as_predefined_name() == Some(wanted))
        .ok_or_else(|| format!("Printer does not support {:?} orientation", wanted))?;

    let mut builder = PrintTicketBuilder::new(device)
        .map_err(|e| format!("Failed to create print ticket: {:?}", e))?;
    builder
        .merge(orientation_option)
        .map_err(|e| format!("Failed to set orientation: {:?}", e))?;
    builder
        .build()
        .map_err(|e| format!("Failed to build print ticket: {:?}", e))
}

pub fn print_files(
    printer_name: &str,
    files: Vec<String>,
    orientation: PrintOrientation,
) -> Result<usize, String> {
    let devices = PrinterDevice::all()
        .map_err(|e| format!("Failed to fetch Windows printers: {}", e))?;

    let selected_device = devices
        .into_iter()
        .find(|dev| dev.name() == printer_name)
        .ok_or_else(|| format!("Printer '{}' not found in Windows devices.", printer_name))?;

    let ticket = build_ticket(&selected_device, orientation)?;

    let pdf_printer = PdfiumPrinter::new(selected_device);
    let mut failed_count = 0;
    let total_files = files.len();

    for file in &files {
        let path = Path::new(file);
        if let Err(e) = pdf_printer.print(path, ticket.clone()) {
            eprintln!("Failed to print {}: {:?}", file, e);
            failed_count += 1;
        }
    }

    if failed_count > 0 {
        Err(format!(
            "Completed with errors: {} out of {} files failed to print.",
            failed_count, total_files
        ))
    } else {
        Ok(total_files)
    }
}