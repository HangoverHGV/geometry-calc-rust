use crate::AppWindow;
use rfd::FileDialog;
use crate::sheet_generator::write_parts_list;

pub fn open_directory_dialog(ui: &AppWindow) {
    ui.on_open_directory(move || {
        let Some(draw_dir) = FileDialog::new()
            .set_title("Choose the Draw folder")
            .pick_folder()
        else {
            return; // user cancelled
        };

        match write_parts_list(&draw_dir) {
            Ok(path) => println!("Created {}", path.display()),
            Err(e) => eprintln!("Export failed: {e}"),
        }
    });
}
