use crate::style::{Color, Style};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Action {
    Print(char),
    CarriageReturn,
    Newline,
    Backspace,
    Tab,
    ClearLine(LineClearMode),
    ClearScreen(ScreenClearMode),
    SaveCursor,
    RestoreCursor,
    EnterAlternateScreen,
    ExitAlternateScreen,
    SetApplicationCursorKeys(bool),
    SetBracketedPaste(bool),
    SetCursorVisible(bool),
    CursorPosition { row: usize, col: usize },
    CursorUp(usize),
    CursorDown(usize),
    CursorRight(usize),
    CursorLeft(usize),
    CursorColumn(usize),
    SetGraphicRendition(Vec<usize>),
    Ignore,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineClearMode {
    FromCursor,
    ToCursor,
    Entire,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ScreenClearMode {
    FromCursor,
    ToCursor,
    Entire,
}

#[derive(Default)]
pub(crate) struct Parser {
    parser: vte::Parser,
}

impl Parser {
    #[cfg(test)]
    pub(crate) fn advance(&mut self, ch: char) -> Option<Action> {
        let mut bytes = [0; 4];
        let encoded = ch.encode_utf8(&mut bytes);
        self.advance_bytes(encoded.as_bytes()).pop()
    }

    pub(crate) fn advance_bytes(&mut self, bytes: &[u8]) -> Vec<Action> {
        let mut performer = ActionCollector::default();
        self.parser.advance(&mut performer, bytes);
        performer.actions
    }
}

#[derive(Debug, Default)]
struct ActionCollector {
    actions: Vec<Action>,
}

impl vte::Perform for ActionCollector {
    fn print(&mut self, ch: char) {
        let action = match ch {
            '\u{08}' | '\u{7f}' => Action::Backspace,
            ch if ch.is_control() => Action::Ignore,
            ch => Action::Print(ch),
        };
        self.actions.push(action);
    }

    fn execute(&mut self, byte: u8) {
        let action = match byte {
            b'\r' => Action::CarriageReturn,
            b'\n' => Action::Newline,
            0x08 | 0x7f => Action::Backspace,
            b'\t' => Action::Tab,
            _ => Action::Ignore,
        };
        self.actions.push(action);
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        if ignore {
            self.actions.push(Action::Ignore);
            return;
        }

        self.actions
            .push(parse_csi(&params_to_numbers(params), intermediates, action));
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        if ignore {
            self.actions.push(Action::Ignore);
            return;
        }

        let action = match (intermediates, byte) {
            ([], b'7') => Action::SaveCursor,
            ([], b'8') => Action::RestoreCursor,
            // Character set designations are parsed but ignored for now.
            ([b'(' | b')' | b'*' | b'+'], _) => return,
            _ => Action::Ignore,
        };
        self.actions.push(action);
    }
}

fn parse_csi(numbers: &[usize], intermediates: &[u8], final_byte: char) -> Action {
    if intermediates == b"?" {
        return parse_private_csi(numbers, final_byte);
    }

    match final_byte {
        'A' => Action::CursorUp(first_or_default(&numbers, 1)),
        'B' => Action::CursorDown(first_or_default(&numbers, 1)),
        'C' => Action::CursorRight(first_or_default(&numbers, 1)),
        'D' => Action::CursorLeft(first_or_default(&numbers, 1)),
        'G' => Action::CursorColumn(first_or_default(&numbers, 1).saturating_sub(1)),
        'H' | 'f' => Action::CursorPosition {
            row: first_or_default(&numbers, 1).saturating_sub(1),
            col: numbers.get(1).copied().unwrap_or(1).saturating_sub(1),
        },
        'J' => match first_or_default(&numbers, 0) {
            0 => Action::ClearScreen(ScreenClearMode::FromCursor),
            1 => Action::ClearScreen(ScreenClearMode::ToCursor),
            2 | 3 => Action::ClearScreen(ScreenClearMode::Entire),
            _ => Action::Ignore,
        },
        'K' => match first_or_default(&numbers, 0) {
            0 => Action::ClearLine(LineClearMode::FromCursor),
            1 => Action::ClearLine(LineClearMode::ToCursor),
            2 => Action::ClearLine(LineClearMode::Entire),
            _ => Action::Ignore,
        },
        's' => Action::SaveCursor,
        'u' => Action::RestoreCursor,
        'm' => Action::SetGraphicRendition(numbers.to_vec()),
        _ => Action::Ignore,
    }
}

fn parse_private_csi(numbers: &[usize], final_byte: char) -> Action {
    match final_byte {
        'h' if contains_any(&numbers, &[1]) => Action::SetApplicationCursorKeys(true),
        'l' if contains_any(&numbers, &[1]) => Action::SetApplicationCursorKeys(false),
        'h' if contains_any(&numbers, &[25]) => Action::SetCursorVisible(true),
        'l' if contains_any(&numbers, &[25]) => Action::SetCursorVisible(false),
        'h' if contains_any(&numbers, &[47, 1047, 1049]) => Action::EnterAlternateScreen,
        'l' if contains_any(&numbers, &[47, 1047, 1049]) => Action::ExitAlternateScreen,
        'h' if contains_any(&numbers, &[2004]) => Action::SetBracketedPaste(true),
        'l' if contains_any(&numbers, &[2004]) => Action::SetBracketedPaste(false),
        'h' | 'l' => Action::Ignore,
        _ => Action::Ignore,
    }
}

fn params_to_numbers(params: &vte::Params) -> Vec<usize> {
    params
        .iter()
        .map(|param| param.first().copied().unwrap_or(0) as usize)
        .collect()
}

fn first_or_default(numbers: &[usize], default: usize) -> usize {
    match numbers.first().copied() {
        Some(0) | None => default,
        Some(value) => value,
    }
}

fn contains_any(numbers: &[usize], targets: &[usize]) -> bool {
    numbers.iter().any(|number| targets.contains(number))
}

pub(crate) fn apply_sgr(style: &mut Style, numbers: &[usize]) {
    let numbers = if numbers.is_empty() {
        &[0][..]
    } else {
        numbers
    };
    let mut index = 0;

    while index < numbers.len() {
        match numbers[index] {
            0 => style.reset(),
            1 => style.bold = true,
            3 => style.italic = true,
            4 => style.underline = true,
            7 => style.inverse = true,
            22 => style.bold = false,
            23 => style.italic = false,
            24 => style.underline = false,
            27 => style.inverse = false,
            30..=37 => style.set_foreground(Some(Color::Indexed((numbers[index] - 30) as u8))),
            39 => style.set_foreground(None),
            40..=47 => style.set_background(Some(Color::Indexed((numbers[index] - 40) as u8))),
            49 => style.set_background(None),
            90..=97 => style.set_foreground(Some(Color::Indexed((numbers[index] - 90 + 8) as u8))),
            100..=107 => {
                style.set_background(Some(Color::Indexed((numbers[index] - 100 + 8) as u8)))
            }
            38 | 48 => {
                let is_foreground = numbers[index] == 38;
                if let Some((color, consumed)) = parse_extended_color(&numbers[index + 1..]) {
                    if is_foreground {
                        style.set_foreground(Some(color));
                    } else {
                        style.set_background(Some(color));
                    }
                    index += consumed;
                }
            }
            _ => {}
        }

        index += 1;
    }
}

fn parse_extended_color(numbers: &[usize]) -> Option<(Color, usize)> {
    match numbers {
        [5, color, ..] => Some((Color::Indexed((*color).min(255) as u8), 2)),
        [2, red, green, blue, ..] => Some((
            Color::Rgb(
                (*red).min(255) as u8,
                (*green).min(255) as u8,
                (*blue).min(255) as u8,
            ),
            4,
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, LineClearMode, Parser, ScreenClearMode};

    #[test]
    fn parses_cursor_position() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(parser.advance(';'), None);
        assert_eq!(parser.advance('5'), None);
        assert_eq!(
            parser.advance('H'),
            Some(Action::CursorPosition { row: 0, col: 4 })
        );
    }

    #[test]
    fn skips_osc_until_bel() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance(']'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance(';'), None);
        assert_eq!(parser.advance('t'), None);
        assert_eq!(parser.advance('i'), None);
        assert_eq!(parser.advance('t'), None);
        assert_eq!(parser.advance('l'), None);
        assert_eq!(parser.advance('e'), None);
        assert_eq!(parser.advance('\u{07}'), None);
        assert_eq!(parser.advance('x'), Some(Action::Print('x')));
    }

    #[test]
    fn skips_charset_designation() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('('), None);
        assert_eq!(parser.advance('B'), None);
        assert_eq!(parser.advance('x'), Some(Action::Print('x')));
    }

    #[test]
    fn parses_erase_modes() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('2'), None);
        assert_eq!(
            parser.advance('K'),
            Some(Action::ClearLine(LineClearMode::Entire))
        );

        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(
            parser.advance('J'),
            Some(Action::ClearScreen(ScreenClearMode::ToCursor))
        );
    }

    #[test]
    fn parses_save_and_restore_cursor() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('7'), Some(Action::SaveCursor));
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('8'), Some(Action::RestoreCursor));
    }

    #[test]
    fn parses_alternate_screen_private_modes() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('?'), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('4'), None);
        assert_eq!(parser.advance('9'), None);
        assert_eq!(parser.advance('h'), Some(Action::EnterAlternateScreen));

        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('?'), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('4'), None);
        assert_eq!(parser.advance('9'), None);
        assert_eq!(parser.advance('l'), Some(Action::ExitAlternateScreen));
    }

    #[test]
    fn ignores_other_private_modes() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('?'), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('h'), Some(Action::Ignore));
    }

    #[test]
    fn parses_tui_private_modes() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance_bytes(b"\x1b[?25l"), vec![Action::SetCursorVisible(false)]);
        assert_eq!(parser.advance_bytes(b"\x1b[?25h"), vec![Action::SetCursorVisible(true)]);
        assert_eq!(
            parser.advance_bytes(b"\x1b[?2004h"),
            vec![Action::SetBracketedPaste(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?1h"),
            vec![Action::SetApplicationCursorKeys(true)]
        );
    }

    #[test]
    fn parses_sgr_parameters() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('['), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(parser.advance(';'), None);
        assert_eq!(parser.advance('3'), None);
        assert_eq!(parser.advance('1'), None);
        assert_eq!(
            parser.advance('m'),
            Some(Action::SetGraphicRendition(vec![1, 31]))
        );
    }
}
