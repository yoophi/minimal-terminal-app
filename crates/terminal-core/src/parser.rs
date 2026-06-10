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
    CursorPosition { row: usize, col: usize },
    CursorUp(usize),
    CursorDown(usize),
    CursorRight(usize),
    CursorLeft(usize),
    CursorColumn(usize),
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

#[derive(Clone, Debug, Default)]
pub(crate) struct Parser {
    state: ParserState,
    csi: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ParserState {
    #[default]
    Ground,
    Escape,
    Csi,
    Osc,
    OscEscape,
    Charset,
}

impl Parser {
    pub(crate) fn advance(&mut self, ch: char) -> Option<Action> {
        match self.state {
            ParserState::Ground => self.advance_ground(ch),
            ParserState::Escape => self.advance_escape(ch),
            ParserState::Csi => self.advance_csi(ch),
            ParserState::Osc => self.advance_osc(ch),
            ParserState::OscEscape => self.advance_osc_escape(ch),
            ParserState::Charset => self.advance_charset(ch),
        }
    }

    fn advance_ground(&mut self, ch: char) -> Option<Action> {
        match ch {
            '\u{1b}' => {
                self.state = ParserState::Escape;
                None
            }
            '\r' => Some(Action::CarriageReturn),
            '\n' => Some(Action::Newline),
            '\u{08}' | '\u{7f}' => Some(Action::Backspace),
            '\t' => Some(Action::Tab),
            ch if ch.is_control() => Some(Action::Ignore),
            ch => Some(Action::Print(ch)),
        }
    }

    fn advance_escape(&mut self, ch: char) -> Option<Action> {
        match ch {
            '[' => {
                self.csi.clear();
                self.state = ParserState::Csi;
                None
            }
            ']' => {
                self.state = ParserState::Osc;
                None
            }
            '(' | ')' | '*' | '+' => {
                self.state = ParserState::Charset;
                None
            }
            '7' => {
                self.state = ParserState::Ground;
                Some(Action::SaveCursor)
            }
            '8' => {
                self.state = ParserState::Ground;
                Some(Action::RestoreCursor)
            }
            _ => {
                self.state = ParserState::Ground;
                Some(Action::Ignore)
            }
        }
    }

    fn advance_csi(&mut self, ch: char) -> Option<Action> {
        if ('@'..='~').contains(&ch) {
            let action = parse_csi(&self.csi, ch);
            self.csi.clear();
            self.state = ParserState::Ground;
            Some(action)
        } else {
            self.csi.push(ch);
            None
        }
    }

    fn advance_osc(&mut self, ch: char) -> Option<Action> {
        match ch {
            '\u{07}' => self.state = ParserState::Ground,
            '\u{1b}' => self.state = ParserState::OscEscape,
            _ => {}
        }
        None
    }

    fn advance_osc_escape(&mut self, ch: char) -> Option<Action> {
        self.state = if ch == '\\' {
            ParserState::Ground
        } else {
            ParserState::Osc
        };
        None
    }

    fn advance_charset(&mut self, _ch: char) -> Option<Action> {
        self.state = ParserState::Ground;
        None
    }
}

fn parse_csi(params: &str, final_byte: char) -> Action {
    if params.starts_with('?') {
        return parse_private_csi(params, final_byte);
    }

    let numbers = parse_numbers(params);
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
        'm' => Action::Ignore,
        _ => Action::Ignore,
    }
}

fn parse_private_csi(params: &str, final_byte: char) -> Action {
    let numbers = parse_numbers(params);
    match final_byte {
        'h' if contains_any(&numbers, &[47, 1047, 1049]) => Action::EnterAlternateScreen,
        'l' if contains_any(&numbers, &[47, 1047, 1049]) => Action::ExitAlternateScreen,
        'h' | 'l' => Action::Ignore,
        _ => Action::Ignore,
    }
}

fn parse_numbers(params: &str) -> Vec<usize> {
    params
        .trim_start_matches('?')
        .split(';')
        .filter_map(|part| {
            if part.is_empty() {
                Some(0)
            } else {
                part.parse().ok()
            }
        })
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
        assert_eq!(parser.advance('2'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('0'), None);
        assert_eq!(parser.advance('4'), None);
        assert_eq!(parser.advance('h'), Some(Action::Ignore));
    }
}
