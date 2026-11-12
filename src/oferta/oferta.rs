//! Data model for the `oferta-typst` price offer template.
//!
//! The structs serialize to exactly the shape `oferta-typst/oferta.typ`
//! reads (see `oferta-typst/README.md`), so a saved file can be compiled
//! with `typst compile --font-path fonts --input file=<name>.json oferta.typ`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::path::Path;

/// VAT rate the template uses when `cota_tva` is missing.
pub const DEFAULT_COTA_TVA: f64 = 21.0;

/// Whole offer — the root of `oferta.json`.
///
/// Required keys (`data_oferta`, `obiect_titlu`, `obiect_descriere`,
/// `client.nume`, `items`) are always written, even when empty, because the
/// template reads them without a default. Optional keys are left out when
/// empty so the template falls back to its own defaults.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Oferta {
    pub data_oferta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valabilitate: Option<String>,
    pub obiect_titlu: String,
    pub obiect_descriere: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referinta: Option<String>,
    pub client: Client,
    pub items: Vec<Item>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "ser_opt_num")]
    pub cota_tva: Option<f64>,
    /// A number (`15`) or a range (`"10-15"`); kept as text here.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "ser_termen",
        deserialize_with = "de_termen"
    )]
    pub termen_zile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditii_plata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livrare: Option<String>,
}

/// `client` — the beneficiary block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Client {
    pub nume: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

/// One row of `items[]`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub denumire: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriere: Option<String>,
    pub um: String,
    #[serde(serialize_with = "ser_num")]
    pub cant: f64,
    /// Unit price without VAT, in RON.
    #[serde(serialize_with = "ser_num")]
    pub pret: f64,
}

/// Same totals the template prints, with the same rounding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Totals {
    pub fara_tva: f64,
    pub tva: f64,
    pub de_plata: f64,
}

/// A fresh offer: today's date, the default VAT rate, no rows.
pub fn new_oferta() -> Oferta {
    Oferta {
        data_oferta: today(),
        cota_tva: Some(DEFAULT_COTA_TVA),
        ..Oferta::default()
    }
}

/// Today's date as the template expects it, e.g. `12.11.2026`.
pub fn today() -> String {
    chrono::Local::now().format("%d.%m.%Y").to_string()
}

pub fn load(path: &Path) -> Result<Oferta, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("Could not read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("Not a valid offer file: {e}"))
}

pub fn save(path: &Path, oferta: &Oferta) -> Result<(), String> {
    let mut text =
        serde_json::to_string_pretty(oferta).map_err(|e| format!("Could not encode offer: {e}"))?;
    text.push('\n');
    std::fs::write(path, text).map_err(|e| format!("Could not write {}: {e}", path.display()))
}

/// Value of one row: `round(cant × pret, 2)`, like `valoare()` in the template.
pub fn line_value(cant: f64, pret: f64) -> f64 {
    round2(cant * pret)
}

pub fn totals(line_values: impl IntoIterator<Item = f64>, cota_tva: f64) -> Totals {
    let fara_tva: f64 = line_values.into_iter().sum();
    let tva = round2(fara_tva * cota_tva / 100.0);
    Totals { fara_tva, tva, de_plata: fara_tva + tva }
}

/// Names of required values that are still empty. The file can still be
/// saved, but the PDF would have blanks in these places.
pub fn missing_fields(oferta: &Oferta) -> Vec<String> {
    let mut missing = Vec::new();
    let mut need = |empty: bool, name: &str| {
        if empty {
            missing.push(name.to_string());
        }
    };
    need(oferta.data_oferta.trim().is_empty(), "date");
    need(oferta.obiect_titlu.trim().is_empty(), "title");
    need(oferta.obiect_descriere.trim().is_empty(), "description");
    need(oferta.client.nume.trim().is_empty(), "client name");
    need(oferta.items.is_empty(), "items");
    for (i, item) in oferta.items.iter().enumerate() {
        need(item.denumire.trim().is_empty(), &format!("row {} name", i + 1));
        need(item.um.trim().is_empty(), &format!("row {} U.M.", i + 1));
    }
    missing
}

/// Parses a number typed the Romanian or the English way.
///
/// `7,5` and `7.5` are both 7.5. When both separators appear, the last one is
/// the decimal mark (`1.234,5`, `1,234.5`). Several dots alone are thousands
/// separators (`1.234.567`), but a single dot is always a decimal point, so
/// `1.500` is 1.5. Spaces are ignored.
pub fn parse_number(input: &str) -> Option<f64> {
    let s: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if s.is_empty() {
        return None;
    }
    let normalized = match (s.rfind(','), s.rfind('.')) {
        (Some(comma), Some(dot)) if comma > dot => s.replace('.', "").replace(',', "."),
        (Some(_), Some(_)) => s.replace(',', ""),
        (Some(_), None) if s.matches(',').count() == 1 => s.replace(',', "."),
        (Some(_), None) => return None,
        (None, Some(_)) if s.matches('.').count() > 1 => s.replace('.', ""),
        _ => s,
    };
    normalized.parse::<f64>().ok().filter(|v| v.is_finite())
}

/// A number for editing: `12000`, `7,5`. No thousands separators, so the
/// text parses back to the same value.
pub fn format_number(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        x.to_string().replace('.', ",")
    }
}

