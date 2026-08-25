use calamine::{open_workbook_auto, Data, DataType, Reader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct RawRow {
    base_name: String,
    piece_len_mm: f64,
    count: u32,
    mass_per_instance_kg: Option<f64>,
}

/// Final merged, per-part result — this is what feeds the Slint UI.
/// bars_6m / bars_12m = ceil(total_length_mm / bar_length_mm), independent
/// of each other — "how many of this size would it take to cover the
/// total length" — not a combined least-waste purchase plan.
#[derive(Debug, Clone, PartialEq)]
pub struct BomSummary {
    pub name: String,
    pub total_length_m: f64,
    pub total_mass_kg: f64,
    pub bars_6m: u32,
    pub bars_12m: u32,
    pub length_6m_total_m: f64,
    pub length_12m_total_m: f64,
    pub mass_incomplete: bool,
}

const STOCK_SMALL_MM: f64 = 6000.0;
const STOCK_LARGE_MM: f64 = 12000.0;

pub fn parse_spreadsheet(path: &str) -> Result<Vec<BomSummary>, String> {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    let raw_rows = if ext == "csv" { parse_csv(path)? } else { parse_excel(path)? };
    Ok(summarize(raw_rows))
}

fn base_name_and_length(part_no: &str) -> (String, f64) {
    static PATTERN: &str = r"^(.*)-\s*([\d.]+)$";
    let re = Regex::new(PATTERN).unwrap();
    if let Some(caps) = re.captures(part_no.trim()) {
        let base = caps[1].trim().to_string();
        let len: f64 = caps[2].parse().unwrap_or(0.0);
        (base, len)
    } else {
        (part_no.trim().to_string(), 0.0)
    }
}

fn parse_mm(s: &str) -> Option<f64> { s.replace("mm", "").trim().parse::<f64>().ok() }

fn parse_kg(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.eq_ignore_ascii_case("*varies*") { return None; }
    s.replace("kg", "").trim().parse::<f64>().ok()
}

fn parse_csv(path: &str) -> Result<Vec<RawRow>, String> {
    let mut rdr = csv::Reader::from_path(path).map_err(|e| format!("Failed to open CSV: {e}"))?;
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| format!("Failed to read CSV row: {e}"))?;
        push_row(&mut rows, record.get(1), record.get(4), record.get(5), record.get(8))?;
    }
    Ok(rows)
}

fn parse_excel(path: &str) -> Result<Vec<RawRow>, String> {
    let mut workbook = open_workbook_auto(path).map_err(|e| format!("Failed to open spreadsheet: {e}"))?;
    let sheet_name = workbook.sheet_names().first().cloned()
        .ok_or_else(|| "Spreadsheet has no sheets.".to_string())?;
    let range = workbook.worksheet_range(&sheet_name)
        .map_err(|e| format!("Failed to read sheet '{sheet_name}': {e}"))?;
    let mut rows = Vec::new();
    for row in range.rows().skip(1) {
        let part_no = row.get(1).map(cell_to_string).unwrap_or_default();
        let unit_qty = row.get(4).map(cell_to_string).unwrap_or_default();
        let qty = row.get(5).map(cell_to_string).unwrap_or_default();
        let mass = row.get(8).map(cell_to_string).unwrap_or_default();
        push_row(&mut rows, Some(part_no.as_str()), Some(unit_qty.as_str()), Some(qty.as_str()), Some(mass.as_str()))?;
    }
    Ok(rows)
}

fn push_row(
    rows: &mut Vec<RawRow>,
    part_no: Option<&str>,
    unit_qty: Option<&str>,
    qty: Option<&str>,
    mass: Option<&str>,
) -> Result<(), String> {
    let part_no = part_no.unwrap_or("").trim();
    let unit_qty = unit_qty.unwrap_or("").trim();
    let qty = qty.unwrap_or("").trim();
    let mass = mass.unwrap_or("").trim();
    if part_no.is_empty() || unit_qty.is_empty() { return Ok(()); }
    if unit_qty.eq_ignore_ascii_case("each") { return Ok(()); }
    let Some(unit_len) = parse_mm(unit_qty) else { return Ok(()) };
    let Some(total_len) = parse_mm(qty) else { return Ok(()) };
    if unit_len <= 0.0 { return Ok(()); }
    let count = (total_len / unit_len).round() as u32;
    let (base_name, piece_len) = base_name_and_length(part_no);
    let piece_len = if piece_len > 0.0 { piece_len } else { unit_len };
    let mass_per_instance = parse_kg(mass);
    rows.push(RawRow { base_name, piece_len_mm: piece_len, count, mass_per_instance_kg: mass_per_instance });
    Ok(())
}

fn cell_to_string(cell: &Data) -> String {
    if let Some(s) = cell.get_string() { s.to_string() }
    else if let Some(i) = cell.get_int() { i.to_string() }
    else if let Some(f) = cell.get_float() { f.to_string() }
    else if let Some(b) = cell.get_bool() { b.to_string() }
    else { String::new() }
}

fn summarize(raw_rows: Vec<RawRow>) -> Vec<BomSummary> {
    let mut groups: HashMap<String, Vec<RawRow>> = HashMap::new();
    for row in raw_rows {
        groups.entry(row.base_name.clone()).or_default().push(row);
    }

    let mut out: Vec<BomSummary> = groups.into_iter().map(|(name, items)| {
        let total_len_mm: f64 = items.iter().map(|it| it.piece_len_mm * it.count as f64).sum();
        let total_mass_kg: f64 = items.iter().map(|it| it.mass_per_instance_kg.unwrap_or(0.0) * it.count as f64).sum();
        let mass_incomplete = items.iter().any(|it| it.mass_per_instance_kg.is_none());

        let bars_6m = (total_len_mm / STOCK_SMALL_MM).ceil() as u32;
        let bars_12m = (total_len_mm / STOCK_LARGE_MM).ceil() as u32;

        BomSummary {
            name,
            total_length_m: total_len_mm / 1000.0,
            total_mass_kg,
            bars_6m,
            bars_12m,
            length_6m_total_m: bars_6m as f64 * (STOCK_SMALL_MM / 1000.0),
            length_12m_total_m: bars_12m as f64 * (STOCK_LARGE_MM / 1000.0),
            mass_incomplete,
        }
    }).collect();

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}