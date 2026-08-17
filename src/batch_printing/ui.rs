use slint::{ModelRc, SharedString, VecModel};
use crate::AppWindow;
use crate::batch_printing::get_all_printers;

pub fn setup_ui_printers(ui: &AppWindow) {
    let printer_names = match get_all_printers() {
        Ok(Some(names)) => names,
        Ok(None) => {
            // Handle the case where no printers were found
            Vec::new()
        }
        Err(e) => {
            ui.invoke_trigger_backend_message(2, "Failed to fetch printers".into());
            return;
        }
    };

    let shared_printer_names : Vec<SharedString> = printer_names.into_iter()
        .map(Into::into)
        .collect();

    let printer_model = ModelRc::new(VecModel::from(shared_printer_names));

    ui.set_printers_list(printer_model);

}