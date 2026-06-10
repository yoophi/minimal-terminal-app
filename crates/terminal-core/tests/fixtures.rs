use terminal_core::{Color, Cursor, Style, TerminalState};

#[derive(Debug)]
struct Fixture<'a> {
    name: &'a str,
    input: &'a [u8],
    rows: usize,
    cols: usize,
    expected_lines: &'a [&'a str],
    expected_cursor: Cursor,
}

#[test]
fn parser_golden_fixtures() {
    let fixtures = [
        Fixture {
            name: "plain text",
            input: b"hello",
            rows: 3,
            cols: 16,
            expected_lines: &["hello", "", ""],
            expected_cursor: Cursor::new(0, 5),
        },
        Fixture {
            name: "carriage return rewrite",
            input: b"prompt\rnext",
            rows: 3,
            cols: 16,
            expected_lines: &["nextpt", "", ""],
            expected_cursor: Cursor::new(0, 4),
        },
        Fixture {
            name: "cursor movement",
            input: b"abc\x1b[1D!",
            rows: 3,
            cols: 16,
            expected_lines: &["ab!", "", ""],
            expected_cursor: Cursor::new(0, 3),
        },
        Fixture {
            name: "clear line",
            input: b"old prompt\r\x1b[Knew",
            rows: 3,
            cols: 16,
            expected_lines: &["new", "", ""],
            expected_cursor: Cursor::new(0, 3),
        },
        Fixture {
            name: "osc skip",
            input: b"\x1b]0;title\x07x",
            rows: 3,
            cols: 16,
            expected_lines: &["x", "", ""],
            expected_cursor: Cursor::new(0, 1),
        },
        Fixture {
            name: "malformed escape recovery",
            input: b"\x1b[9999Dleft",
            rows: 3,
            cols: 16,
            expected_lines: &["left", "", ""],
            expected_cursor: Cursor::new(0, 4),
        },
    ];

    for fixture in fixtures {
        let mut terminal = TerminalState::new(fixture.rows, fixture.cols);
        terminal.append_bytes(fixture.input);

        let snapshot = terminal.snapshot(fixture.rows);
        assert_eq!(
            snapshot.lines, fixture.expected_lines,
            "fixture '{}' lines differ",
            fixture.name
        );
        assert_eq!(
            snapshot.cursor, fixture.expected_cursor,
            "fixture '{}' cursor differs",
            fixture.name
        );
    }
}

#[test]
fn sgr_golden_fixture() {
    let mut terminal = TerminalState::new(3, 32);
    terminal.append_bytes(b"\x1b[1;31mred\x1b[0m \x1b[38;5;196midx\x1b[0m \x1b[38;2;1;2;3mrgb");

    let snapshot = terminal.snapshot(3);
    assert_eq!(snapshot.lines[0], "red idx rgb");

    let spans = &snapshot.styled_lines[0].spans;
    assert_eq!(spans[0].text, "red");
    assert_eq!(
        spans[0].style,
        Style {
            foreground: Some(Color::Indexed(1)),
            bold: true,
            ..Style::default()
        }
    );
    assert_eq!(spans[1].text, " ");
    assert_eq!(spans[1].style, Style::default());
    assert_eq!(spans[2].text, "idx");
    assert_eq!(spans[2].style.foreground, Some(Color::Indexed(196)));
    assert_eq!(spans[3].text, " ");
    assert_eq!(spans[3].style, Style::default());
    assert_eq!(spans[4].text, "rgb");
    assert_eq!(spans[4].style.foreground, Some(Color::Rgb(1, 2, 3)));
}

#[test]
fn alternate_screen_golden_fixture() {
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
