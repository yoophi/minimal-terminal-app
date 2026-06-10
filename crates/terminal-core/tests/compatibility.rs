use terminal_core::{Color, Cursor, CursorStyle, Style, TerminalState};

#[test]
fn basic_terminal_sequences_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 16);

    terminal.append_bytes(b"abc\tz\rxy\nline2\x1b[1A\x1b[3G!");

    let snapshot = terminal.snapshot(4);
    assert_eq!(snapshot.lines[0], "xy!     z");
    assert_eq!(snapshot.lines[1], "line2");
    assert_eq!(snapshot.cursor, Cursor::new(0, 3));
}

#[test]
fn dec_special_graphics_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 20);

    terminal.append_bytes(b"\x1b(0lqk\r\nx x\r\nmqj\x1b(B ascii");

    assert_eq!(
        terminal.snapshot(4).lines,
        vec!["┌─┐", "│ │", "└─┘ ascii", ""]
    );
}

#[test]
fn iso_latin1_supplemental_charset_has_core_evidence() {
    let mut terminal = TerminalState::new(4, 20);

    terminal.append_bytes(b"\x1b-A\x1b~\xc2\xa1\xc2\xa3\xc2\xa4\xc3\xbf");

    assert_eq!(terminal.snapshot(4).lines[0], "¡£¤ÿ");
}

#[test]
fn style_sequences_have_snapshot_evidence() {
    let mut terminal = TerminalState::new(3, 32);

    terminal.append_bytes(b"\x1b[1;4;31mred\x1b[0m \x1b[38;5;196midx\x1b[0m \x1b[48;2;1;2;3mbg");

    let snapshot = terminal.snapshot(3);
    assert_eq!(snapshot.lines[0], "red idx bg");

    let spans = &snapshot.styled_lines[0].spans;
    assert_eq!(
        spans[0].style,
        Style {
            foreground: Some(Color::Indexed(1)),
            bold: true,
            underline: true,
            ..Style::default()
        }
    );
    assert_eq!(spans[2].style.foreground, Some(Color::Indexed(196)));
    assert_eq!(spans[4].style.background, Some(Color::Rgb(1, 2, 3)));
}

#[test]
fn tui_private_modes_and_editing_sequences_have_core_evidence() {
    let mut terminal = TerminalState::new(5, 16);

    terminal.append_bytes(b"\x1b[?25l\x1b[?2004h\x1b[?1h\x1b=");
    let snapshot = terminal.snapshot(5);
    assert!(!snapshot.modes.cursor_visible);
    assert!(snapshot.modes.bracketed_paste);
    assert!(snapshot.modes.application_cursor_keys);
    assert!(snapshot.modes.application_keypad);

    terminal.append_bytes(b"\x1b[?1000h\x1b[?1006h");
    let snapshot = terminal.snapshot(5);
    assert!(snapshot.modes.mouse_reporting);
    assert!(snapshot.modes.sgr_mouse);

    terminal.append_bytes(b"abcdef\r\x1b[2C\x1b[2@\r\x1b[2C\x1b[3P");
    assert_eq!(terminal.snapshot(5).lines[0], "abdef");

    terminal.append_bytes(b"\x1b[2;4r\x1b[4;1H\nx");
    assert_eq!(terminal.snapshot(5).lines, vec!["abdef", "", "", "x", ""]);
}

#[test]
fn device_status_reports_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b[5n\x1b[3;6H\x1b[6n");

    assert_eq!(
        terminal.take_pending_responses(),
        b"\x1b[0n\x1b[3;6R".to_vec()
    );
}

#[test]
fn primary_device_attributes_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b[c");

    assert_eq!(terminal.take_pending_responses(), b"\x1b[?1;2c".to_vec());
}

#[test]
fn secondary_device_attributes_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b[>c");

    assert_eq!(terminal.take_pending_responses(), b"\x1b[>0;0;0c".to_vec());
}

#[test]
fn osc52_clipboard_write_has_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b]52;c;aGVsbG8=\x07");

    assert_eq!(
        terminal.take_pending_clipboard_writes(),
        vec!["hello".to_string()]
    );
}

#[test]
fn osc52_clipboard_query_is_denied_without_readback() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b]52;c;?\x07");

    assert_eq!(
        terminal.take_pending_responses(),
        b"\x1b]52;c;\x07".to_vec()
    );
    assert!(terminal.take_pending_clipboard_writes().is_empty());
}

#[test]
fn osc_title_write_has_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b]2;minimal terminal\x07");

    assert_eq!(
        terminal.take_pending_title_writes(),
        vec!["minimal terminal".to_string()]
    );
}

#[test]
fn cursor_style_sequences_have_core_evidence() {
    let mut terminal = TerminalState::new(4, 10);

    terminal.append_bytes(b"\x1b[6 q");
    assert_eq!(terminal.snapshot(4).modes.cursor_style, CursorStyle::Bar);

    terminal.append_bytes(b"\x1b[4 q");
    assert_eq!(
        terminal.snapshot(4).modes.cursor_style,
        CursorStyle::Underline
    );
}
