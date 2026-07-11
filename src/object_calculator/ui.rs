use slint::ComponentHandle;
use crate::AppWindow; // Access AppWindow from the root
use super::calculator_logic::calculate_metrics;

pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_calculate_distance(move |total_str, thickness_str, number_of_elements_str| {
        let ui = ui_handle.unwrap();

        let total_dist: f64 = total_str.parse().unwrap_or(0.0);
        let thickness: f64 = thickness_str.parse().unwrap_or(0.0);
        let number_of_elements: i32 = number_of_elements_str.parse().unwrap_or(0);

        let (gap_dist, center_dist) = calculate_metrics(total_dist, thickness, number_of_elements);

        ui.set_computed_distance(format!("{:.2}", gap_dist).into());
        ui.set_computed_center_distance(format!("{:.2}", center_dist).into());
    });
}