use calamine::{open_workbook_auto, Reader, DataType};
use crate::BomRow;
use slint::SharedString;

pub fn parse_spreadsheet(path: &str) -> Result<Vec<BomRow>, String>{
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    if ext == "csv" {
        parse_csv(path)
    } else {
        parse_excel(path)
    }
}

fn parse_csv(path: &str) -> Result<Vec<BomRow>, String>{
    let mut rdr = csv::Reader::from_path(path)
        .map_err(|e| format!("Failed to open CSV: {}", e))?;
    let mut rows = Vec::new();

    for result in rdr.records(){
        let record = result.map_err(|e| format!("Failed to read CSV row: {}", e))?;
        rows.push(BomRow{
            name: SharedString::from(record.get(0).unwrap_or("")),
            quantity: SharedString::from(record.get(1).unwrap_or("")),
        });
    }

    Ok(rows)
}


fn parse_excel(path: &str) -> Result<Vec<BomRow>, String> {
    let mut workbook = open_workbook_auto(path)
        .map_err(|e| format!("Failed to open spreadsheet: {}", e))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "Spreadsheet has no sheets.".to_string())?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Failed to read sheet '{}': {}", sheet_name, e))?;

    let mut rows = Vec::new();

    for row in range.rows().skip(1) {
        // skip header row
        let name = row.get(0).map(cell_to_string).unwrap_or_default();
        let quantity = row.get(1).map(cell_to_string).unwrap_or_default();

        rows.push(BomRow {
            name: SharedString::from(name),
            quantity: SharedString::from(quantity),
        });
    }

    Ok(rows)
}

fn cell_to_string(cell: &impl DataType) -> String {
    if let Some(s) = cell.get_string() {
        s.to_string()
    } else if let Some(i) = cell.get_int() {
        i.to_string()
    } else if let Some(f) = cell.get_float() {
        f.to_string()
    } else if let Some(b) = cell.get_bool() {
        b.to_string()
    } else {
        String::new()
    }
}