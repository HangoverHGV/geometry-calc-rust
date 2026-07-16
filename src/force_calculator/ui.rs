use slint::{SharedString, ModelRc, VecModel, ComponentHandle};
use crate::AppWindow;
use strum::{EnumIter, IntoEnumIterator};
use crate::force_calculator::force_calculator::DEFAULT_GRAVITY;


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ForceUnits{
    Newtons,
    Kilonewton,
    KilogramForce,
    MetricTonsForce,
}

impl ForceUnits{

    pub fn get_options_model() -> Vec<SharedString> {
        Self::iter().map(|unit| {unit.to_shared_string()}).collect()
    }

    #[allow(unused)]
    pub fn to_newtons(&self, val: f64) -> f64 {
        match *self {
            ForceUnits::Newtons => val,
            ForceUnits::Kilonewton => val * 1000.0,
            ForceUnits::KilogramForce => val *  DEFAULT_GRAVITY,
            ForceUnits::MetricTonsForce => val * ( DEFAULT_GRAVITY * 1000.0),
        }
    }

    pub fn convert_from_newtons(&self, newtons: f64) -> f64 {
        match *self {
            ForceUnits::Newtons => newtons,
            ForceUnits::Kilonewton => newtons / 1000.0,
            ForceUnits::KilogramForce => newtons /  DEFAULT_GRAVITY,
            ForceUnits::MetricTonsForce => newtons / ( DEFAULT_GRAVITY * 1000.0),
        }
    }

    pub fn to_shared_string(&self) -> SharedString {
        match *self {
            ForceUnits::Newtons => SharedString::from("N"),
            ForceUnits::Kilonewton => SharedString::from("kN"),
            ForceUnits::KilogramForce => SharedString::from("kgf"),
            ForceUnits::MetricTonsForce => SharedString::from("tf"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum MassUnits{
    Kilogram,
    MetricTon,
}
impl MassUnits {
    pub fn get_options_model() -> Vec<SharedString> {
        Self::iter().map(|unit| {unit.to_shared_string()}).collect()
    }

    #[allow(unused)]
    pub fn to_kg(&self, val: f64) -> f64{
        match *self {
            MassUnits::Kilogram => val,
            MassUnits::MetricTon => val * 1000.0,
        }
    }

    pub fn to_shared_string(&self) ->SharedString{
        match *self {
            MassUnits::Kilogram => SharedString::from("kg"),
            MassUnits::MetricTon => SharedString::from("t"),
        }
    }

}

pub fn setup_ui_dropdowns(ui: &AppWindow){
    let force_options = ForceUnits::get_options_model();
    let force_model = ModelRc::new(VecModel::from(force_options));

    ui.set_force_unit_options(force_model);

    let mass_options = MassUnits::get_options_model();
    let mass_model = ModelRc::new(VecModel::from(mass_options));

    ui.set_mass_unit_options(mass_model);
}

pub fn setup_callback(ui: &AppWindow){
    let ui_handler = ui.as_weak();
}
