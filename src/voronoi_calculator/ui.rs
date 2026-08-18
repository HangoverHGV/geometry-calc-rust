use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use crate::AppWindow;
use std::rc::Rc;
use crate::voronoi_calculator::{voronoi_calculator, VoronoiParameters};

pub fn setup_callbacks(ui: &AppWindow) {
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

            let width = width_str.parse::<f64>().unwrap_or(0.0);
            let height = height_str.parse::<f64>().unwrap_or(0.0);

            let params = VoronoiParameters {
                width,
                height,
                seed,
                space,
                min_edge,
            };


            let raw_paths = voronoi_calculator(params);

            let shared_paths: Vec<SharedString> = raw_paths
                .into_iter()
                .map(SharedString::from)
                .collect();

            let model = Rc::new(VecModel::from(shared_paths));
            ui.set_voronoi_cells(ModelRc::from(model));
        },
    );
}