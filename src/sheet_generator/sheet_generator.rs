use rust_xlsxwriter::{Format, Workbook};
use std::cmp::Ordering;
use std::error::Error;
use std::path::{Path, PathBuf};
const HEADERS: [&str; 6] = [
    "Nr.Crt.",
    "Reper",
    "Grosime",
    "Material",
    "Cantitate",
    "Observatii",
];
const FONT_SIZE: f64 = 16.0;
const ROW_HEIGHT: f64 = 21.0; // points, fits a 16 pt font
/// Only these count as drawings (skips desktop.ini, Thumbs.db, lock files, ...)
const DRAWING_EXTENSIONS: &[&str] = &["dxf", "dwg"];

pub fn write_parts_list(draw_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = draw_dir
        .parent()
        .ok_or("The chosen folder has no parent folder")?;
    let project_name = project_dir
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Could not read the project folder name")?;

    let names = drawing_names(draw_dir)?;
    if names.is_empty() {
        return Err("No drawings found in the chosen folder".into());
    }

    let header_fmt = Format::new().set_font_size(FONT_SIZE).set_bold();
    let cell_fmt = Format::new().set_font_size(FONT_SIZE);

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    let _ = ws.set_name(project_name); // stays "Sheet1" if not a valid sheet name

    // Header row
    ws.set_row_height(0, ROW_HEIGHT)?;
    for (col, title) in HEADERS.iter().enumerate() {
        ws.write_string_with_format(0, col as u16, *title, &header_fmt)?;
    }

    // One row per drawing, in order
    for (i, name) in names.iter().enumerate() {
        let row = (i + 1) as u32;
        ws.set_row_height(row, ROW_HEIGHT)?;
        ws.write_number_with_format(row, 0, (i + 1) as f64, &cell_fmt)?;
        ws.write_string_with_format(row, 1, name, &cell_fmt)?;
    }

    // Widths sized for 16 pt; empty columns also get the 16 pt format
    // so anything typed in later matches
    for (col, title) in HEADERS.iter().enumerate() {
        let longest = if col == 1 {
            names.iter().map(|n| n.chars().count()).max().unwrap_or(0)
        } else {
            0
        }
        .max(title.chars().count());

        ws.set_column_width(col as u16, longest as f64 * FONT_SIZE / 11.0 + 2.0)?;
        ws.set_column_format(col as u16, &cell_fmt)?;
    }

    let out = draw_dir.join(format!("{project_name}.xlsx"));
    wb.save(&out)?;
    Ok(out)
}

fn drawing_names(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut names: Vec<String> = std::fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            path.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| DRAWING_EXTENSIONS.iter().any(|d| e.eq_ignore_ascii_case(d)))
        })
        .filter_map(|path| path.file_stem()?.to_str().map(str::to_owned))
        .collect();
    names.sort_by(|a, b| natural_cmp(a, b));
    names.dedup();
    Ok(names)
}

fn natural_cmp(mut a: &str, mut b: &str) -> Ordering {
    loop {
        match (a.is_empty(), b.is_empty()) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }
        let a_digit = a.as_bytes()[0].is_ascii_digit();
        let b_digit = b.as_bytes()[0].is_ascii_digit();
        let a_end = a
            .find(|c: char| c.is_ascii_digit() != a_digit)
            .unwrap_or(a.len());
        let b_end = b
            .find(|c: char| c.is_ascii_digit() != b_digit)
            .unwrap_or(b.len());
        let (a_chunk, b_chunk) = (&a[..a_end], &b[..b_end]);

        let ord = if a_digit && b_digit {
            let (an, bn) = (
                a_chunk.trim_start_matches('0'),
                b_chunk.trim_start_matches('0'),
            );
            an.len().cmp(&bn.len()).then_with(|| an.cmp(bn))
        } else {
            a_chunk.to_lowercase().cmp(&b_chunk.to_lowercase())
        };
        if ord != Ordering::Equal {
            return ord;
        }
        a = &a[a_end..];
        b = &b[b_end..];
    }
}
