use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::logging;
use crate::mouse;
use crate::pty::PtyWriter;
use crate::terminal_buffer::TerminalBuffer;

const INPUT_DELAY_MS: u64 = 500;
const FOLLOWUP_INPUT_DELAY_MS: u64 = 1_000;
const SNAPSHOT_DELAY_MS: u64 = 2_000;

pub(crate) fn start_if_requested(buffer: Arc<Mutex<TerminalBuffer>>, writer: PtyWriter) {
    let input = env::var("MINIMAL_TERMINAL_SMOKE_INPUT").ok();
    let followup_input = env::var("MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT").ok();
    let second_followup_input = env::var("MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT").ok();
    let third_followup_input = env::var("MINIMAL_TERMINAL_SMOKE_THIRD_FOLLOWUP_INPUT").ok();
    let mouse_report = env::var("MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT").ok();
    let snapshot_path = env::var("MINIMAL_TERMINAL_SMOKE_SNAPSHOT_PATH").ok();

    if input.is_none()
        && followup_input.is_none()
        && second_followup_input.is_none()
        && third_followup_input.is_none()
        && mouse_report.is_none()
        && snapshot_path.is_none()
    {
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

        if let Some(input) = followup_input {
            thread::sleep(Duration::from_millis(env_u64(
                "MINIMAL_TERMINAL_SMOKE_FOLLOWUP_INPUT_DELAY_MS",
                FOLLOWUP_INPUT_DELAY_MS,
            )));
            if let Err(error) = writer.write_all(input.as_bytes()) {
                logging::pty_error(&format!("smoke follow-up input write failed: {error}"));
            }
        }

        if let Some(input) = second_followup_input {
            thread::sleep(Duration::from_millis(env_u64(
                "MINIMAL_TERMINAL_SMOKE_SECOND_FOLLOWUP_INPUT_DELAY_MS",
                FOLLOWUP_INPUT_DELAY_MS,
            )));
            if let Err(error) = writer.write_all(input.as_bytes()) {
                logging::pty_error(&format!(
                    "smoke second follow-up input write failed: {error}"
                ));
            }
        }

        if let Some(input) = third_followup_input {
            thread::sleep(Duration::from_millis(env_u64(
                "MINIMAL_TERMINAL_SMOKE_THIRD_FOLLOWUP_INPUT_DELAY_MS",
                FOLLOWUP_INPUT_DELAY_MS,
            )));
            if let Err(error) = writer.write_all(input.as_bytes()) {
                logging::pty_error(&format!(
                    "smoke third follow-up input write failed: {error}"
                ));
            }
        }

        if let Some(report) = mouse_report {
            thread::sleep(Duration::from_millis(env_u64(
                "MINIMAL_TERMINAL_SMOKE_MOUSE_REPORT_DELAY_MS",
                FOLLOWUP_INPUT_DELAY_MS,
            )));
            if let Some(bytes) = mouse_report_bytes(&buffer, &report) {
                for chunk in bytes {
                    if let Err(error) = writer.write_all(&chunk) {
                        logging::pty_error(&format!("smoke mouse report write failed: {error}"));
                        break;
                    }
                    thread::sleep(Duration::from_millis(50));
                }
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

fn mouse_report_bytes(buffer: &Arc<Mutex<TerminalBuffer>>, report: &str) -> Option<Vec<Vec<u8>>> {
    let modes = buffer
        .lock()
        .map(|buffer| buffer.snapshot(1).modes)
        .unwrap_or_default();

    if !modes.mouse_reporting {
        logging::pty_error("smoke mouse report skipped: mouse reporting mode is disabled");
        return None;
    }

    let bytes = match report {
        "left-press" if modes.sgr_mouse => {
            vec![mouse::sgr_mouse_report(mouse::LEFT_BUTTON, 0, 1, 2, false)]
        }
        "left-press" => vec![mouse::legacy_mouse_report(
            mouse::LEFT_BUTTON,
            0,
            1,
            2,
            false,
        )],
        "wheel-down" if modes.sgr_mouse => {
            vec![mouse::sgr_mouse_report(mouse::WHEEL_DOWN, 0, 1, 2, false)]
        }
        "wheel-down" => vec![mouse::legacy_mouse_report(
            mouse::WHEEL_DOWN,
            0,
            1,
            2,
            false,
        )],
        "wheel-down-5" if modes.sgr_mouse => (0..5)
            .map(|_| mouse::sgr_mouse_report(mouse::WHEEL_DOWN, 0, 1, 2, false))
            .collect(),
        "wheel-down-5" => (0..5)
            .map(|_| mouse::legacy_mouse_report(mouse::WHEEL_DOWN, 0, 1, 2, false))
            .collect(),
        _ => {
            logging::pty_error(&format!(
                "smoke mouse report skipped: unknown report {report}"
            ));
            return None;
        }
    };

    Some(bytes)
}

fn env_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
