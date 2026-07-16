use crate::force_calculator::ui::{ForceUnits, MassUnits, AreaUnits};
pub const DEFAULT_GRAVITY: f64 = 9.80665;

pub fn calculate_newton(mass: f64, mass_unit: MassUnits, export_unit: ForceUnits, area: Option<f64>, area_units: AreaUnits) -> (f64, f64){
    let mass_kg = mass_unit.to_kg(mass);
    let force_n = ForceUnits::KilogramForce.to_newtons(mass_kg);

    let exported_force = export_unit.convert_from_newtons(force_n);

    let mpa = if let Some(raw_area) = area {
        let area_mm2 = area_units.convert_to_mm2(raw_area);
        if area_mm2 > 0.0 {
            force_n / area_mm2
        }else {
            0.0
        }
    }else {
        0.0
    };

    (exported_force, mpa)

}