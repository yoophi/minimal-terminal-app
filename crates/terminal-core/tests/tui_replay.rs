use terminal_core::{Color, Cursor, CursorStyle, Style, TerminalState};

fn decode_fixture(input: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = input.trim_end_matches('\n').chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            let mut encoded = [0; 4];
            bytes.extend_from_slice(ch.encode_utf8(&mut encoded).as_bytes());
            continue;
        }

        match chars.next() {
            Some('x') if chars.next() == Some('1') && chars.next() == Some('b') => {
                bytes.push(0x1b);
            }
            Some('n') => bytes.push(b'\n'),
            Some('r') => bytes.push(b'\r'),
            Some(other) => {
                bytes.push(b'\\');
                let mut encoded = [0; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut encoded).as_bytes());
            }
            None => bytes.push(b'\\'),
        }
    }

    bytes
}

#[test]
fn less_replay_restores_main_screen() {
    let input = decode_fixture(include_str!("fixtures/tui/less_open_close.ansi"));
    let mut terminal = TerminalState::new(4, 20);

    terminal.append_bytes(&input);

    let snapshot = terminal.snapshot(4);
    assert_eq!(snapshot.lines[0], "shell");
    assert_eq!(snapshot.cursor, Cursor::new(0, 5));
    assert!(snapshot.modes.cursor_visible);
}

#[test]
fn vim_replay_restores_cursor_modes() {
    let input = decode_fixture(include_str!("fixtures/tui/vim_minimal.ansi"));
    let mut terminal = TerminalState::new(4, 24);

    terminal.append_bytes(&input);

    let snapshot = terminal.snapshot(4);
    assert_eq!(snapshot.lines[0], "prompt");
    assert_eq!(snapshot.cursor, Cursor::new(0, 6));
    assert!(snapshot.modes.cursor_visible);
    assert_eq!(snapshot.modes.cursor_style, CursorStyle::Block);
}

#[test]
fn top_replay_keeps_styled_redraw() {
    let input = decode_fixture(include_str!("fixtures/tui/top_minimal.ansi"));
    let mut terminal = TerminalState::new(4, 24);

    terminal.append_bytes(&input);

    let snapshot = terminal.snapshot(4);
    assert_eq!(snapshot.lines[0], "top");
    assert_eq!(snapshot.lines[1], "PID CPU");
    assert_eq!(
        snapshot.styled_lines[0].spans[0].style,
        Style {
            foreground: Some(Color::Indexed(2)),
            bold: true,
            ..Style::default()
        }
    );
}
