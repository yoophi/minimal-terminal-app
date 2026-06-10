use crate::cursor::{Cursor, CursorStyle};
use crate::grid::Grid;
use crate::parser::{apply_sgr, Action, LineClearMode, Parser, ScreenClearMode};
use crate::style::{Style, StyledLine};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalSnapshot {
    pub lines: Vec<String>,
    pub styled_lines: Vec<StyledLine>,
    pub cursor: Cursor,
    pub modes: TerminalModes,
    pub scrollback_len: usize,
    pub viewport_start_absolute_row: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalModes {
    pub cursor_visible: bool,
    pub cursor_style: CursorStyle,
    pub bracketed_paste: bool,
    pub application_cursor_keys: bool,
    pub application_keypad: bool,
    pub mouse_reporting: bool,
    pub sgr_mouse: bool,
}

impl Default for TerminalModes {
    fn default() -> Self {
        Self {
            cursor_visible: true,
            cursor_style: CursorStyle::Block,
            bracketed_paste: false,
            application_cursor_keys: false,
            application_keypad: false,
            mouse_reporting: false,
            sgr_mouse: false,
        }
    }
}

pub struct TerminalState {
    grid: Grid,
    cursor: Cursor,
    saved_cursor: Cursor,
    current_style: Style,
    modes: TerminalModes,
    scroll_region: Option<(usize, usize)>,
    pending_responses: Vec<u8>,
    pending_clipboard_writes: Vec<String>,
    pending_title_writes: Vec<String>,
    main_screen: Option<ScreenState>,
    parser: Parser,
}

#[derive(Clone, Debug)]
struct ScreenState {
    grid: Grid,
    cursor: Cursor,
    saved_cursor: Cursor,
    current_style: Style,
    modes: TerminalModes,
    scroll_region: Option<(usize, usize)>,
}

impl TerminalState {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            cursor: Cursor::default(),
            saved_cursor: Cursor::default(),
            current_style: Style::default(),
            modes: TerminalModes::default(),
            scroll_region: None,
            pending_responses: Vec::new(),
            pending_clipboard_writes: Vec::new(),
            pending_title_writes: Vec::new(),
            main_screen: None,
            parser: Parser::default(),
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        for action in self.parser.advance_bytes(bytes) {
            self.apply(action);
        }
    }

    pub fn take_pending_responses(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pending_responses)
    }

    pub fn take_pending_clipboard_writes(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_clipboard_writes)
    }

    pub fn take_pending_title_writes(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_title_writes)
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
            styled_lines: self.grid.visible_styled_lines_from(start, visible_rows),
            cursor: Cursor::new(self.cursor.row.saturating_sub(start), self.cursor.col),
            modes: self.modes,
            scrollback_len: self.grid.scrollback_len(),
            viewport_start_absolute_row: self.grid.scrollback_len() + start,
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
            styled_lines: self
                .grid
                .scrollback_styled_lines(offset_from_bottom, max_visible_lines),
            cursor: Cursor::default(),
            modes: self.modes,
            scrollback_len: self.grid.scrollback_len(),
            viewport_start_absolute_row: self
                .grid
                .scrollback_len()
                .saturating_sub(offset_from_bottom)
                .saturating_sub(max_visible_lines),
        }
    }

    pub fn combined_snapshot(
        &self,
        offset_from_bottom: usize,
        max_visible_lines: usize,
    ) -> TerminalSnapshot {
        let max_visible_lines = max_visible_lines.max(1);
        let scrollback_len = self.grid.scrollback_len();
        let mut lines = self.grid.scrollback_lines(0, scrollback_len);
        let mut styled_lines = self.grid.scrollback_styled_lines(0, scrollback_len);
        lines.extend(self.grid.visible_lines_from(0, self.grid.rows()));
        styled_lines.extend(self.grid.visible_styled_lines_from(0, self.grid.rows()));

        let total = lines.len();
        let end = total.saturating_sub(offset_from_bottom).max(1).min(total);
        let start = end.saturating_sub(max_visible_lines);
        let cursor_absolute_row = scrollback_len + self.cursor.row;
        let cursor = if cursor_absolute_row >= start && cursor_absolute_row < end {
            Cursor::new(cursor_absolute_row - start, self.cursor.col)
        } else {
            Cursor::default()
        };

        TerminalSnapshot {
            lines: lines[start..end].to_vec(),
            styled_lines: styled_lines[start..end].to_vec(),
            cursor,
            modes: self.modes,
            scrollback_len,
            viewport_start_absolute_row: start,
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.grid.resize(rows, cols, &mut self.cursor);
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Print(ch) => self.grid.put_char(&mut self.cursor, ch, self.current_style),
            Action::CarriageReturn => self.grid.carriage_return(&mut self.cursor),
            Action::Newline => self
                .grid
                .newline_in_region(&mut self.cursor, self.scroll_region),
            Action::Backspace => self.grid.backspace(&mut self.cursor),
            Action::Tab => {
                let next_tab = ((self.cursor.col / 8) + 1) * 8;
                while self.cursor.col < next_tab.min(self.grid.cols()) {
                    self.grid
                        .put_char(&mut self.cursor, ' ', self.current_style);
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
            Action::InsertBlankChars(count) => self.grid.insert_blank_chars(self.cursor, count),
            Action::DeleteChars(count) => self.grid.delete_chars(self.cursor, count),
            Action::EraseChars(count) => self.grid.erase_chars(self.cursor, count),
            Action::InsertLines(count) => {
                self.grid
                    .insert_blank_lines(self.cursor, count, self.scroll_region)
            }
            Action::DeleteLines(count) => {
                self.grid
                    .delete_lines(self.cursor, count, self.scroll_region)
            }
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
            Action::SetApplicationCursorKeys(enabled) => {
                self.modes.application_cursor_keys = enabled
            }
            Action::SetApplicationKeypad(enabled) => self.modes.application_keypad = enabled,
            Action::SetBracketedPaste(enabled) => self.modes.bracketed_paste = enabled,
            Action::SetCursorVisible(visible) => self.modes.cursor_visible = visible,
            Action::SetCursorStyle(style) => self.modes.cursor_style = style,
            Action::SetMouseReporting(enabled) => self.modes.mouse_reporting = enabled,
            Action::SetSgrMouse(enabled) => self.modes.sgr_mouse = enabled,
            Action::PrimaryDeviceAttributes => {
                self.pending_responses.extend_from_slice(b"\x1b[?1;2c")
            }
            Action::SecondaryDeviceAttributes => {
                self.pending_responses.extend_from_slice(b"\x1b[>0;0;0c")
            }
            Action::SetClipboard(text) => self.pending_clipboard_writes.push(text),
            Action::ClipboardQueryDenied(selector) => self
                .pending_responses
                .extend_from_slice(format!("\x1b]52;{selector};\x07").as_bytes()),
            Action::SetWindowTitle(title) => self.pending_title_writes.push(title),
            Action::DeviceStatusReport => self.pending_responses.extend_from_slice(b"\x1b[0n"),
            Action::CursorPositionReport => {
                self.pending_responses.extend_from_slice(
                    format!("\x1b[{};{}R", self.cursor.row + 1, self.cursor.col + 1).as_bytes(),
                );
            }
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
            Action::SetScrollRegion(region) => {
                self.scroll_region = region;
                self.grid.move_cursor(&mut self.cursor, 0, 0);
            }
            Action::SetGraphicRendition(numbers) => apply_sgr(&mut self.current_style, &numbers),
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
            current_style: self.current_style,
            modes: self.modes,
            scroll_region: self.scroll_region,
        };
        let rows = self.grid.rows();
        let cols = self.grid.cols();
        self.grid = Grid::new(rows, cols);
        self.cursor = Cursor::default();
        self.saved_cursor = Cursor::default();
        self.current_style = Style::default();
        self.modes.cursor_visible = true;
        self.modes.cursor_style = CursorStyle::Block;
        self.modes.bracketed_paste = false;
        self.modes.application_cursor_keys = false;
        self.modes.application_keypad = false;
        self.modes.mouse_reporting = false;
        self.modes.sgr_mouse = false;
        self.scroll_region = None;
        self.main_screen = Some(main_screen);
    }

    fn exit_alternate_screen(&mut self) {
        let Some(main_screen) = self.main_screen.take() else {
            return;
        };

        self.grid = main_screen.grid;
        self.cursor = main_screen.cursor;
        self.saved_cursor = main_screen.saved_cursor;
        self.current_style = main_screen.current_style;
        self.modes = main_screen.modes;
        self.scroll_region = main_screen.scroll_region;
    }
}

#[cfg(test)]
mod tests {
    use super::TerminalState;
    use crate::{Color, Cursor, CursorStyle, Style};

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

    #[test]
    fn tracks_tui_modes() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"\x1b[?25l\x1b[?2004h\x1b[?1h\x1b=");

        let snapshot = terminal.snapshot(3);
        assert!(!snapshot.modes.cursor_visible);
        assert!(snapshot.modes.bracketed_paste);
        assert!(snapshot.modes.application_cursor_keys);
        assert!(snapshot.modes.application_keypad);

        terminal.append_bytes(b"\x1b[?25h\x1b[?2004l\x1b[?1l\x1b>");
        let snapshot = terminal.snapshot(3);
        assert!(snapshot.modes.cursor_visible);
        assert!(!snapshot.modes.bracketed_paste);
        assert!(!snapshot.modes.application_cursor_keys);
        assert!(!snapshot.modes.application_keypad);

        terminal.append_bytes(b"\x1b[?1000h\x1b[?1006h");
        let snapshot = terminal.snapshot(3);
        assert!(snapshot.modes.mouse_reporting);
        assert!(snapshot.modes.sgr_mouse);
    }

    #[test]
    fn handles_insert_delete_and_erase_characters() {
        let mut terminal = TerminalState::new(3, 12);
        terminal.append_bytes(b"abcdef\r\x1b[2C\x1b[2@");
        assert_eq!(terminal.snapshot(3).lines[0], "ab  cdef");

        terminal.append_bytes(b"\r\x1b[2C\x1b[3P");
        assert_eq!(terminal.snapshot(3).lines[0], "abdef");

        terminal.append_bytes(b"\r\x1b[1C\x1b[2X");
        assert_eq!(terminal.snapshot(3).lines[0], "a  ef");
    }

    #[test]
    fn handles_scroll_region_newline() {
        let mut terminal = TerminalState::new(5, 12);
        terminal.append_bytes(b"0\n1\n2\n3\n4");
        terminal.append_bytes(b"\x1b[2;4r\x1b[4;1H\nx");

        let snapshot = terminal.snapshot(5);
        assert_eq!(snapshot.lines, vec!["0", "2", "3", "x", "4"]);
    }

    #[test]
    fn handles_insert_and_delete_lines_in_scroll_region() {
        let mut terminal = TerminalState::new(5, 12);
        terminal.append_bytes(b"0\n1\n2\n3\n4");
        terminal.append_bytes(b"\x1b[2;5r\x1b[3;1H\x1b[L");

        let snapshot = terminal.snapshot(5);
        assert_eq!(snapshot.lines, vec!["0", "1", "", "2", "3"]);

        terminal.append_bytes(b"\x1b[3;1H\x1b[M");
        let snapshot = terminal.snapshot(5);
        assert_eq!(snapshot.lines, vec!["0", "1", "2", "3", ""]);
    }

    #[test]
    fn stores_sgr_style_per_cell() {
        let mut terminal = TerminalState::new(3, 20);
        terminal.append_bytes(b"\x1b[1;31mred\x1b[0m plain");

        let snapshot = terminal.snapshot(3);
        assert_eq!(snapshot.lines[0], "red plain");
        assert_eq!(snapshot.styled_lines[0].spans.len(), 2);
        assert_eq!(snapshot.styled_lines[0].spans[0].text, "red");
        assert_eq!(
            snapshot.styled_lines[0].spans[0].style,
            Style {
                foreground: Some(Color::Indexed(1)),
                bold: true,
                ..Style::default()
            }
        );
        assert_eq!(snapshot.styled_lines[0].spans[1].text, " plain");
        assert_eq!(snapshot.styled_lines[0].spans[1].style, Style::default());
    }

    #[test]
    fn stores_extended_sgr_colors() {
        let mut terminal = TerminalState::new(3, 20);
        terminal.append_bytes(b"\x1b[38;5;196midx\x1b[48;2;1;2;3m rgb");

        let snapshot = terminal.snapshot(3);
        assert_eq!(
            snapshot.styled_lines[0].spans[0].style.foreground,
            Some(Color::Indexed(196))
        );
        assert_eq!(
            snapshot.styled_lines[0].spans[1].style.background,
            Some(Color::Rgb(1, 2, 3))
        );
    }

    #[test]
    fn queues_device_status_report_responses() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b[5n");

        assert_eq!(terminal.take_pending_responses(), b"\x1b[0n".to_vec());
        assert!(terminal.take_pending_responses().is_empty());
    }

    #[test]
    fn queues_primary_device_attributes_response() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b[c");

        assert_eq!(terminal.take_pending_responses(), b"\x1b[?1;2c".to_vec());
    }

    #[test]
    fn queues_secondary_device_attributes_response() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b[>0c");

        assert_eq!(terminal.take_pending_responses(), b"\x1b[>0;0;0c".to_vec());
    }

    #[test]
    fn queues_osc52_clipboard_write() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b]52;c;aGVsbG8=\x07");

        assert_eq!(
            terminal.take_pending_clipboard_writes(),
            vec!["hello".to_string()]
        );
        assert!(terminal.take_pending_clipboard_writes().is_empty());
    }

    #[test]
    fn denies_osc52_clipboard_query_without_readback() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b]52;c;?\x07");

        assert_eq!(
            terminal.take_pending_responses(),
            b"\x1b]52;c;\x07".to_vec()
        );
        assert!(terminal.take_pending_clipboard_writes().is_empty());
    }

    #[test]
    fn queues_osc_title_write() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b]2;minimal terminal\x07");

        assert_eq!(
            terminal.take_pending_title_writes(),
            vec!["minimal terminal".to_string()]
        );
        assert!(terminal.take_pending_title_writes().is_empty());
    }

    #[test]
    fn renders_dec_special_graphics_line_drawing() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b(0lqk\r\nx x\r\nmqj\x1b(B ascii");

        let snapshot = terminal.snapshot(4);
        assert_eq!(snapshot.lines[0], "┌─┐");
        assert_eq!(snapshot.lines[1], "│ │");
        assert_eq!(snapshot.lines[2], "└─┘ ascii");
    }

    #[test]
    fn renders_g1_dec_special_graphics_with_locking_shift() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b)0\x0elqk\x0f ascii");

        assert_eq!(terminal.snapshot(4).lines[0], "┌─┐ ascii");
    }

    #[test]
    fn renders_g2_and_g3_dec_special_graphics_with_locking_shift() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b*0\x1bnlqk\x0f\r\n\x1b+0\x1bomqj\x0f ascii");

        let snapshot = terminal.snapshot(4);
        assert_eq!(snapshot.lines[0], "┌─┐");
        assert_eq!(snapshot.lines[1], "└─┘ ascii");
    }

    #[test]
    fn renders_g2_and_g3_dec_special_graphics_with_single_shift() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b*0\x1b+0\x1bNlx \x1bOmx");

        assert_eq!(terminal.snapshot(4).lines[0], "┌x └x");
    }

    #[test]
    fn renders_right_side_g_sets_with_locking_shift() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b)0\x1b~\xc3\xb1q \x1b*0\x1b}\xc3\xac \x1b+A\x1b|\xc2\xa3");

        assert_eq!(terminal.snapshot(4).lines[0], "─q ┌ £");
    }

    #[test]
    fn renders_british_nrcs_charset() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b(A#x\x1b(B# \x1b*A\x1bN#x");

        assert_eq!(terminal.snapshot(4).lines[0], "£x# £x");
    }

    #[test]
    fn renders_german_nrcs_charset() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b(K@[\\] {|}~\x1b(B@");

        assert_eq!(terminal.snapshot(4).lines[0], "§ÄÖÜ äöüß@");
    }

    #[test]
    fn renders_finnish_nrcs_charset() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b(C[\\]^`{|}~\x1b(B[");

        assert_eq!(terminal.snapshot(4).lines[0], "ÄÖÅÜéäöåü[");
    }

    #[test]
    fn renders_french_nrcs_charset() {
        let mut terminal = TerminalState::new(4, 20);

        terminal.append_bytes(b"\x1b(R#@[\\] {|}~\x1b(B#");

        assert_eq!(terminal.snapshot(4).lines[0], "£à°ç§ éùè¨#");
    }

    #[test]
    fn queues_cursor_position_report_responses() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b[2;4H\x1b[6n");

        assert_eq!(terminal.take_pending_responses(), b"\x1b[2;4R".to_vec());
    }

    #[test]
    fn combined_snapshot_spans_scrollback_and_live_screen() {
        let mut terminal = TerminalState::new(3, 10);
        terminal.append_bytes(b"one\ntwo\nthree\nfour");

        let snapshot = terminal.combined_snapshot(0, 3);
        assert_eq!(snapshot.lines, vec!["two", "three", "four"]);
        assert_eq!(snapshot.viewport_start_absolute_row, 1);

        let snapshot = terminal.combined_snapshot(1, 3);
        assert_eq!(snapshot.lines, vec!["one", "two", "three"]);
        assert_eq!(snapshot.viewport_start_absolute_row, 0);
    }

    #[test]
    fn tracks_cursor_style_mode() {
        let mut terminal = TerminalState::new(4, 10);

        terminal.append_bytes(b"\x1b[6 q");
        assert_eq!(terminal.snapshot(4).modes.cursor_style, CursorStyle::Bar);

        terminal.append_bytes(b"\x1b[4 q");
        assert_eq!(
            terminal.snapshot(4).modes.cursor_style,
            CursorStyle::Underline
        );

        terminal.append_bytes(b"\x1b[2 q");
        assert_eq!(terminal.snapshot(4).modes.cursor_style, CursorStyle::Block);
    }
}
