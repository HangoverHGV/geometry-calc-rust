mod batch_printing;
mod ui;
mod worker;

pub use batch_printing::get_all_printers;
pub use ui::setup_ui_printers;
pub use ui::open_file_dialog;
pub use worker::WORKER_ARG;

/// Entry point for `cad-helper-3d --print-worker`: prints one job and
/// returns the process exit code. Called from `main` before any UI exists.
pub fn run_worker() -> i32 {
    worker::run(batch_printing::print_job)
}
