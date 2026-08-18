use crate::voronoi_calculator::{voronoi_calculator, voronoi_polygons, VoronoiParameters, write_dxf};
use crate::AppWindow;
use rfd::FileDialog;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;

pub fn setup_callbacks(ui: &AppWindow) {
    setup_calculate(ui);
    setup_export_dxf(ui);
}

fn setup_calculate(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_calculate_voronoi(
        move |width_str: SharedString,
              height_str: SharedString,
              seed: f32,
              space: f32,
              min_edge: f32| {
            let ui = match ui_handle.upgrade() {
                Some(app) => app,
                None => return,
            };

            let params = VoronoiParameters {
                width: width_str.parse::<f64>().unwrap_or(0.0),
                height: height_str.parse::<f64>().unwrap_or(0.0),
                seed,
                space,
                min_edge,
            };

            let raw_paths = voronoi_calculator(params);

            let shared_paths: Vec<SharedString> =
                raw_paths.into_iter().map(SharedString::from).collect();

            let model = Rc::new(VecModel::from(shared_paths));
            ui.set_voronoi_cells(ModelRc::from(model));
        },
    );
}

fn setup_export_dxf(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_export_dxf(
        move |width_str: SharedString,
              height_str: SharedString,
              seed: f32,
              space: f32,
              min_edge: f32| {
            let ui = match ui_handle.upgrade() {
                Some(app) => app,
                None => return,
            };

            let width = width_str.parse::<f64>().unwrap_or(0.0);
            let height = height_str.parse::<f64>().unwrap_or(0.0);

            if width <= 0.0 || height <= 0.0 {
                ui.invoke_trigger_backend_message(
                    1,
                    "Cannot export: enter a valid width and height first.".into(),
                );
                return;
            }

            // Recomputed from the same parameters that drive the preview, so
            // the exported file always matches what's on screen without
            // needing to cache geometry between callbacks.
            let params = VoronoiParameters {
                width,
                height,
                seed,
                space,
                min_edge,
            };

            let polygons = voronoi_polygons(&params);

            if polygons.is_empty() {
                ui.invoke_trigger_backend_message(
                    1,
                    "Cannot export: no cells were generated. Try lowering 'Min cell edges' or 'Space between cells'.".into(),
                );
                return;
            }

            let chosen = FileDialog::new()
                .set_title("Export Voronoi pattern as DXF")
                .set_file_name("voronoi.dxf")
                .add_filter("DXF Files", &["dxf", "DXF"])
                .add_filter("All Files", &["*"])
                .save_file();

            // User cancelled the dialog — stay silent, that's not an error.
            let Some(mut path) = chosen else {
                return;
            };

            // Some platforms/filters don't append the extension for you.
            if path.extension().is_none() {
                path.set_extension("dxf");
            }

            match write_dxf(&path, &polygons, width, height) {
                Ok(()) => {
                    ui.invoke_trigger_backend_message(
                        0,
                        format!(
                            "Exported {} cell(s) to {}",
                            polygons.len(),
                            path.to_string_lossy()
                        )
                            .into(),
                    );
                }
                Err(e) => {
                    ui.invoke_trigger_backend_message(
                        1,
                        format!("Failed to write DXF: {}", e).into(),
                    );
                }
            }
        },
    );
}
