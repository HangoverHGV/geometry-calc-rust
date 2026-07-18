use slint::ComponentHandle;
use crate::AppWindow;
use super::right_triangle::calculate_all_triangle_elements;
use super::isosceles_triangle::calculate_all_isosceles_elements;

pub fn setup_callbacks(ui: &AppWindow){
    let ui_handle = ui.as_weak();

    let ui_handle_right = ui_handle.clone();
    ui.on_calculate_triangle(move |h_str, c1_str, c2_str, abc_str, acb_str| {
        let ui = ui_handle_right.unwrap();

        let h: f64 = h_str.parse().unwrap_or(0.0);
        let c1: f64 = c1_str.parse().unwrap_or(0.0);
        let c2: f64 = c2_str.parse().unwrap_or(0.0);
        let abc: f64 = abc_str.parse().unwrap_or(0.0);
        let acb: f64 = acb_str.parse().unwrap_or(0.0);

        if let Some((res_h, res_c1, res_c2, res_abc, res_acb)) = calculate_all_triangle_elements(h, c1, c2, abc, acb) {
            ui.set_computed_hypotenuse_text(format!("{:.2}", res_h).into());
            ui.set_computed_cathetus1_text(format!("{:.2}", res_c1).into());
            ui.set_computed_cathetus2_text(format!("{:.2}", res_c2).into());
            ui.set_computed_abc_angle(format!("{:.2}", res_abc).into());
            ui.set_computed_acb_angle(format!("{:.2}", res_acb).into());
        } else {
            // 👇 Crucial: Reset result labels back to blank/empty when calculations are invalid or empty
            ui.set_computed_hypotenuse_text("".into());
            ui.set_computed_cathetus1_text("".into());
            ui.set_computed_cathetus2_text("".into());
            ui.set_computed_abc_angle("".into());
            ui.set_computed_acb_angle("".into());
        }
    });

    let ui_handle_iso = ui_handle.clone();
    ui.on_calculate_isosceles_tringle(move |ab_str, bc_str, abc_str, bac_str| {
        let ui = ui_handle_iso.unwrap();

        let ab: f64 = ab_str.parse().unwrap_or(0.0);
        let bc: f64 = bc_str.parse().unwrap_or(0.0);
        let abc: f64 = abc_str.parse().unwrap_or(0.0);
        let bac: f64 = bac_str.parse().unwrap_or(0.0);

        if let Some((res_ab, res_bc, res_abc, res_bac)) = calculate_all_isosceles_elements(ab, bc, abc, bac){
            ui.set_computed_ab_text(format!("{:.2}", res_ab).into());
            ui.set_computed_bc_text(format!("{:.2}", res_bc).into());
            ui.set_computed_abc_is_angle(format!("{:.2}", res_abc).into());
            ui.set_computed_bac_is_angle(format!("{:.2}", res_bac).into());
        }else {
            ui.set_computed_ab_text("".into());
            ui.set_computed_bc_text("".into());
            ui.set_computed_abc_is_angle("".into());
            ui.set_computed_bac_is_angle("".into());
        }
    });
}