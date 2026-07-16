use std::str::FromStr;
use slint::{SharedString, ModelRc, VecModel, ComponentHandle};
use crate::AppWindow;
use strum::{EnumIter, IntoEnumIterator};
use crate::force_calculator::force_calculator::{calculate_newton, DEFAULT_GRAVITY};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum AreaUnits{
    SquareMeter,
    SquareCentimeter,
    SquareMillimeter,
    SquareKilometer,
}

impl AreaUnits{
    pub fn get_options_model() -> Vec<SharedString>{
        Self::iter().map(|unit| {unit.to_shared_string()}).collect()
    }

    pub fn to_shared_string(&self) -> SharedString {
        match *self {
            AreaUnits::SquareMeter => SharedString::from("m²"),
            AreaUnits::SquareCentimeter => SharedString::from("cm²"),
            AreaUnits::SquareMillimeter => SharedString::from("mm²"),
            AreaUnits::SquareKilometer => SharedString::from("km²"),
        }
    }

    pub fn convert_to_mm2(&self, val: f64) -> f64{
        match *self {
            AreaUnits::SquareMeter => val * 1_000_000.0,
            AreaUnits::SquareCentimeter => val * 100.0,
            AreaUnits::SquareMillimeter => val,
            AreaUnits::SquareKilometer => val * 1_000_000_000_000.0,
        }
    }

}

impl FromStr for AreaUnits{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::iter().find(|unit| unit.to_shared_string() == s).ok_or(())
    }
}

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

impl FromStr for ForceUnits{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::iter().find(|variant| variant.to_shared_string() == s).ok_or(())
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

impl FromStr for MassUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::iter().find(|variant| variant.to_shared_string() == s).ok_or(())
    }
}

pub fn setup_ui_dropdowns(ui: &AppWindow){
    let force_options = ForceUnits::get_options_model();
    let force_model = ModelRc::new(VecModel::from(force_options));

    ui.set_force_unit_options(force_model);

    let mass_options = MassUnits::get_options_model();
    let mass_model = ModelRc::new(VecModel::from(mass_options));

    ui.set_mass_unit_options(mass_model);

    let area_option = AreaUnits::get_options_model();
    let area_model = ModelRc::new(VecModel::from(area_option));

    ui.set_area_unit_options(area_model);
}

pub fn setup_callback(ui: &AppWindow){
    let ui_handle = ui.as_weak();

    ui.on_calculate_force(move |mass_str, mass_unit_str, export_unit_str, area_str, area_unis_str| {
        let ui = ui_handle.unwrap();

        let mass: f64 = mass_str.parse().unwrap_or(0.0);
        let mass_unit: MassUnits = mass_unit_str.parse().unwrap_or(MassUnits::Kilogram);
        let export_unit: ForceUnits = export_unit_str.parse().unwrap_or(ForceUnits::Newtons);
        let area: Option<f64> = if area_str.trim().is_empty(){
            None
        } else {
            Some(area_str.parse().unwrap_or(0.0))
        };

        let area_units: AreaUnits = area_unis_str.parse().unwrap_or(AreaUnits::SquareMeter);

        let (force, mpa_val) = calculate_newton(mass, mass_unit, export_unit, area, area_units);

        ui.set_computed_force(format!("{:.2}", force).into());
        ui.set_computed_mpa(format!("{:.9}", mpa_val).into());
    });




}
