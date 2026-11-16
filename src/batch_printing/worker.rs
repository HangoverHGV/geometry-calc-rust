
use std::fmt::Display;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

pub const WORKER_ARG: &str = "--print-worker";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintJob {
    pub printer: String,
    pub landscape: bool,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PrintReport {
    /// Files handed to the spooler without an error.
    pub printed: usize,
    pub failed: Vec<FailedFile>,
    /// Set when the job couldn't start at all (printer missing, bad ticket...).
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailedFile {
    pub file: String,
    pub error: String,
}

impl PrintReport {
    pub fn job_error(message: impl Into<String>) -> Self {
        PrintReport { error: Some(message.into()), ..PrintReport::default() }
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none() && self.failed.is_empty()
    }
}

// ---------- app side ----------

/// Runs `job` in a worker process and waits for it to finish.
/// Blocks, so call it from a background thread.
pub fn run_in_worker(job: &PrintJob) -> Result<PrintReport, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("Could not find the app executable to start printing: {e}"))?;
    let input = serde_json::to_vec(job).map_err(|e| format!("Could not encode the print job: {e}"))?;

    let mut command = Command::new(exe);
    command
        .arg(WORKER_ARG)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("Could not start the print worker: {e}"))?;

    // Dropping stdin after writing closes it, so the worker sees the end of the job.
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&input)
            .map_err(|e| format!("Could not send the job to the print worker: {e}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Lost contact with the print worker: {e}"))?;
    parse_report(&output.stdout, &output.stderr, output.status)
}

/// The report is the last line of stdout that looks like JSON. Anything else
/// (pdfium or driver chatter) is ignored. No report means the worker crashed.
fn parse_report(stdout: &[u8], stderr: &[u8], status: impl Display) -> Result<PrintReport, String> {
    let stdout = String::from_utf8_lossy(stdout);
    let report = stdout
        .lines()
        .rev()
        .filter(|line| line.trim_start().starts_with('{'))
        .find_map(|line| serde_json::from_str::<PrintReport>(line).ok());

    report.ok_or_else(|| {
        let stderr = String::from_utf8_lossy(stderr);
        match stderr.lines().rev().find(|l| !l.trim().is_empty()) {
            Some(last) => format!("The print worker stopped unexpectedly ({status}): {}", last.trim()),
            None => format!("The print worker stopped unexpectedly ({status})."),
        }
    })
}

// ---------- worker side ----------

/// Entry point for `--print-worker`: reads the job from stdin, prints it
/// with `print` and writes the report to stdout. Returns the exit code.
pub fn run(print: fn(&PrintJob) -> PrintReport) -> i32 {
    let mut input = String::new();
    let report = match std::io::stdin().read_to_string(&mut input) {
        Err(e) => PrintReport::job_error(format!("Could not read the print job: {e}")),
        Ok(_) => match serde_json::from_str::<PrintJob>(&input) {
            Ok(job) => print(&job),
            Err(e) => PrintReport::job_error(format!("Invalid print job: {e}")),
        },
    };

    // One line, so the app can find it among any other output. Ignore write
    // errors: the app may have been closed and nobody is listening.
    if let Ok(line) = serde_json::to_string(&report) {
        let _ = writeln!(std::io::stdout(), "{line}");
    }
    if report.is_ok() { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report_line(report: &PrintReport) -> String {
        serde_json::to_string(report).unwrap()
    }

    #[test]
    fn finds_the_report_among_other_output() {
        let report = PrintReport {
            printed: 2,
            failed: vec![FailedFile { file: "b.pdf".into(), error: "boom".into() }],
            error: None,
        };
        let stdout = format!("pdfium: loading\n{}\n\n", report_line(&report));
        assert_eq!(parse_report(stdout.as_bytes(), b"", "exit code: 1"), Ok(report));
    }

    #[test]
    fn missing_report_means_the_worker_crashed() {
        let err = parse_report(b"", b"thread 'main' panicked\nstack overflow\n", "exit code: 101").unwrap_err();
        assert_eq!(err, "The print worker stopped unexpectedly (exit code: 101): stack overflow");

        let err = parse_report(b"{not json", b"", "exit code: -1").unwrap_err();
        assert_eq!(err, "The print worker stopped unexpectedly (exit code: -1).");
    }

    #[test]
    fn job_survives_the_round_trip() {
        let job = PrintJob {
            printer: "HP LaserJet (Birou)".into(),
            landscape: true,
            files: vec![r"D:\Proiecte\VDN271001\Draw\Reper 1.pdf".into(), "ș ț ă.pdf".into()],
        };
        let back: PrintJob = serde_json::from_str(&serde_json::to_string(&job).unwrap()).unwrap();
        assert_eq!(back, job);
    }
}
