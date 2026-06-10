use terminal_core::{Color, Cursor, Style, TerminalState};

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

    terminal.append_bytes(b"\x1b[?25l\x1b[?2004h\x1b[?1h");
    let snapshot = terminal.snapshot(5);
    assert!(!snapshot.modes.cursor_visible);
    assert!(snapshot.modes.bracketed_paste);
    assert!(snapshot.modes.application_cursor_keys);

    terminal.append_bytes(b"abcdef\r\x1b[2C\x1b[2@\r\x1b[2C\x1b[3P");
    assert_eq!(terminal.snapshot(5).lines[0], "abdef");

    terminal.append_bytes(b"\x1b[2;4r\x1b[4;1H\nx");
    assert_eq!(terminal.snapshot(5).lines, vec!["abdef", "", "", "x", ""]);
}

