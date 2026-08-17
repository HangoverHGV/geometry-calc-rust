use printers::{
    common::{
        base::job::PrinterJobOptions,
        converters::{
            Converter,
            GhostscriptConverterOptions,
        },
    },
    get_printer_by_name,
    get_default_printer,
    get_printers
};

pub fn get_all_printers() -> Result<Option<Vec<String>>, String>{
    let printers = get_printers();

    if printers.is_empty(){
        return Ok(None);
    }

    let printer_names: Vec<String> = printers.into_iter()
                                                 .map(|printer| printer.name)
                                                 .collect();

    Ok(Some(printer_names))
}