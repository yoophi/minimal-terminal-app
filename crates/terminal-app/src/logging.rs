use std::sync::OnceLock;

use oslog::OsLog;

const SUBSYSTEM: &str = "dev.minimal-terminal.app";

static APP_LOG: OnceLock<OsLog> = OnceLock::new();
static PTY_LOG: OnceLock<OsLog> = OnceLock::new();

pub fn app_info(message: &str) {
    APP_LOG
        .get_or_init(|| OsLog::new(SUBSYSTEM, "app"))
        .default(message);
}

pub fn app_error(message: &str) {
    APP_LOG
        .get_or_init(|| OsLog::new(SUBSYSTEM, "app"))
        .error(message);
}

pub fn pty_info(message: &str) {
    PTY_LOG
        .get_or_init(|| OsLog::new(SUBSYSTEM, "pty"))
        .default(message);
}

pub fn pty_error(message: &str) {
    PTY_LOG
        .get_or_init(|| OsLog::new(SUBSYSTEM, "pty"))
        .error(message);
}
