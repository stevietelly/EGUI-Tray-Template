use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const MAX_LOG_BYTES: u64 = 1024 * 1024; // 1 MiB

fn log_path() -> Option<PathBuf> {
    let dir = dirs::data_local_dir()?.join("Featherweight");
    fs::create_dir_all(&dir).ok()?;
    Some(dir.join("template.log"))
}

/// Append a timestamped line to the log file.
/// Truncates the file at startup via [`log_init`] if it exceeds ~1 MiB.
/// Never panics — failures are silently ignored so logging never crashes the app.
pub fn log(msg: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{timestamp}] {msg}\n");
    eprintln!("{line}");

    // Also mirror to local working directory so in-tree diagnostics can inspect it.
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("featherweight.log") {
        let _ = f.write_all(line.as_bytes());
    }

    let Some(path) = log_path() else { return };
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = f.write_all(line.as_bytes());
    }
}

/// Call once at startup: truncates the log file if it is larger than ~1 MiB.
pub fn log_init() {
    let _ = fs::remove_file("template.log");

    let Some(path) = log_path() else { return };

    if let Ok(meta) = fs::metadata(&path) {
        if meta.len() > MAX_LOG_BYTES {
            let _ = fs::remove_file(&path);
        }
    }

    log("=== Featherweight started ===");
}
