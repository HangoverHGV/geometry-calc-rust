use std::path::Path;
use winprint::printer::{FilePrinter, PdfiumPrinter, PrinterDevice};
use winprint::ticket::{
    FeatureOptionPackWithPredefined, PredefinedPageOrientation, PrintCapabilities,
    PrintTicket, PrintTicketBuilder,
};
use crate::PrintOrientation;
use super::worker::{FailedFile, PrintJob, PrintReport};

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

/// Prints a whole job. Runs inside the `--print-worker` process (see
/// `worker.rs`), never on the UI thread.
pub fn print_job(job: &PrintJob) -> PrintReport {
    let orientation = if job.landscape {
        PrintOrientation::Landscape
    } else {
        PrintOrientation::Portrait
    };

    let devices = match PrinterDevice::all() {
        Ok(devices) => devices,
        Err(e) => return PrintReport::job_error(format!("Failed to fetch Windows printers: {}", e)),
    };

    let Some(selected_device) = devices.into_iter().find(|dev| dev.name() == job.printer) else {
        return PrintReport::job_error(format!(
            "Printer '{}' not found in Windows devices.",
            job.printer
        ));
    };

    let ticket = match build_ticket(&selected_device, orientation) {
        Ok(ticket) => ticket,
        Err(e) => return PrintReport::job_error(e),
    };

    let pdf_printer = PdfiumPrinter::new(selected_device);
    let mut report = PrintReport::default();

    for file in &job.files {
        match pdf_printer.print(Path::new(file), ticket.clone()) {
            Ok(_) => report.printed += 1,
            Err(e) => report.failed.push(FailedFile {
                file: file.clone(),
                error: format!("{:?}", e),
            }),
        }
    }

    report
}