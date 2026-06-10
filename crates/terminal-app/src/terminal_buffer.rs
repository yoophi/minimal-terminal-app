use terminal_core::{TerminalSnapshot, TerminalState};

const DEFAULT_ROWS: usize = 32;
const DEFAULT_COLS: usize = 100;

#[derive(Debug)]
pub struct TerminalBuffer {
    terminal: TerminalState,
}

impl TerminalBuffer {
    pub fn new(_max_lines: usize) -> Self {
        Self {
            terminal: TerminalState::new(DEFAULT_ROWS, DEFAULT_COLS),
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.terminal.append_bytes(bytes);
    }

    pub fn snapshot(&self, max_visible_lines: usize) -> TerminalSnapshot {
        self.terminal.snapshot(max_visible_lines)
    }
}
