use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use super::oferta::{self, Client, Item, Oferta};
use super::pdf;
use crate::{AppWindow, OfertaItem, OfertaState};

const MSG_INFO: i32 = 0;
const MSG_WARN: i32 = 1;
const MSG_ERROR: i32 = 2;

/// Rows of the items table. Rust owns the model; Slint writes edits into it.
type Rows = Rc<VecModel<OfertaItem>>;
/// The JSON file last opened or saved, so Save offers it again.
type CurrentFile = Rc<RefCell<Option<PathBuf>>>;

pub fn setup_callbacks(ui: &AppWindow) {
    let rows: Rows = Rc::new(VecModel::default());
    let current_file: CurrentFile = Rc::new(RefCell::new(None));

    let state = ui.global::<OfertaState>();
    state.set_items(ModelRc::from(rows.clone()));
    fill_form(ui, &rows, &oferta::new_oferta());

    let weak = ui.as_weak();
    let r = rows.clone();
    state.on_recalculate(move || {
        if let Some(ui) = weak.upgrade() {
            recalculate(&ui, &r);
        }
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    state.on_add_item(move || {
        r.push(blank_row());
        if let Some(ui) = weak.upgrade() {
            recalculate(&ui, &r);
        }
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    state.on_remove_item(move |index| {
        if let Ok(index) = usize::try_from(index)
            && index < r.row_count()
        {
            r.remove(index);
        }
        if r.row_count() == 0 {
            r.push(blank_row()); // always leave one row to type into
        }
        if let Some(ui) = weak.upgrade() {
            recalculate(&ui, &r);
        }
    });

    let weak = ui.as_weak();
    state.on_set_today(move || {
        if let Some(ui) = weak.upgrade() {
            ui.global::<OfertaState>().set_data_oferta(oferta::today().into());
        }
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    let file = current_file.clone();
    state.on_new_oferta(move || {
        let Some(ui) = weak.upgrade() else { return };
        fill_form(&ui, &r, &oferta::new_oferta());
        file.replace(None);
        ui.global::<OfertaState>().set_loaded_file(SharedString::new());
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    let file = current_file.clone();
    state.on_load_json(move || {
        let Some(ui) = weak.upgrade() else { return };

        let mut dialog = FileDialog::new()
            .set_title("Open offer data")
            .add_filter("Offer data (JSON)", &["json"])
            .add_filter("All files", &["*"]);
        if let Some(dir) = current_dir(&file) {
            dialog = dialog.set_directory(dir);
        }
        let Some(path) = dialog.pick_file() else { return };

        match oferta::load(&path) {
            Ok(data) => {
                fill_form(&ui, &r, &data);
                remember_file(&ui, &file, path);
            }
            Err(e) => ui.invoke_trigger_backend_message(MSG_ERROR, e.into()),
        }
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    let file = current_file.clone();
    state.on_save_json(move || {
        let Some(ui) = weak.upgrade() else { return };

        let data = match read_form(&ui, &r) {
            Ok(data) => data,
            Err(e) => {
                ui.invoke_trigger_backend_message(MSG_ERROR, e.into());
                return;
            }
        };

        let suggested = file
            .borrow()
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| default_file_name(&data, "json"));
        let mut dialog = FileDialog::new()
            .set_title("Save offer data")
            .add_filter("Offer data (JSON)", &["json"])
            .set_file_name(suggested);
        if let Some(dir) = current_dir(&file) {
            dialog = dialog.set_directory(dir);
        }
        let Some(mut path) = dialog.save_file() else { return };
        if path.extension().is_none() {
            path.set_extension("json");
        }

        if let Err(e) = oferta::save(&path, &data) {
            ui.invoke_trigger_backend_message(MSG_ERROR, e.into());
            return;
        }

        let name = file_name(&path);
        remember_file(&ui, &file, path);

        report_done(&ui, "Saved", &name, &oferta::missing_fields(&data));
    });

    let weak = ui.as_weak();
    let r = rows.clone();
    let file = current_file.clone();
    let last_pdf: CurrentFile = Rc::new(RefCell::new(None));
    state.on_generate_pdf(move || {
        let Some(ui) = weak.upgrade() else { return };
        if ui.global::<OfertaState>().get_generating_pdf() {
            return;
        }

        let data = match read_form(&ui, &r) {
            Ok(data) => data,
            Err(e) => {
                ui.invoke_trigger_backend_message(MSG_ERROR, e.into());
                return;
            }
        };

        // Suggest the PDF next to the JSON with the same name; otherwise
        // the folder of the last PDF and a name built from the form.
        let json = file.borrow().clone();
        let suggested = json
            .as_ref()
            .map(|p| file_name(&p.with_extension("pdf")))
            .unwrap_or_else(|| default_file_name(&data, "pdf"));
        let mut dialog = FileDialog::new()
            .set_title("Save offer PDF")
            .add_filter("PDF", &["pdf"])
            .set_file_name(suggested);
        if let Some(dir) = current_dir(&file).or_else(|| current_dir(&last_pdf)) {
            dialog = dialog.set_directory(dir);
        }
        let Some(mut path) = dialog.save_file() else { return };
        if path.extension().is_none() {
            path.set_extension("pdf");
        }
        last_pdf.replace(Some(path.clone()));

        // Typst takes a moment (longer in debug builds), so render off the
        // UI thread and report back through the event loop.
        ui.global::<OfertaState>().set_generating_pdf(true);
        let weak = ui.as_weak();
        std::thread::spawn(move || {
            let result = pdf::render(&data).and_then(|bytes| write_pdf(&path, &bytes));
            let missing = oferta::missing_fields(&data);
            let name = file_name(&path);
            let _ = weak.upgrade_in_event_loop(move |ui| {
                ui.global::<OfertaState>().set_generating_pdf(false);
                match result {
                    Ok(()) => report_done(&ui, "Created", &name, &missing),
                    Err(e) => ui.invoke_trigger_backend_message(MSG_ERROR, e.into()),
                }
            });
        });
    });
}

/// "Saved x.json" / "Created x.pdf", or a warning naming the required
/// fields that are still empty.
fn report_done(ui: &AppWindow, verb: &str, name: &str, missing: &[String]) {
    if missing.is_empty() {
        ui.invoke_trigger_backend_message(MSG_INFO, format!("{verb} {name}").into());
    } else {
        ui.invoke_trigger_backend_message(
            MSG_WARN,
            format!("{verb} {name}, but still empty: {}", missing.join(", ")).into(),
        );
    }
}

fn write_pdf(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| {
        let hint = if e.kind() == std::io::ErrorKind::PermissionDenied {
            " Close it if it's open in a PDF viewer."
        } else {
            ""
        };
        format!("Could not write {}: {e}.{hint}", file_name(path))
    })
}

/// Puts an offer into the form.
fn fill_form(ui: &AppWindow, rows: &Rows, data: &Oferta) {
    let s = ui.global::<OfertaState>();

    s.set_data_oferta(data.data_oferta.as_str().into());
    s.set_valabilitate(opt_text(&data.valabilitate));
    s.set_obiect_titlu(data.obiect_titlu.as_str().into());
    s.set_obiect_descriere(data.obiect_descriere.as_str().into());
    s.set_referinta(opt_text(&data.referinta));

    s.set_client_nume(data.client.nume.as_str().into());
    s.set_client_cui(opt_text(&data.client.cui));
    s.set_client_adresa(opt_text(&data.client.adresa));
    s.set_client_contact(opt_text(&data.client.contact));

    s.set_cota_tva(data.cota_tva.map(oferta::format_number).unwrap_or_default().into());
    s.set_termen_zile(opt_text(&data.termen_zile));
    s.set_conditii_plata(opt_text(&data.conditii_plata));
    s.set_livrare(opt_text(&data.livrare));

    let mut new_rows: Vec<OfertaItem> = data.items.iter().map(to_row).collect();
    if new_rows.is_empty() {
        new_rows.push(blank_row());
    }
    rows.set_vec(new_rows);

    recalculate(ui, rows);
}

/// Reads the form back into an offer. Fails only on values that would
/// break the template (a quantity, price or VAT rate that isn't a number);
/// empty required text is reported after saving by `missing_fields`.
fn read_form(ui: &AppWindow, rows: &Rows) -> Result<Oferta, String> {
    let s = ui.global::<OfertaState>();

    let mut items = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if is_blank(&row) {
            continue;
        }
        let n = i + 1;
        items.push(Item {
            denumire: text(&row.denumire),
            descriere: opt(&row.descriere),
            um: text(&row.um),
            cant: required_number(&row.cant, &format!("Row {n}: quantity"))?,
            pret: required_number(&row.pret, &format!("Row {n}: unit price"))?,
        });
    }

    let cota_tva = match opt(&s.get_cota_tva()) {
        None => None,
        Some(t) => Some(
            oferta::parse_number(&t).ok_or_else(|| format!("VAT rate \"{t}\" is not a number"))?,
        ),
    };

    Ok(Oferta {
        data_oferta: text(&s.get_data_oferta()),
        valabilitate: opt(&s.get_valabilitate()),
        obiect_titlu: text(&s.get_obiect_titlu()),
        obiect_descriere: text(&s.get_obiect_descriere()),
        referinta: opt(&s.get_referinta()),
        client: Client {
            nume: text(&s.get_client_nume()),
            cui: opt(&s.get_client_cui()),
            adresa: opt(&s.get_client_adresa()),
            contact: opt(&s.get_client_contact()),
        },
        items,
        cota_tva,
        termen_zile: opt(&s.get_termen_zile()),
        conditii_plata: opt(&s.get_conditii_plata()),
        livrare: opt(&s.get_livrare()),
    })
}

/// Refreshes each row's value and the totals preview.
fn recalculate(ui: &AppWindow, rows: &Rows) {
    let mut values = Vec::with_capacity(rows.row_count());
    let mut all_numbers = true;

    for i in 0..rows.row_count() {
        let Some(mut row) = rows.row_data(i) else { continue };
        let value = match (preview_number(&row.cant), preview_number(&row.pret)) {
            (Some(cant), Some(pret)) => {
                let v = oferta::line_value(cant, pret);
                values.push(v);
                oferta::lei(v)
            }
            _ => {
                all_numbers = false;
                "?".to_string()
            }
        };
        if row.valoare.as_str() != value {
            row.valoare = value.into();
            rows.set_row_data(i, row);
        }
    }

    let s = ui.global::<OfertaState>();
    let cota = match s.get_cota_tva().trim() {
        "" => Some(oferta::DEFAULT_COTA_TVA),
        t => oferta::parse_number(t),
    };

    match cota {
        Some(cota) if all_numbers => {
            let t = oferta::totals(values, cota);
            s.set_total_fara_tva(oferta::lei(t.fara_tva).into());
            s.set_total_tva(oferta::lei(t.tva).into());
            s.set_total_de_plata(oferta::lei(t.de_plata).into());
        }
        _ => {
            s.set_total_fara_tva("?".into());
            s.set_total_tva("?".into());
            s.set_total_de_plata("?".into());
        }
    }
}

fn to_row(item: &Item) -> OfertaItem {
    OfertaItem {
        denumire: item.denumire.as_str().into(),
        descriere: opt_text(&item.descriere),
        um: item.um.as_str().into(),
        cant: oferta::format_number(item.cant).into(),
        pret: oferta::format_number(item.pret).into(),
        valoare: SharedString::new(),
    }
}

fn blank_row() -> OfertaItem {
    OfertaItem {
        um: "buc.".into(),
        cant: "1".into(),
        ..OfertaItem::default()
    }
}

/// A row nobody has typed into (U.M. and quantity have defaults, so they
/// don't count). Blank rows are skipped when saving.
fn is_blank(row: &OfertaItem) -> bool {
    row.denumire.trim().is_empty() && row.descriere.trim().is_empty() && row.pret.trim().is_empty()
}

fn required_number(value: &str, what: &str) -> Result<f64, String> {
    let t = value.trim();
    if t.is_empty() {
        return Err(format!("{what} is empty"));
    }
    oferta::parse_number(t).ok_or_else(|| format!("{what} \"{t}\" is not a number"))
}

/// For the live preview an empty cell counts as 0; text that isn't a
/// number gives `None` and shows as "?".
fn preview_number(value: &str) -> Option<f64> {
    match value.trim() {
        "" => Some(0.0),
        t => oferta::parse_number(t),
    }
}

fn text(value: &str) -> String {
    value.trim().to_string()
}

fn opt(value: &str) -> Option<String> {
    let t = value.trim();
    (!t.is_empty()).then(|| t.to_string())
}

fn opt_text(value: &Option<String>) -> SharedString {
    value.as_deref().unwrap_or_default().into()
}

fn current_dir(file: &CurrentFile) -> Option<PathBuf> {
    file.borrow().as_ref()?.parent().map(Path::to_path_buf)
}

fn remember_file(ui: &AppWindow, file: &CurrentFile, path: PathBuf) {
    ui.global::<OfertaState>().set_loaded_file(file_name(&path).into());
    file.replace(Some(path));
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// e.g. `oferta S.C. Exemplu S.R.L. 12.11.2026.json`, minus characters
/// Windows doesn't allow in file names.
fn default_file_name(data: &Oferta, extension: &str) -> String {
    let base = format!("oferta {} {}", data.client.nume.trim(), data.data_oferta.trim());
    let cleaned: String = base
        .chars()
        .filter(|c| !c.is_control() && !r#"<>:"/\|?*"#.contains(*c))
        .collect();
    let cleaned = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    format!("{}.{extension}", cleaned.trim_end_matches('.'))
}
