
const DEFAULT_GRAVITY: f64 = 9.80665;

#[derive(Debug, Clone, Copy)]
enum ForceUnits{
    Newtons,
    Kilonewton,
    KilogramForce,
    MetricTonsForce,
}

impl ForceUnits{
    pub fn to_newtons(&self, val: f64) -> f64 {
        match *self {
            ForceUnits::Newtons => val,
            ForceUnits::Kilonewton => val * 1000.0,
            ForceUnits::KilogramForce => val * DEFAULT_GRAVITY,
            ForceUnits::MetricTonsForce => val * (DEFAULT_GRAVITY * 1000.0),
        }
    }

    pub fn convert_from_newtons(&self, newtons: f64) -> f64 {
        match *self {
            ForceUnits::Newtons => newtons,
            ForceUnits::Kilonewton => newtons / 1000.0,
            ForceUnits::KilogramForce => newtons / DEFAULT_GRAVITY,
            ForceUnits::MetricTonsForce => newtons / (DEFAULT_GRAVITY * 1000.0),
        }
    }
}

impl MassUnits {
    pub fn to_kg(&self, val: f64) -> f64{
        match *self {
            MassUnits::Kilogram => val,
            MassUnits::MetricTon => val * 1000.0,
        }
    }

}

enum MassUnits{
    Kilogram,
    MetricTon,
}

pub fn calculate_newton(mass: f64, mass_unit: MassUnits, export_unit: ForceUnits, area: Option<f64>) -> (f64, f64){
    let mass_kg = mass_unit.to_kg(mass);
    let force_n = ForceUnits::KilogramForce.to_newtons(mass_kg);

    let exported_force = export_unit.convert_from_newtons(force_n);

    let mpa = if let Some(area_mm2) = area {
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