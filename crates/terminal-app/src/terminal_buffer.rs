use terminal_core::{TerminalSnapshot, TerminalState};

const DEFAULT_ROWS: usize = 32;
const DEFAULT_COLS: usize = 100;

pub struct TerminalBuffer {
    terminal: TerminalState,
}

impl TerminalBuffer {
    pub fn new(_max_lines: usize) -> Self {
        Self {
            terminal: TerminalState::new(DEFAULT_ROWS, DEFAULT_COLS),
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.terminal.append_bytes(bytes);
        self.terminal.take_pending_responses()
    }

    pub fn take_pending_clipboard_writes(&mut self) -> Vec<String> {
        self.terminal.take_pending_clipboard_writes()
    }

    pub fn take_pending_title_writes(&mut self) -> Vec<String> {
        self.terminal.take_pending_title_writes()
    }

    pub fn snapshot(&self, max_visible_lines: usize) -> TerminalSnapshot {
        self.terminal.snapshot(max_visible_lines)
    }

    pub fn combined_snapshot(
        &self,
        offset_from_bottom: usize,
        max_visible_lines: usize,
    ) -> TerminalSnapshot {
        self.terminal
            .combined_snapshot(offset_from_bottom, max_visible_lines)
    }

    pub fn scrollback_len(&self) -> usize {
        self.terminal.scrollback_len()
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.terminal.resize(rows, cols);
    }
}
