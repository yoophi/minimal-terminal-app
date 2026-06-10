#[derive(Debug)]
pub struct TerminalBuffer {
    lines: Vec<String>,
    current_line: String,
    max_lines: usize,
    escape: EscapeState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EscapeState {
    None,
    Esc,
    Csi,
}

impl TerminalBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: Vec::new(),
            current_line: String::new(),
            max_lines,
            escape: EscapeState::None,
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        let text = String::from_utf8_lossy(bytes);
        for ch in text.chars() {
            self.push_char(ch);
        }
        self.trim();
    }

    pub fn visible_text(&self, max_visible_lines: usize) -> String {
        let mut lines = self.lines.clone();
        lines.push(self.current_line.clone());

        let start = lines.len().saturating_sub(max_visible_lines);
        lines[start..].join("\n")
    }

    fn push_char(&mut self, ch: char) {
        match self.escape {
            EscapeState::None => self.push_printable_or_control(ch),
            EscapeState::Esc => self.advance_escape_after_esc(ch),
            EscapeState::Csi => self.advance_escape_after_csi(ch),
        }
    }

    fn push_printable_or_control(&mut self, ch: char) {
        match ch {
            '\u{1b}' => self.escape = EscapeState::Esc,
            '\r' => self.current_line.clear(),
            '\n' => self.commit_line(),
            '\u{08}' | '\u{7f}' => {
                self.current_line.pop();
            }
            '\t' => self.current_line.push_str("    "),
            ch if ch.is_control() => {}
            ch => self.current_line.push(ch),
        }
    }

    fn advance_escape_after_esc(&mut self, ch: char) {
        self.escape = if ch == '[' {
            EscapeState::Csi
        } else {
            EscapeState::None
        };
    }

    fn advance_escape_after_csi(&mut self, ch: char) {
        if ('@'..='~').contains(&ch) {
            self.escape = EscapeState::None;
        }
    }

    fn commit_line(&mut self) {
        self.lines.push(std::mem::take(&mut self.current_line));
    }

    fn trim(&mut self) {
        if self.lines.len() > self.max_lines {
            let overflow = self.lines.len() - self.max_lines;
            self.lines.drain(0..overflow);
        }
    }
}

