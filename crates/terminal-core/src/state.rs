use crate::cursor::Cursor;
use crate::grid::Grid;
use crate::parser::{Action, LineClearMode, Parser, ScreenClearMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalSnapshot {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub scrollback_len: usize,
}

#[derive(Clone, Debug)]
pub struct TerminalState {
    grid: Grid,
    cursor: Cursor,
    saved_cursor: Cursor,
    main_screen: Option<ScreenState>,
    parser: Parser,
}

#[derive(Clone, Debug)]
struct ScreenState {
    grid: Grid,
    cursor: Cursor,
    saved_cursor: Cursor,
}

impl TerminalState {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            cursor: Cursor::default(),
            saved_cursor: Cursor::default(),
            main_screen: None,
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
            scrollback_len: self.grid.scrollback_len(),
        }
    }

    pub fn rows(&self) -> usize {
        self.grid.rows()
    }

    pub fn cols(&self) -> usize {
        self.grid.cols()
    }

    pub fn scrollback_len(&self) -> usize {
        self.grid.scrollback_len()
    }

    pub fn scrollback_snapshot(
        &self,
        offset_from_bottom: usize,
        max_visible_lines: usize,
    ) -> TerminalSnapshot {
        TerminalSnapshot {
            lines: self
                .grid
                .scrollback_lines(offset_from_bottom, max_visible_lines),
            cursor: Cursor::default(),
            scrollback_len: self.grid.scrollback_len(),
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.grid.resize(rows, cols, &mut self.cursor);
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
            Action::ClearLine(mode) => match mode {
                LineClearMode::FromCursor => self.grid.clear_line_from_cursor(self.cursor),
                LineClearMode::ToCursor => self.grid.clear_line_to_cursor(self.cursor),
                LineClearMode::Entire => self.grid.clear_entire_line(self.cursor.row),
            },
            Action::ClearScreen(mode) => match mode {
                ScreenClearMode::FromCursor => self.grid.clear_screen_from_cursor(self.cursor),
                ScreenClearMode::ToCursor => self.grid.clear_screen_to_cursor(self.cursor),
                ScreenClearMode::Entire => self.grid.clear_screen(&mut self.cursor),
            },
            Action::SaveCursor => self.saved_cursor = self.cursor,
            Action::RestoreCursor => {
                self.grid.move_cursor(
                    &mut self.cursor,
                    self.saved_cursor.row,
                    self.saved_cursor.col,
                );
            }
            Action::EnterAlternateScreen => self.enter_alternate_screen(),
            Action::ExitAlternateScreen => self.exit_alternate_screen(),
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

    fn enter_alternate_screen(&mut self) {
        if self.main_screen.is_some() {
            self.grid.clear_screen(&mut self.cursor);
            return;
        }

        let main_screen = ScreenState {
            grid: self.grid.clone(),
            cursor: self.cursor,
            saved_cursor: self.saved_cursor,
        };
        let rows = self.grid.rows();
        let cols = self.grid.cols();
        self.grid = Grid::new(rows, cols);
        self.cursor = Cursor::default();
        self.saved_cursor = Cursor::default();
        self.main_screen = Some(main_screen);
    }

    fn exit_alternate_screen(&mut self) {
        let Some(main_screen) = self.main_screen.take() else {
            return;
        };

        self.grid = main_screen.grid;
        self.cursor = main_screen.cursor;
        self.saved_cursor = main_screen.saved_cursor;
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
    fn csi_clear_entire_line_removes_text_on_both_sides() {
        let mut terminal = TerminalState::new(3, 16);
        terminal.append_bytes(b"old prompt");
        terminal.append_bytes(b"\rnew\x1b[2Kok");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "   ok");
        assert_eq!(snapshot.cursor, Cursor::new(0, 5));
    }

    #[test]
    fn saves_and_restores_cursor() {
        let mut terminal = TerminalState::new(3, 16);
        terminal.append_bytes(b"ab\x1b7cd\x1b8X");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "abXd");
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

    #[test]
    fn resize_preserves_visible_content_and_clamps_cursor() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"hello");

        terminal.resize(2, 4);

        let snapshot = terminal.snapshot(2);
        assert_eq!(snapshot.lines, vec!["hell", ""]);
        assert_eq!(snapshot.cursor, Cursor::new(0, 3));
    }

    #[test]
    fn scrolling_records_scrollback_length() {
        let mut terminal = TerminalState::new(2, 10);
        terminal.append_bytes(b"one\ntwo\nthree");

        assert_eq!(terminal.scrollback_len(), 1);
    }

    #[test]
    fn exposes_scrollback_snapshot() {
        let mut terminal = TerminalState::new(2, 10);
        terminal.append_bytes(b"one\ntwo\nthree");

        let snapshot = terminal.scrollback_snapshot(0, 10);
        assert_eq!(snapshot.lines, vec!["one"]);
        assert_eq!(snapshot.scrollback_len, 1);
    }

    #[test]
    fn alternate_screen_restores_main_screen() {
        let mut terminal = TerminalState::new(3, 16);
        terminal.append_bytes(b"main");
        terminal.append_bytes(b"\x1b[?1049halt");

        let alternate = terminal.snapshot(3);
        assert_eq!(alternate.lines[0], "alt");

        terminal.append_bytes(b"\x1b[?1049l");
        let restored = terminal.snapshot(3);
        assert_eq!(restored.lines[0], "main");
        assert_eq!(restored.cursor, Cursor::new(0, 4));
    }

    #[test]
    fn wide_characters_advance_cursor_by_two_cells() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes("한글".as_bytes());

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "한글");
        assert_eq!(snapshot.cursor, Cursor::new(0, 4));
    }

    #[test]
    fn backspace_removes_whole_wide_character() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes("A한".as_bytes());
        terminal.append_bytes(&[0x7f]);

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "A");
        assert_eq!(snapshot.cursor, Cursor::new(0, 1));
    }
}
