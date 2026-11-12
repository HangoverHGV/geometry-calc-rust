//! Renders the offer PDF in-process with Typst.
//!
//! The template, its images and fonts are compiled into the executable,
//! so nothing has to be installed or shipped next to it. Editing anything
//! in `oferta-typst/` and rebuilding picks up the change.

use std::sync::OnceLock;

use typst::diag::{Severity, SourceDiagnostic};
use typst::foundations::{Dict, Value};
use typst_as_lib::{TypstAsLibError, TypstEngine, TypstTemplateMainFile};
use typst_layout::PagedDocument;

use super::oferta::Oferta;

static TEMPLATE: &str = include_str!("../../oferta-typst/oferta.typ");
static LOGO: &[u8] = include_bytes!("../../oferta-typst/assets/logo.png");
static STAMPILA: &[u8] = include_bytes!("../../oferta-typst/assets/stampila.png");
static FONTS: [&[u8]; 4] = [
    include_bytes!("../../oferta-typst/fonts/LiberationSans-Regular.ttf"),
    include_bytes!("../../oferta-typst/fonts/LiberationSans-Bold.ttf"),
    include_bytes!("../../oferta-typst/fonts/LiberationSans-Italic.ttf"),
    include_bytes!("../../oferta-typst/fonts/LiberationSans-BoldItalic.ttf"),
];

/// Built once and reused; Typst also caches work between compilations.
fn engine() -> &'static TypstEngine<TypstTemplateMainFile> {
    static ENGINE: OnceLock<TypstEngine<TypstTemplateMainFile>> = OnceLock::new();
    ENGINE.get_or_init(|| {
        TypstEngine::builder()
            // A real path (not a detached source) so `assets/...` resolves
            // relative to the template, as it does with the CLI.
            .main_file(("oferta.typ", TEMPLATE))
            .fonts(FONTS)
            .with_static_file_resolver([("assets/logo.png", LOGO), ("assets/stampila.png", STAMPILA)])
            .build()
    })
}

/// Returns the PDF bytes for an offer.
///
/// The data goes in as the JSON string `sys.inputs.data`, the same input the
/// CLI gets from `--input data='{...}'`.
pub fn render(oferta: &Oferta) -> Result<Vec<u8>, String> {
    let json = serde_json::to_string(oferta).map_err(|e| format!("Could not encode offer: {e}"))?;
    let mut inputs = Dict::new();
    inputs.insert("data".into(), Value::Str(json.as_str().into()));

    let document: PagedDocument = engine()
        .compile_with_input(inputs)
        .output
        .map_err(describe_error)?;

    typst_pdf::pdf(&document, &Default::default()).map_err(|diags| describe_diagnostics(&diags))
}

fn describe_error(err: TypstAsLibError) -> String {
    match err {
        TypstAsLibError::TypstSource(diags) => describe_diagnostics(&diags),
        other => format!("PDF template error: {other}"),
    }
}

fn describe_diagnostics(diags: &[SourceDiagnostic]) -> String {
    let messages: Vec<&str> = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .map(|d| d.message.as_str())
        .collect();
    format!("PDF template error: {}", messages.join("; "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oferta::oferta::{Client, Item};

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF-") && bytes.len() > 10_000
    }

    #[test]
    fn renders_the_example_offer() {
        let example: Oferta =
            serde_json::from_str(include_str!("../../oferta-typst/oferta.json")).unwrap();
        assert!(is_pdf(&render(&example).unwrap()));
    }

    #[test]
    fn renders_with_only_the_required_fields() {
        let minimal = Oferta {
            data_oferta: "12.11.2026".into(),
            obiect_titlu: "Titlu".into(),
            obiect_descriere: "Descriere".into(),
            client: Client { nume: "Client".into(), ..Client::default() },
            items: vec![Item { denumire: "X".into(), um: "buc.".into(), cant: 7.5, pret: 99.99, ..Item::default() }],
            ..Oferta::default()
        };
        assert!(is_pdf(&render(&minimal).unwrap()));
    }

    #[test]
    fn renders_a_blank_form() {
        // What "New" + "Generate PDF" produces: empty strings, no rows.
        assert!(is_pdf(&render(&Oferta::default()).unwrap()));
    }
}
