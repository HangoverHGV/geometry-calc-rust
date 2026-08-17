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
pub fn get_all_printers(){
    for printer in get_printers() {
        println!("{:?}", printer);
    }
}