use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const NO_WINDOW: u32 = 0x0800_0000; // CREATE_NO_WINDOW

/// `shutdown /a` returns this when nothing was pending.
const ERROR_NO_SHUTDOWN_IN_PROGRESS: i32 = 1116;

fn run_shutdown(args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    let mut cmd = Command::new("shutdown");
    cmd.args(args);
    #[cfg(windows)]
    cmd.creation_flags(NO_WINDOW);
    cmd.status()
}

fn state_path() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join("cad-helper-3d");
    fs::create_dir_all(&dir).ok()?;
    Some(dir.join("shutdown.txt"))
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn save_deadline(unix: u64) {
    if let Some(p) = state_path() {
        let _ = fs::write(p, unix.to_string());
    }
}

fn clear_deadline() {
    if let Some(p) = state_path() {
        let _ = fs::remove_file(p);
    }
}

/// Seconds left on a shutdown scheduled in a previous run, if still in the future.
pub fn restore_remaining() -> Option<i32> {
    let p = state_path()?;
    let deadline: u64 = fs::read_to_string(&p).ok()?.trim().parse().ok()?;
    if deadline > now_unix() {
        Some((deadline - now_unix()).min(i32::MAX as u64) as i32)
    } else {
        let _ = fs::remove_file(p); // stale
        None
    }
}

/// Schedules a shutdown `secs` from now. Returns a message for the UI.
pub fn schedule(secs: i32) -> Result<String, String> {
    if secs <= 0 {
        return Err("Pick a time greater than zero.".into());
    }

    // Clear any pending shutdown, otherwise the next request is rejected.
    let _ = run_shutdown(&["/a"]);

    match run_shutdown(&["/s", "/t", &secs.to_string()]) {
        Ok(st) if st.success() => {
            save_deadline(now_unix() + secs as u64);
            let h = secs / 3600;
            let m = (secs % 3600) / 60;
            Ok(if h > 0 {
                format!("Shutdown scheduled in {h}h {m}m.")
            } else {
                format!("Shutdown scheduled in {m} min.")
            })
        }
        Ok(st) => Err(format!("shutdown failed (exit code {:?}).", st.code())),
        Err(e) => Err(format!("Could not run shutdown: {e}")),
    }
}

pub fn cancel() -> Result<String, String> {
    clear_deadline();

    match run_shutdown(&["/a"]) {
        Ok(st) if st.success() => Ok("Shutdown cancelled.".into()),
        Ok(st) if st.code() == Some(ERROR_NO_SHUTDOWN_IN_PROGRESS) => {
            Ok("No shutdown was pending.".into())
        }
        Ok(st) => Err(format!("Could not cancel (exit code {:?}).", st.code())),
        Err(e) => Err(format!("Could not run shutdown: {e}")),
    }
}