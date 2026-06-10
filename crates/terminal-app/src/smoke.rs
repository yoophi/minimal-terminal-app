use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::logging;
use crate::pty::PtyWriter;
use crate::terminal_buffer::TerminalBuffer;

const INPUT_DELAY_MS: u64 = 500;
const SNAPSHOT_DELAY_MS: u64 = 2_000;

pub(crate) fn start_if_requested(buffer: Arc<Mutex<TerminalBuffer>>, writer: PtyWriter) {
    let input = env::var("MINIMAL_TERMINAL_SMOKE_INPUT").ok();
    let snapshot_path = env::var("MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH").ok();

    if input.is_none() && snapshot_path.is_none() {
        return;
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(env_u64(
            "MINIMAL_TERMINAL_SMOKE_INPUT_DELAY_MS",
            INPUT_DELAY_MS,
        )));

        if let Some(input) = input {
            if let Err(error) = writer.write_all(input.as_bytes()) {
                logging::pty_error(&format!("smoke input write failed: {error}"));
            }
        }

        thread::sleep(Duration::from_millis(env_u64(
            "MINIMAL_TERMINAL_SMOKE_SNAPSHOT_DELAY_MS",
            SNAPSHOT_DELAY_MS,
        )));

        if let Some(path) = snapshot_path {
            let snapshot_text = buffer
                .lock()
                .map(|buffer| buffer.combined_snapshot(0, usize::MAX).lines.join("\n"))
                .unwrap_or_else(|_| "terminal buffer unavailable".to_string());
            if let Err(error) = fs::write(&path, snapshot_text) {
                logging::pty_error(&format!("smoke snapshot write failed: {error}"));
            }
        }

        if env::var("MINIMAL_TERMINAL_SMOKE_EXIT")
            .map(|value| value != "0")
            .unwrap_or(true)
        {
            std::process::exit(0);
        }
    });
}

fn env_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
