use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use crate::AppWindow;
use crate::batch_printing::get_all_printers;
use crate::batch_printing::worker::{run_in_worker, PrintJob, PrintReport};
use crate::{PrintOrientation, PrinterItem};
use std::path::Path;
use std::rc::Rc;
use rfd::FileDialog;

const MSG_INFO: i32 = 0;
const MSG_WARN: i32 = 1;
const MSG_ERROR: i32 = 2;

pub fn setup_ui_printers(ui: &AppWindow) {
    let printer_names = match get_all_printers() {
        Ok(Some(names)) => names,
        Ok(None) => {
            // Handle the case where no printers were found
            Vec::new()
        }
        Err(_e) => {
            ui.invoke_trigger_backend_message(2, "Failed to fetch printers".into());
            return;
        }
    };

    let printer_model = Rc::new(VecModel::default());

    for name in printer_names {
        printer_model.push(PrinterItem {
            name: SharedString::from(name),
            is_selected: false,
        });
    }

    let model_rc = ModelRc::new(printer_model.clone());

    ui.set_printers_list(model_rc);

    ui.on_select_printer(move |selected_name| {

        let mut updated_items = Vec::new();


        for i in 0..printer_model.row_count() {
            if let Some(mut item) = printer_model.row_data(i) {

                item.is_selected = item.name == selected_name;
                updated_items.push(item);
            }
        }

        printer_model.set_vec(updated_items);
    });
}

pub fn open_file_dialog(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_open_file_dialog(move |printer_name| {
        let Some(ui) = ui_handle.upgrade() else { return };

        // One batch at a time, so two batches don't interleave in the queue.
        if ui.get_printing() {
            ui.invoke_trigger_backend_message(
                MSG_WARN,
                "Still printing the previous batch.".into(),
            );
            return;
        }

        if printer_name.is_empty() {
            ui.invoke_trigger_backend_message(
                MSG_WARN,
                "Cannot print: No printer selected in the UI.".into()
            );
            return;
        }

        let files = FileDialog::new()
            .set_title("Choose files to print")
            .add_filter("PDF Files", &["pdf", "PDF"])
            .add_filter("All Files", &["*"])
            .pick_files();

        let Some(paths) = files else { return };

        let job = PrintJob {
            printer: printer_name.to_string(),
            landscape: ui.get_print_orientation() == PrintOrientation::Landscape,
            files: paths
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
        };

        ui.set_printing(true);
        ui.invoke_trigger_backend_message(
            MSG_INFO,
            format!("Printing {} file(s) to {}...", job.files.len(), job.printer).into(),
        );

        // The print job itself runs in a separate process; this thread only
        // waits for it, so the window stays responsive.
        let weak = ui.as_weak();
        std::thread::spawn(move || {
            let result = run_in_worker(&job);
            let _ = weak.upgrade_in_event_loop(move |ui| {
                ui.set_printing(false);
                let (kind, message) = describe_result(&job, result);
                ui.invoke_trigger_backend_message(kind, message.into());
            });
        });
    });
}

/// Message type and text for a finished job.
fn describe_result(job: &PrintJob, result: Result<PrintReport, String>) -> (i32, String) {
    let report = match result {
        Ok(report) => report,
        Err(e) => return (MSG_ERROR, e),
    };
    if let Some(error) = report.error {
        return (MSG_ERROR, error);
    }
    if report.failed.is_empty() {
        return (
            MSG_INFO,
            format!("Successfully queued: {} file(s) to {}", report.printed, job.printer),
        );
    }

    const SHOWN: usize = 3;
    let mut names: Vec<String> = report
        .failed
        .iter()
        .take(SHOWN)
        .map(|f| {
            Path::new(&f.file)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| f.file.clone())
        })
        .collect();
    if report.failed.len() > SHOWN {
        names.push(format!("and {} more", report.failed.len() - SHOWN));
    }
    for failed in &report.failed {
        eprintln!("Failed to print {}: {}", failed.file, failed.error);
    }

    (
        MSG_WARN,
        format!(
            "{} of {} file(s) failed to print: {}",
            report.failed.len(),
            job.files.len(),
            names.join(", ")
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch_printing::worker::FailedFile;

    fn job(n: usize) -> PrintJob {
        PrintJob {
            printer: "HP".into(),
            landscape: false,
            files: (1..=n).map(|i| format!(r"D:\Draw\Reper {i}.pdf")).collect(),
        }
    }

    fn failed(name: &str) -> FailedFile {
        FailedFile { file: format!(r"D:\Draw\{name}"), error: "E".into() }
    }

    #[test]
    fn all_printed() {
        let report = PrintReport { printed: 4, ..PrintReport::default() };
        assert_eq!(
            describe_result(&job(4), Ok(report)),
            (MSG_INFO, "Successfully queued: 4 file(s) to HP".into())
        );
    }

    #[test]
    fn some_failed_lists_the_first_three_names() {
        let report = PrintReport {
            printed: 1,
            failed: ["a.pdf", "b.pdf", "c.pdf", "d.pdf", "e.pdf"].map(failed).to_vec(),
            error: None,
        };
        let (kind, text) = describe_result(&job(6), Ok(report));
        assert_eq!(kind, MSG_WARN);
        // Path::file_name only splits on '\' on Windows
        if cfg!(windows) {
            assert_eq!(text, "5 of 6 file(s) failed to print: a.pdf, b.pdf, c.pdf, and 2 more");
        } else {
            assert!(text.starts_with("5 of 6 file(s) failed to print: ") && text.ends_with(", and 2 more"));
        }
    }

    #[test]
    fn job_and_worker_errors_are_errors() {
        let report = PrintReport::job_error("Printer 'HP' not found in Windows devices.");
        assert_eq!(describe_result(&job(1), Ok(report)).0, MSG_ERROR);
        assert_eq!(describe_result(&job(1), Err("crashed".into())), (MSG_ERROR, "crashed".into()));
    }
}
