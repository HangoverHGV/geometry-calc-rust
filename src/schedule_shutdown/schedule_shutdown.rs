use std::process::Command;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const NO_WINDOW: u32 = 0x0800_0000; // CREATE_NO_WINDOW

fn run_shutdown(args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    let mut cmd = Command::new("shutdown");
    cmd.args(args);
    #[cfg(windows)]
    cmd.creation_flags(NO_WINDOW);
    cmd.status()
}

/// Schedules a shutdown `secs` from now. Returns a message suitable for the UI.
pub fn schedule(secs: i32) -> Result<String, String> {
    if secs <= 0 {
        return Err("Pick a time greater than zero.".into());
    }

    // Clear any pending shutdown, otherwise the next request is rejected.
    let _ = run_shutdown(&["/a"]);

    match run_shutdown(&["/s", "/t", &secs.to_string()]) {
        Ok(st) if st.success() => {
            let h = secs / 3600;
            let m = (secs % 3600) / 60;
            Ok(if h > 0 {
                format!("Shutdown scheduled in {h}h {m}m.")
            } else {
                format!("Shutdown scheduled in {m} min.")
            })
        }
        Ok(st) => Err(format!("shutdown failed (exit code {:?})", st.code())),
        Err(e) => Err(format!("Could not run shutdown: {e}")),
    }
}

pub fn cancel() -> Result<String, String> {
    match run_shutdown(&["/a"]) {
        Ok(_) => Ok("Shutdown cancelled.".into()),
        Err(e) => Err(format!("Could not run shutdown: {e}")),
    }
}