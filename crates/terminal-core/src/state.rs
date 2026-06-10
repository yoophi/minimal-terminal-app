use crate::cursor::Cursor;
use crate::grid::Grid;
use crate::parser::{Action, Parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalSnapshot {
    pub lines: Vec<String>,
    pub cursor: Cursor,
}

#[derive(Clone, Debug)]
pub struct TerminalState {
    grid: Grid,
    cursor: Cursor,
    parser: Parser,
}

impl TerminalState {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            cursor: Cursor::default(),
            parser: Parser::default(),
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        let text = String::from_utf8_lossy(bytes);
        for ch in text.chars() {
            if let Some(action) = self.parser.advance(ch) {
                self.apply(action);
            }
        }
    }

    pub fn snapshot(&self, max_visible_lines: usize) -> TerminalSnapshot {
        let visible_rows = max_visible_lines.max(1).min(self.grid.rows());
        let start = if self.cursor.row < visible_rows {
            0
        } else {
            self.cursor.row + 1 - visible_rows
        };
        TerminalSnapshot {
            lines: self.grid.visible_lines_from(start, visible_rows),
            cursor: Cursor::new(self.cursor.row.saturating_sub(start), self.cursor.col),
        }
    }

    pub fn rows(&self) -> usize {
        self.grid.rows()
    }

    pub fn cols(&self) -> usize {
        self.grid.cols()
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Print(ch) => self.grid.put_char(&mut self.cursor, ch),
            Action::CarriageReturn => self.grid.carriage_return(&mut self.cursor),
            Action::Newline => self.grid.newline(&mut self.cursor),
            Action::Backspace => self.grid.backspace(&mut self.cursor),
            Action::Tab => {
                let next_tab = ((self.cursor.col / 8) + 1) * 8;
                while self.cursor.col < next_tab.min(self.grid.cols()) {
                    self.grid.put_char(&mut self.cursor, ' ');
                }
            }
            Action::ClearLineFromCursor => self.grid.clear_line_from_cursor(self.cursor),
            Action::ClearScreen => self.grid.clear_screen(&mut self.cursor),
            Action::CursorPosition { row, col } => {
                self.grid.move_cursor(&mut self.cursor, row, col)
            }
            Action::CursorUp(count) => self.grid.move_up(&mut self.cursor, count),
            Action::CursorDown(count) => self.grid.move_down(&mut self.cursor, count),
            Action::CursorRight(count) => self.grid.move_right(&mut self.cursor, count),
            Action::CursorLeft(count) => self.grid.move_left(&mut self.cursor, count),
            Action::CursorColumn(col) => {
                let row = self.cursor.row;
                self.grid.move_cursor(&mut self.cursor, row, col);
            }
            Action::Ignore => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TerminalState;
    use crate::Cursor;

    #[test]
    fn writes_printable_text_and_tracks_cursor() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"hello");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines, vec!["hello", "", ""]);
        assert_eq!(snapshot.cursor, Cursor::new(0, 5));
    }

    #[test]
    fn carriage_return_rewrites_current_line() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"prompt");
        terminal.append_bytes(b"\rnext");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "nextpt");
        assert_eq!(snapshot.cursor, Cursor::new(0, 4));
    }

    #[test]
    fn newline_moves_to_next_row() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"one\ntwo");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines, vec!["one", "two", ""]);
        assert_eq!(snapshot.cursor, Cursor::new(1, 3));
    }

    #[test]
    fn csi_clear_line_removes_stale_prompt_text() {
        let mut terminal = TerminalState::new(3, 16);
        terminal.append_bytes(b"old prompt");
        terminal.append_bytes(b"\r\x1b[Knew");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "new");
        assert_eq!(snapshot.cursor, Cursor::new(0, 3));
    }

    #[test]
    fn scrolls_when_output_exceeds_rows() {
        let mut terminal = TerminalState::new(2, 10);
        terminal.append_bytes(b"one\ntwo\nthree");

        let snapshot = terminal.snapshot(2);
        assert_eq!(snapshot.lines, vec!["two", "three"]);
        assert_eq!(snapshot.cursor, Cursor::new(1, 5));
    }

    #[test]
    fn snapshot_keeps_top_rows_visible_before_scrolling() {
        let mut terminal = TerminalState::new(4, 10);
        terminal.append_bytes(b"prompt");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines, vec!["prompt", "", ""]);
        assert_eq!(snapshot.cursor, Cursor::new(0, 6));
    }
}
