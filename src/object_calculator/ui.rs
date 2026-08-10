use slint::ComponentHandle;
use crate::AppWindow; // Access AppWindow from the root
use super::calculator_logic::try_calculate;

pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_calculate_distance(move |total_str, thickness_str, number_of_elements_str| {
        let ui = ui_handle.unwrap();

        // 2. Match the Result from our helper function
        match try_calculate(total_str.as_str(), thickness_str.as_str(), number_of_elements_str.as_str()) {
            Ok((gap_dist, center_dist)) => {
                // Success: Update the UI with the numbers
                ui.set_computed_distance(format!("{:.2}", gap_dist).into());
                ui.set_computed_center_distance(format!("{:.2}", center_dist).into());
            }
            Err(error_message) => {
                ui.invoke_trigger_backend_message(2, error_message.into());
                ui.set_computed_distance("".into());
                ui.set_computed_center_distance("".into());
            }
        }
    });
}