/// Money the way the template prints it: `12000` → `12.000,00`.
pub fn lei(x: f64) -> String {
    let cents = (x * 100.0).round() as i64;
    let digits = (cents.unsigned_abs() / 100).to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            grouped.push('.');
        }
        grouped.push(ch);
    }
    let sign = if cents < 0 { "-" } else { "" };
    format!("{sign}{grouped},{:02}", cents.unsigned_abs() % 100)
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

// ---------- serde helpers ----------

/// Whole numbers are written as integers (`1`, not `1.0`) to match the
/// hand-written example file.
fn ser_num<S: Serializer>(x: &f64, s: S) -> Result<S::Ok, S::Error> {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        s.serialize_i64(*x as i64)
    } else {
        s.serialize_f64(*x)
    }
}

fn ser_opt_num<S: Serializer>(x: &Option<f64>, s: S) -> Result<S::Ok, S::Error> {
    match x {
        Some(v) => ser_num(v, s),
        None => s.serialize_none(),
    }
}

/// `"15"` → `15`, `"10-15"` → `"10-15"`.
fn ser_termen<S: Serializer>(x: &Option<String>, s: S) -> Result<S::Ok, S::Error> {
    match x {
        Some(t) => match t.trim().parse::<i64>() {
            Ok(n) => s.serialize_i64(n),
            Err(_) => s.serialize_str(t),
        },
        None => s.serialize_none(),
    }
}

fn de_termen<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Null => None,
        Value::String(s) => Some(s),
        Value::Number(n) => Some(n.as_f64().map(format_number).unwrap_or_else(|| n.to_string())),
        other => Some(other.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../oferta-typst/oferta.json");

    #[test]
    fn example_file_round_trips_unchanged() {
        let oferta: Oferta = serde_json::from_str(EXAMPLE).unwrap();
        let written: Value = serde_json::to_value(&oferta).unwrap();
        let original: Value = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(written, original);
    }

    #[test]
    fn example_file_maps_every_field() {
        let o: Oferta = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(o.data_oferta, "12.11.2026");
        assert_eq!(o.client.cui.as_deref(), Some("RO12345678"));
        assert_eq!(o.items.len(), 1);
        assert_eq!(o.items[0].pret, 12000.0);
        assert_eq!(o.cota_tva, Some(21.0));
        assert_eq!(o.termen_zile.as_deref(), Some("15"));
        assert!(missing_fields(&o).is_empty());
    }

    #[test]
    fn empty_optionals_are_left_out_and_required_keys_kept() {
        let json = serde_json::to_value(Oferta::default()).unwrap();
        let mut keys: Vec<&str> = json.as_object().unwrap().keys().map(String::as_str).collect();
        keys.sort();
        assert_eq!(
            keys,
            ["client", "data_oferta", "items", "obiect_descriere", "obiect_titlu"]
        );
        assert_eq!(json["client"], serde_json::json!({ "nume": "" }));
    }

    #[test]
    fn termen_range_stays_text() {
        let o = Oferta { termen_zile: Some("10-15".into()), ..Oferta::default() };
        let json = serde_json::to_value(&o).unwrap();
        assert_eq!(json["termen_zile"], "10-15");
        let back: Oferta = serde_json::from_value(json).unwrap();
        assert_eq!(back.termen_zile.as_deref(), Some("10-15"));
    }

    #[test]
    fn fractional_quantity_stays_fractional() {
        let item = Item { cant: 7.5, pret: 120.0, ..Item::default() };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["cant"], 7.5);
        assert_eq!(json["pret"], 120);
    }

    #[test]
    fn parses_romanian_and_english_numbers() {
        assert_eq!(parse_number("7,5"), Some(7.5));
        assert_eq!(parse_number("7.5"), Some(7.5));
        assert_eq!(parse_number("12000"), Some(12000.0));
        assert_eq!(parse_number("12 000"), Some(12000.0));
        assert_eq!(parse_number("1.234,56"), Some(1234.56));
        assert_eq!(parse_number("1,234.56"), Some(1234.56));
        assert_eq!(parse_number("1.234.567"), Some(1234567.0));
        assert_eq!(parse_number("1.500"), Some(1.5));
        assert_eq!(parse_number("-5"), Some(-5.0));
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("1,2,3"), None);
        assert_eq!(parse_number("abc"), None);
        assert_eq!(parse_number("inf"), None);
    }

    #[test]
    fn formats_like_the_template() {
        assert_eq!(lei(12000.0), "12.000,00");
        assert_eq!(lei(1234567.891), "1.234.567,89");
        assert_eq!(lei(999.0), "999,00");
        assert_eq!(lei(0.0), "0,00");
        assert_eq!(lei(-1500.5), "-1.500,50");
        assert_eq!(format_number(12000.0), "12000");
        assert_eq!(format_number(7.5), "7,5");
    }

    #[test]
    fn totals_match_the_example_pdf() {
        let t = totals([line_value(1.0, 12000.0)], DEFAULT_COTA_TVA);
        assert_eq!(t, Totals { fara_tva: 12000.0, tva: 2520.0, de_plata: 14520.0 });
    }

    #[test]
    fn missing_fields_are_listed() {
        let o = Oferta {
            items: vec![Item { cant: 1.0, pret: 1.0, ..Item::default() }],
            ..new_oferta()
        };
        assert_eq!(
            missing_fields(&o),
            ["title", "description", "client name", "row 1 name", "row 1 U.M."]
        );
    }
}
