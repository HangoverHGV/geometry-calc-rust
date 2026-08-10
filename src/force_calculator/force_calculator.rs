use crate::force_calculator::ui::{ForceUnits, MassUnits, AreaUnits};
pub const DEFAULT_GRAVITY: f64 = 9.80665;

pub fn calculate_newton(
    mass: f64,
    mass_unit: MassUnits,
    export_unit: ForceUnits,
    area: Option<f64>,
    area_units: AreaUnits
) -> Result<Option<(f64, f64)>, String> {

    let eps = 1e-6;

    // If mass is missing, zero, or negative, quietly wait for more input.
    if mass <= eps {
        return Ok(None);
    }

    let mass_kg = mass_unit.to_kg(mass);
    // Assuming you calculate weight force from mass using Standard Gravity
    let force_n = ForceUnits::KilogramForce.to_newtons(mass_kg);

    let exported_force = export_unit.convert_from_newtons(force_n);

    // Validate Area and Calculate Pressure (MPa)
    let mpa = match area {
        Some(raw_area) => {
            if raw_area < 0.0 {
                return Err("Area cannot be negative.".into());
            } else if raw_area <= eps {
                // Catch Division by Zero!
                return Err("Area cannot be zero when calculating pressure.".into());
            } else {
                let area_mm2 = area_units.convert_to_mm2(raw_area);
                force_n / area_mm2
            }
        }
        None => 0.0, // If no area provided, we just don't calculate MPa
    };

    Ok(Some((exported_force, mpa)))
}