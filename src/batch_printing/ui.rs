use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use crate::AppWindow;
use crate::batch_printing::{get_all_printers, print_files};
use crate::PrinterItem;
use std::rc::Rc;
use rfd::FileDialog;

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

pub fn open_file_dialog(ui: &AppWindow){
    let ui_handle = ui.as_weak();

    ui.on_open_file_dialog(move |printer_name|{
        if printer_name.is_empty() {
            eprintln!("Cannot print: No printer selected in the UI.");
            return;
        }

        // 2. Open the file dialog
        let files = FileDialog::new()
            .set_title("Choose files to print")
            .pick_files();

        // 3. If the user selected files (didn't click cancel)
        if let Some(paths) = files {
            // Convert Vec<PathBuf> into Vec<String>
            let file_strings: Vec<String> = paths
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();

            // 4. Send directly to your print function!
            println!("Sending {} files to printer '{}'", file_strings.len(), printer_name);
            print_files(&printer_name, file_strings);
        }
    });
}


