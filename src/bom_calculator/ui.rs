use slint::{ComponentHandle, ModelRc, VecModel};
use crate::AppWindow;
use crate::bom_calculator::parse_spreadsheet;
use rfd::FileDialog;
use std::rc::Rc;

pub fn upload_spreadsheet(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_upload_bom_spreadsheet(move || {
        let ui = ui_handle.unwrap();

        let file = FileDialog::new()
            .set_title("Choose a BOM spreadsheet")
            .add_filter("Spreadsheet Files", &["csv", "xls", "xlsx", "xlsm"])
            .add_filter("All Files", &["*"])
            .pick_file();

        let Some(path) = file else { return };

        let path_str = path.to_string_lossy().to_string();
        let file_name = path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();

        match parse_spreadsheet(&path_str) {
            Ok(rows) => {
                let slint_rows: Vec<crate::BomRow> = rows
                    .into_iter()
                    .map(|r| crate::BomRow {
                        name: r.name.into(),
                        total_length_m: format!("{:.3}", r.total_length_m).into(),
                        total_mass_kg: format!("{:.3}", r.total_mass_kg).into(),
                        bars_6m: r.bars_6m as i32,
                        bars_12m: r.bars_12m as i32,
                        length_6m_m: format!("{:.1} m", r.length_6m_total_m).into(),
                        length_12m_m: format!("{:.1} m", r.length_12m_total_m).into(),
                    })
                    .collect();

                let model = Rc::new(VecModel::from(slint_rows));
                ui.set_bom_rows(ModelRc::from(model));
                ui.set_bom_loaded_file_name(file_name.into());
            }
            Err(error_message) => {
                ui.invoke_trigger_backend_message(1, error_message.into());
            }
        }
    });
}