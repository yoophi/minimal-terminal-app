use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::cursor::CursorStyle;
use crate::style::{Color, Style};

const MAX_OSC52_DECODED_BYTES: usize = 1024 * 1024;
const MAX_OSC52_SELECTOR_BYTES: usize = 64;
const MAX_OSC_TITLE_BYTES: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Action {
    Print(char),
    CarriageReturn,
    Newline,
    Backspace,
    Tab,
    ClearLine(LineClearMode),
    ClearScreen(ScreenClearMode),
    InsertBlankChars(usize),
    DeleteChars(usize),
    EraseChars(usize),
    InsertLines(usize),
    DeleteLines(usize),
    SaveCursor,
    RestoreCursor,
    EnterAlternateScreen,
    ExitAlternateScreen,
    SetApplicationCursorKeys(bool),
    SetApplicationKeypad(bool),
    SetBracketedPaste(bool),
    SetCursorVisible(bool),
    SetCursorStyle(CursorStyle),
    SetMouseReporting(bool),
    SetSgrMouse(bool),
    PrimaryDeviceAttributes,
    SecondaryDeviceAttributes,
    SetClipboard(String),
    ClipboardQueryDenied(String),
    SetWindowTitle(String),
    DeviceStatusReport,
    CursorPositionReport,
    CursorPosition { row: usize, col: usize },
    CursorUp(usize),
    CursorDown(usize),
    CursorRight(usize),
    CursorLeft(usize),
    CursorColumn(usize),
    SetScrollRegion(Option<(usize, usize)>),
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

pub(crate) struct Parser {
    parser: vte::Parser,
    g0_charset: Charset,
    g1_charset: Charset,
    g2_charset: Charset,
    g3_charset: Charset,
    active_charset: ActiveCharset,
    active_right_charset: Option<ActiveCharset>,
    single_shift_charset: Option<ActiveCharset>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            parser: vte::Parser::default(),
            g0_charset: Charset::Ascii,
            g1_charset: Charset::Ascii,
            g2_charset: Charset::Ascii,
            g3_charset: Charset::Ascii,
            active_charset: ActiveCharset::G0,
            active_right_charset: None,
            single_shift_charset: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Charset {
    Ascii,
    British,
    DecCyrillic,
    DecGreekSupplemental,
    DecHebrewSupplemental,
    DecSupplementalGraphics,
    DecSpecialGraphics,
    DecTechnical,
    DecTurkishSupplemental,
    Dutch,
    Finnish,
    French,
    FrenchCanadian,
    German,
    Greek,
    Hebrew,
    Italian,
    JisKatakana,
    JisRoman,
    NorwegianDanish,
    Portuguese,
    Russian,
    SerboCroatian,
    Spanish,
    Swedish,
    Swiss,
    Turkish,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ActiveCharset {
    G0,
    G1,
    G2,
    G3,
}

impl Parser {
    #[cfg(test)]
    pub(crate) fn advance(&mut self, ch: char) -> Option<Action> {
        let mut bytes = [0; 4];
        let encoded = ch.encode_utf8(&mut bytes);
        self.advance_bytes(encoded.as_bytes()).pop()
    }

    pub(crate) fn advance_bytes(&mut self, bytes: &[u8]) -> Vec<Action> {
        let mut performer = ActionCollector {
            actions: Vec::new(),
            g0_charset: self.g0_charset,
            g1_charset: self.g1_charset,
            g2_charset: self.g2_charset,
            g3_charset: self.g3_charset,
            active_charset: self.active_charset,
            active_right_charset: self.active_right_charset,
            single_shift_charset: self.single_shift_charset,
        };
        self.parser.advance(&mut performer, bytes);
        self.g0_charset = performer.g0_charset;
        self.g1_charset = performer.g1_charset;
        self.g2_charset = performer.g2_charset;
        self.g3_charset = performer.g3_charset;
        self.active_charset = performer.active_charset;
        self.active_right_charset = performer.active_right_charset;
        self.single_shift_charset = performer.single_shift_charset;
        performer.actions
    }
}

#[derive(Debug)]
struct ActionCollector {
    actions: Vec<Action>,
    g0_charset: Charset,
    g1_charset: Charset,
    g2_charset: Charset,
    g3_charset: Charset,
    active_charset: ActiveCharset,
    active_right_charset: Option<ActiveCharset>,
    single_shift_charset: Option<ActiveCharset>,
}

impl vte::Perform for ActionCollector {
    fn print(&mut self, ch: char) {
        let single_shift_charset = self.single_shift_charset.take();
        let is_right_side = single_shift_charset.is_none()
            && self.active_right_charset.is_some()
            && is_gr_printable(ch);
        let active_charset = if let Some(active_charset) = single_shift_charset {
            active_charset
        } else if is_right_side {
            self.active_right_charset.unwrap_or(self.active_charset)
        } else {
            self.active_charset
        };
        let charset = match active_charset {
            ActiveCharset::G0 => self.g0_charset,
            ActiveCharset::G1 => self.g1_charset,
            ActiveCharset::G2 => self.g2_charset,
            ActiveCharset::G3 => self.g3_charset,
        };
        let mapped_ch = if is_right_side {
            gr_to_gl_printable(ch)
        } else {
            ch
        };
        let action = match ch {
            '\u{08}' | '\u{7f}' => Action::Backspace,
            ch if ch.is_control() => Action::Ignore,
            _ => Action::Print(map_printable_char(mapped_ch, charset)),
        };
        self.actions.push(action);
    }

    fn execute(&mut self, byte: u8) {
        let action = match byte {
            b'\r' => Action::CarriageReturn,
            b'\n' => Action::Newline,
            0x08 | 0x7f => Action::Backspace,
            b'\t' => Action::Tab,
            0x0e => {
                self.active_charset = ActiveCharset::G1;
                return;
            }
            0x0f => {
                self.active_charset = ActiveCharset::G0;
                return;
            }
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
            ([], b'=') => Action::SetApplicationKeypad(true),
            ([], b'>') => Action::SetApplicationKeypad(false),
            ([], b'n') => {
                self.active_charset = ActiveCharset::G2;
                return;
            }
            ([], b'o') => {
                self.active_charset = ActiveCharset::G3;
                return;
            }
            ([], b'~') => {
                self.active_right_charset = Some(ActiveCharset::G1);
                return;
            }
            ([], b'}') => {
                self.active_right_charset = Some(ActiveCharset::G2);
                return;
            }
            ([], b'|') => {
                self.active_right_charset = Some(ActiveCharset::G3);
                return;
            }
            ([], b'N') => {
                self.single_shift_charset = Some(ActiveCharset::G2);
                return;
            }
            ([], b'O') => {
                self.single_shift_charset = Some(ActiveCharset::G3);
                return;
            }
            ([b'('], b'0') => {
                self.g0_charset = Charset::DecSpecialGraphics;
                return;
            }
            ([b'('], b'A') => {
                self.g0_charset = Charset::British;
                return;
            }
            ([b'('], b'B') => {
                self.g0_charset = Charset::Ascii;
                return;
            }
            ([b'(', b'%'], b'5') => {
                self.g0_charset = Charset::DecSupplementalGraphics;
                return;
            }
            ([b'('], b'>') => {
                self.g0_charset = Charset::DecTechnical;
                return;
            }
            ([b'('], b'4') => {
                self.g0_charset = Charset::Dutch;
                return;
            }
            ([b'('], b'C') => {
                self.g0_charset = Charset::Finnish;
                return;
            }
            ([b'('], b'H' | b'7') => {
                self.g0_charset = Charset::Swedish;
                return;
            }
            ([b'('], b'Q' | b'9') => {
                self.g0_charset = Charset::FrenchCanadian;
                return;
            }
            ([b'('], b'R' | b'f') => {
                self.g0_charset = Charset::French;
                return;
            }
            ([b'('], b'K') => {
                self.g0_charset = Charset::German;
                return;
            }
            ([b'(', b'"'], b'>') => {
                self.g0_charset = Charset::Greek;
                return;
            }
            ([b'(', b'"'], b'?') => {
                self.g0_charset = Charset::DecGreekSupplemental;
                return;
            }
            ([b'(', b'"'], b'4') => {
                self.g0_charset = Charset::DecHebrewSupplemental;
                return;
            }
            ([b'(', b'%'], b'=') => {
                self.g0_charset = Charset::Hebrew;
                return;
            }
            ([b'(', b'%'], b'2') => {
                self.g0_charset = Charset::Turkish;
                return;
            }
            ([b'(', b'%'], b'0') => {
                self.g0_charset = Charset::DecTurkishSupplemental;
                return;
            }
            ([b'(', b'&'], b'5') => {
                self.g0_charset = Charset::Russian;
                return;
            }
            ([b'(', b'&'], b'4') => {
                self.g0_charset = Charset::DecCyrillic;
                return;
            }
            ([b'(', b'%'], b'3') => {
                self.g0_charset = Charset::SerboCroatian;
                return;
            }
            ([b'('], b'Y') => {
                self.g0_charset = Charset::Italian;
                return;
            }
            ([b'('], b'I') => {
                self.g0_charset = Charset::JisKatakana;
                return;
            }
            ([b'('], b'J') => {
                self.g0_charset = Charset::JisRoman;
                return;
            }
            ([b'('], b'Z') => {
                self.g0_charset = Charset::Spanish;
                return;
            }
            ([b'('], b'=') => {
                self.g0_charset = Charset::Swiss;
                return;
            }
            ([b'(', b'%'], b'6') => {
                self.g0_charset = Charset::Portuguese;
                return;
            }
            ([b'('], b'`' | b'E' | b'6') => {
                self.g0_charset = Charset::NorwegianDanish;
                return;
            }
            ([b')'], b'0') => {
                self.g1_charset = Charset::DecSpecialGraphics;
                return;
            }
            ([b')'], b'A') => {
                self.g1_charset = Charset::British;
                return;
            }
            ([b')'], b'B') => {
                self.g1_charset = Charset::Ascii;
                return;
            }
            ([b')', b'%'], b'5') => {
                self.g1_charset = Charset::DecSupplementalGraphics;
                return;
            }
            ([b')'], b'>') => {
                self.g1_charset = Charset::DecTechnical;
                return;
            }
            ([b')'], b'4') => {
                self.g1_charset = Charset::Dutch;
                return;
            }
            ([b')'], b'C') => {
                self.g1_charset = Charset::Finnish;
                return;
            }
            ([b')'], b'H' | b'7') => {
                self.g1_charset = Charset::Swedish;
                return;
            }
            ([b')'], b'Q' | b'9') => {
                self.g1_charset = Charset::FrenchCanadian;
                return;
            }
            ([b')'], b'R' | b'f') => {
                self.g1_charset = Charset::French;
                return;
            }
            ([b')'], b'K') => {
                self.g1_charset = Charset::German;
                return;
            }
            ([b')', b'"'], b'>') => {
                self.g1_charset = Charset::Greek;
                return;
            }
            ([b')', b'"'], b'?') => {
                self.g1_charset = Charset::DecGreekSupplemental;
                return;
            }
            ([b')', b'"'], b'4') => {
                self.g1_charset = Charset::DecHebrewSupplemental;
                return;
            }
            ([b')', b'%'], b'=') => {
                self.g1_charset = Charset::Hebrew;
                return;
            }
            ([b')', b'%'], b'2') => {
                self.g1_charset = Charset::Turkish;
                return;
            }
            ([b')', b'%'], b'0') => {
                self.g1_charset = Charset::DecTurkishSupplemental;
                return;
            }
            ([b')', b'&'], b'5') => {
                self.g1_charset = Charset::Russian;
                return;
            }
            ([b')', b'&'], b'4') => {
                self.g1_charset = Charset::DecCyrillic;
                return;
            }
            ([b')', b'%'], b'3') => {
                self.g1_charset = Charset::SerboCroatian;
                return;
            }
            ([b')'], b'Y') => {
                self.g1_charset = Charset::Italian;
                return;
            }
            ([b')'], b'I') => {
                self.g1_charset = Charset::JisKatakana;
                return;
            }
            ([b')'], b'J') => {
                self.g1_charset = Charset::JisRoman;
                return;
            }
            ([b')'], b'Z') => {
                self.g1_charset = Charset::Spanish;
                return;
            }
            ([b')'], b'=') => {
                self.g1_charset = Charset::Swiss;
                return;
            }
            ([b')', b'%'], b'6') => {
                self.g1_charset = Charset::Portuguese;
                return;
            }
            ([b')'], b'`' | b'E' | b'6') => {
                self.g1_charset = Charset::NorwegianDanish;
                return;
            }
            ([b'*'], b'0') => {
                self.g2_charset = Charset::DecSpecialGraphics;
                return;
            }
            ([b'*'], b'A') => {
                self.g2_charset = Charset::British;
                return;
            }
            ([b'*'], b'B') => {
                self.g2_charset = Charset::Ascii;
                return;
            }
            ([b'*', b'%'], b'5') => {
                self.g2_charset = Charset::DecSupplementalGraphics;
                return;
            }
            ([b'*'], b'>') => {
                self.g2_charset = Charset::DecTechnical;
                return;
            }
            ([b'*'], b'4') => {
                self.g2_charset = Charset::Dutch;
                return;
            }
            ([b'*'], b'C') => {
                self.g2_charset = Charset::Finnish;
                return;
            }
            ([b'*'], b'H' | b'7') => {
                self.g2_charset = Charset::Swedish;
                return;
            }
            ([b'*'], b'Q' | b'9') => {
                self.g2_charset = Charset::FrenchCanadian;
                return;
            }
            ([b'*'], b'R' | b'f') => {
                self.g2_charset = Charset::French;
                return;
            }
            ([b'*'], b'K') => {
                self.g2_charset = Charset::German;
                return;
            }
            ([b'*', b'"'], b'>') => {
                self.g2_charset = Charset::Greek;
                return;
            }
            ([b'*', b'"'], b'?') => {
                self.g2_charset = Charset::DecGreekSupplemental;
                return;
            }
            ([b'*', b'"'], b'4') => {
                self.g2_charset = Charset::DecHebrewSupplemental;
                return;
            }
            ([b'*', b'%'], b'=') => {
                self.g2_charset = Charset::Hebrew;
                return;
            }
            ([b'*', b'%'], b'2') => {
                self.g2_charset = Charset::Turkish;
                return;
            }
            ([b'*', b'%'], b'0') => {
                self.g2_charset = Charset::DecTurkishSupplemental;
                return;
            }
            ([b'*', b'&'], b'5') => {
                self.g2_charset = Charset::Russian;
                return;
            }
            ([b'*', b'&'], b'4') => {
                self.g2_charset = Charset::DecCyrillic;
                return;
            }
            ([b'*', b'%'], b'3') => {
                self.g2_charset = Charset::SerboCroatian;
                return;
            }
            ([b'*'], b'Y') => {
                self.g2_charset = Charset::Italian;
                return;
            }
            ([b'*'], b'I') => {
                self.g2_charset = Charset::JisKatakana;
                return;
            }
            ([b'*'], b'J') => {
                self.g2_charset = Charset::JisRoman;
                return;
            }
            ([b'*'], b'Z') => {
                self.g2_charset = Charset::Spanish;
                return;
            }
            ([b'*'], b'=') => {
                self.g2_charset = Charset::Swiss;
                return;
            }
            ([b'*', b'%'], b'6') => {
                self.g2_charset = Charset::Portuguese;
                return;
            }
            ([b'*'], b'`' | b'E' | b'6') => {
                self.g2_charset = Charset::NorwegianDanish;
                return;
            }
            ([b'+'], b'0') => {
                self.g3_charset = Charset::DecSpecialGraphics;
                return;
            }
            ([b'+'], b'A') => {
                self.g3_charset = Charset::British;
                return;
            }
            ([b'+'], b'B') => {
                self.g3_charset = Charset::Ascii;
                return;
            }
            ([b'+', b'%'], b'5') => {
                self.g3_charset = Charset::DecSupplementalGraphics;
                return;
            }
            ([b'+'], b'>') => {
                self.g3_charset = Charset::DecTechnical;
                return;
            }
            ([b'+'], b'4') => {
                self.g3_charset = Charset::Dutch;
                return;
            }
            ([b'+'], b'C') => {
                self.g3_charset = Charset::Finnish;
                return;
            }
            ([b'+'], b'H' | b'7') => {
                self.g3_charset = Charset::Swedish;
                return;
            }
            ([b'+'], b'Q' | b'9') => {
                self.g3_charset = Charset::FrenchCanadian;
                return;
            }
            ([b'+'], b'R' | b'f') => {
                self.g3_charset = Charset::French;
                return;
            }
            ([b'+'], b'K') => {
                self.g3_charset = Charset::German;
                return;
            }
            ([b'+', b'"'], b'>') => {
                self.g3_charset = Charset::Greek;
                return;
            }
            ([b'+', b'"'], b'?') => {
                self.g3_charset = Charset::DecGreekSupplemental;
                return;
            }
            ([b'+', b'"'], b'4') => {
                self.g3_charset = Charset::DecHebrewSupplemental;
                return;
            }
            ([b'+', b'%'], b'=') => {
                self.g3_charset = Charset::Hebrew;
                return;
            }
            ([b'+', b'%'], b'2') => {
                self.g3_charset = Charset::Turkish;
                return;
            }
            ([b'+', b'%'], b'0') => {
                self.g3_charset = Charset::DecTurkishSupplemental;
                return;
            }
            ([b'+', b'&'], b'5') => {
                self.g3_charset = Charset::Russian;
                return;
            }
            ([b'+', b'&'], b'4') => {
                self.g3_charset = Charset::DecCyrillic;
                return;
            }
            ([b'+', b'%'], b'3') => {
                self.g3_charset = Charset::SerboCroatian;
                return;
            }
            ([b'+'], b'Y') => {
                self.g3_charset = Charset::Italian;
                return;
            }
            ([b'+'], b'I') => {
                self.g3_charset = Charset::JisKatakana;
                return;
            }
            ([b'+'], b'J') => {
                self.g3_charset = Charset::JisRoman;
                return;
            }
            ([b'+'], b'Z') => {
                self.g3_charset = Charset::Spanish;
                return;
            }
            ([b'+'], b'=') => {
                self.g3_charset = Charset::Swiss;
                return;
            }
            ([b'+', b'%'], b'6') => {
                self.g3_charset = Charset::Portuguese;
                return;
            }
            ([b'+'], b'`' | b'E' | b'6') => {
                self.g3_charset = Charset::NorwegianDanish;
                return;
            }
            ([b'(' | b')' | b'*' | b'+'], _) => return,
            _ => Action::Ignore,
        };
        self.actions.push(action);
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        self.actions.push(parse_osc(params));
    }
}

fn map_printable_char(ch: char, charset: Charset) -> char {
    if charset == Charset::DecTurkishSupplemental {
        return match ch {
            '$' | '&' | ',' | '-' | '/' | '4' | '8' => '␦',
            '.' => 'İ',
            '>' => 'ı',
            'P' => 'Ğ',
            'W' => 'Œ',
            ']' => 'Ÿ',
            '^' => 'Ş',
            'p' => 'ğ',
            'w' => 'œ',
            '}' => 'ÿ',
            '~' => 'ş',
            '!'..='~' => char::from_u32(ch as u32 + 0x80).unwrap_or(ch),
            _ => ch,
        };
    }

    if charset == Charset::DecHebrewSupplemental {
        return match ch {
            '!' => '¡',
            '"' => '¢',
            '#' => '£',
            '$' | '&' | ',' | '-' | '.' | '/' | '4' | '8' | '>' | '@'..='_' | '{'..='~' => '␦',
            '%' => '¥',
            '\'' => '§',
            '(' => '¨',
            ')' => '©',
            '*' => '×',
            '+' => '«',
            '0' => '°',
            '1' => '±',
            '2' => '²',
            '3' => '³',
            '5' => 'µ',
            '6' => '¶',
            '7' => '·',
            '9' => '¹',
            ':' => '÷',
            ';' => '»',
            '<' => '¼',
            '=' => '½',
            '?' => '¿',
            '`' => 'א',
            'a' => 'ב',
            'b' => 'ג',
            'c' => 'ד',
            'd' => 'ה',
            'e' => 'ו',
            'f' => 'ז',
            'g' => 'ח',
            'h' => 'ט',
            'i' => 'י',
            'j' => 'ך',
            'k' => 'כ',
            'l' => 'ל',
            'm' => 'ם',
            'n' => 'מ',
            'o' => 'ן',
            'p' => 'נ',
            'q' => 'ס',
            'r' => 'ע',
            's' => 'ף',
            't' => 'פ',
            'u' => 'ץ',
            'v' => 'צ',
            'w' => 'ק',
            'x' => 'ר',
            'y' => 'ש',
            'z' => 'ת',
            _ => ch,
        };
    }

    if charset == Charset::DecGreekSupplemental {
        return match ch {
            '!' => '¡',
            '"' => '¢',
            '#' => '£',
            '$' | '&' | ',' | '-' | '.' | '/' | '4' | '8' | '>' | 'P' | '^' | 'p' | '~' => '␦',
            '%' => '¥',
            '\'' => '§',
            '(' => '¤',
            ')' => '©',
            '*' => 'ª',
            '+' => '«',
            '0' => '°',
            '1' => '±',
            '2' => '²',
            '3' => '³',
            '5' => 'µ',
            '6' => '¶',
            '7' => '·',
            '9' => '¹',
            ':' => 'º',
            ';' => '»',
            '<' => '¼',
            '=' => '½',
            '?' => '¿',
            '@' => 'ϊ',
            'A' => 'Α',
            'B' => 'Β',
            'C' => 'Γ',
            'D' => 'Δ',
            'E' => 'Ε',
            'F' => 'Ζ',
            'G' => 'Η',
            'H' => 'Θ',
            'I' => 'Ι',
            'J' => 'Κ',
            'K' => 'Λ',
            'L' => 'Μ',
            'M' => 'Ν',
            'N' => 'Ξ',
            'O' => 'Ο',
            'Q' => 'Π',
            'R' => 'Ρ',
            'S' => 'Σ',
            'T' => 'Τ',
            'U' => 'Υ',
            'V' => 'Φ',
            'W' => 'Χ',
            'X' => 'Ψ',
            'Y' => 'Ω',
            'Z' => 'ά',
            '[' => 'έ',
            '\\' => 'ή',
            ']' => 'ί',
            '_' => 'ό',
            '`' => 'ϋ',
            'a' => 'α',
            'b' => 'β',
            'c' => 'γ',
            'd' => 'δ',
            'e' => 'ε',
            'f' => 'ζ',
            'g' => 'η',
            'h' => 'θ',
            'i' => 'ι',
            'j' => 'κ',
            'k' => 'λ',
            'l' => 'μ',
            'm' => 'ν',
            'n' => 'ξ',
            'o' => 'ο',
            'q' => 'π',
            'r' => 'ρ',
            's' => 'σ',
            't' => 'τ',
            'u' => 'υ',
            'v' => 'φ',
            'w' => 'χ',
            'x' => 'ψ',
            'y' => 'ω',
            'z' => 'ς',
            '{' => 'ύ',
            '|' => 'ώ',
            '}' => '΄',
            _ => ch,
        };
    }

    if charset == Charset::DecCyrillic {
        return match ch {
            '!'..='?' => '␦',
            '@' => 'ю',
            'A' => 'а',
            'B' => 'б',
            'C' => 'ц',
            'D' => 'д',
            'E' => 'е',
            'F' => 'ф',
            'G' => 'г',
            'H' => 'х',
            'I' => 'и',
            'J' => 'й',
            'K' => 'к',
            'L' => 'л',
            'M' => 'м',
            'N' => 'н',
            'O' => 'о',
            'P' => 'п',
            'Q' => 'я',
            'R' => 'р',
            'S' => 'с',
            'T' => 'т',
            'U' => 'у',
            'V' => 'ж',
            'W' => 'в',
            'X' => 'ь',
            'Y' => 'ы',
            'Z' => 'з',
            '[' => 'ш',
            '\\' => 'э',
            ']' => 'щ',
            '^' => 'ч',
            '_' => 'ъ',
            '`' => 'Ю',
            'a' => 'А',
            'b' => 'Б',
            'c' => 'Ц',
            'd' => 'Д',
            'e' => 'Е',
            'f' => 'Ф',
            'g' => 'Г',
            'h' => 'Х',
            'i' => 'И',
            'j' => 'Й',
            'k' => 'К',
            'l' => 'Л',
            'm' => 'М',
            'n' => 'Н',
            'o' => 'О',
            'p' => 'П',
            'q' => 'Я',
            'r' => 'Р',
            's' => 'С',
            't' => 'Т',
            'u' => 'У',
            'v' => 'Ж',
            'w' => 'В',
            'x' => 'Ь',
            'y' => 'Ы',
            'z' => 'З',
            '{' => 'Ш',
            '|' => 'Э',
            '}' => 'Щ',
            '~' => 'Ч',
            _ => ch,
        };
    }

    if charset == Charset::DecTechnical {
        return match ch {
            '!' => '⎷',
            '"' => '┌',
            '#' => '─',
            '$' => '⌠',
            '%' => '⌡',
            '&' => '│',
            '\'' => '⎡',
            '(' => '⎣',
            ')' => '⎤',
            '*' => '⎦',
            '+' => '⎧',
            ',' => '⎩',
            '-' => '⎫',
            '.' => '⎭',
            '/' => '⎨',
            '0' => '⎬',
            '1'..=';' => '␦',
            '<' => '≤',
            '=' => '≠',
            '>' => '≥',
            '?' => '∫',
            '@' => '∴',
            'A' => '∝',
            'B' => '∞',
            'C' => '÷',
            'D' => 'Δ',
            'E' => '∇',
            'F' => 'Φ',
            'G' => 'Γ',
            'H' => '∼',
            'I' => '≃',
            'J' => 'Θ',
            'K' => '×',
            'L' => 'Λ',
            'M' => '⇔',
            'N' => '⇒',
            'O' => '≡',
            'P' => 'Π',
            'Q' => 'Ψ',
            'R' | 'T' | 'U' => '␦',
            'S' => 'Σ',
            'V' => '√',
            'W' => 'Ω',
            'X' => 'Ξ',
            'Y' => 'Υ',
            'Z' => '⊂',
            '[' => '⊃',
            '\\' => '∩',
            ']' => '∪',
            '^' => '∧',
            '_' => '∨',
            '`' => '¬',
            'a' => 'α',
            'b' => 'β',
            'c' => 'χ',
            'd' => 'δ',
            'e' => 'ε',
            'f' => 'φ',
            'g' => 'γ',
            'h' => 'η',
            'i' => 'ι',
            'j' => 'θ',
            'k' => 'κ',
            'l' => 'λ',
            'm' | 'u' => '␦',
            'n' => 'ν',
            'o' => '∂',
            'p' => 'π',
            'q' => 'ψ',
            'r' => 'ρ',
            's' => 'σ',
            't' => 'τ',
            'v' => 'ƒ',
            'w' => 'ω',
            'x' => 'ξ',
            'y' => 'υ',
            'z' => 'ζ',
            '{' => '←',
            '|' => '↑',
            '}' => '→',
            '~' => '↓',
            _ => ch,
        };
    }

    if charset == Charset::DecSupplementalGraphics {
        return match ch {
            '$' | '&' | ',' | '-' | '.' | '/' | '4' | '8' | '>' | 'P' | '^' | 'p' | '~' => '␦',
            '(' => '¤',
            'W' => 'Œ',
            ']' => 'Ÿ',
            '_' => '_',
            'w' => 'œ',
            '}' => 'ÿ',
            '!'..='~' => char::from_u32(ch as u32 + 0x80).unwrap_or(ch),
            _ => ch,
        };
    }

    if charset == Charset::British {
        return match ch {
            '#' => '£',
            _ => ch,
        };
    }

    if charset == Charset::German {
        return match ch {
            '@' => '§',
            '[' => 'Ä',
            '\\' => 'Ö',
            ']' => 'Ü',
            '{' => 'ä',
            '|' => 'ö',
            '}' => 'ü',
            '~' => 'ß',
            _ => ch,
        };
    }

    if charset == Charset::Dutch {
        return match ch {
            '#' => '£',
            '@' => '¾',
            '[' => 'ĳ',
            '\\' => '½',
            ']' => '|',
            '{' => '¨',
            '|' => 'ƒ',
            '}' => '¼',
            '~' => '´',
            _ => ch,
        };
    }

    if charset == Charset::Finnish {
        return match ch {
            '[' => 'Ä',
            '\\' => 'Ö',
            ']' => 'Å',
            '^' => 'Ü',
            '`' => 'é',
            '{' => 'ä',
            '|' => 'ö',
            '}' => 'å',
            '~' => 'ü',
            _ => ch,
        };
    }

    if charset == Charset::Swedish {
        return match ch {
            '@' => 'É',
            '[' => 'Ä',
            '\\' => 'Ö',
            ']' => 'Å',
            '^' => 'Ü',
            '`' => 'é',
            '{' => 'ä',
            '|' => 'ö',
            '}' => 'å',
            '~' => 'ü',
            _ => ch,
        };
    }

    if charset == Charset::Swiss {
        return match ch {
            '#' => 'ù',
            '@' => 'à',
            '[' => 'é',
            '\\' => 'ç',
            ']' => 'ê',
            '^' => 'î',
            '_' => 'è',
            '`' => 'ô',
            '{' => 'ä',
            '|' => 'ö',
            '}' => 'ü',
            '~' => 'û',
            _ => ch,
        };
    }

    if charset == Charset::French {
        return match ch {
            '#' => '£',
            '@' => 'à',
            '[' => '°',
            '\\' => 'ç',
            ']' => '§',
            '{' => 'é',
            '|' => 'ù',
            '}' => 'è',
            '~' => '¨',
            _ => ch,
        };
    }

    if charset == Charset::FrenchCanadian {
        return match ch {
            '@' => 'à',
            '[' => 'â',
            '\\' => 'ç',
            ']' => 'ê',
            '^' => 'î',
            '`' => 'ô',
            '{' => 'é',
            '|' => 'ù',
            '}' => 'è',
            '~' => 'û',
            _ => ch,
        };
    }

    if charset == Charset::Italian {
        return match ch {
            '#' => '£',
            '@' => '§',
            '[' => '°',
            '\\' => 'ç',
            ']' => 'é',
            '`' => 'ù',
            '{' => 'à',
            '|' => 'ò',
            '}' => 'è',
            '~' => 'ì',
            _ => ch,
        };
    }

    if charset == Charset::JisRoman {
        return match ch {
            '\\' => '¥',
            '~' => '‾',
            _ => ch,
        };
    }

    if charset == Charset::JisKatakana {
        return match ch {
            '!'..='_' => char::from_u32(0xff61 + (ch as u32 - '!' as u32)).unwrap_or(ch),
            '`'..='}' => '␦',
            _ => ch,
        };
    }

    if charset == Charset::Greek {
        return match ch {
            'a' => 'Α',
            'b' => 'Β',
            'c' => 'Γ',
            'd' => 'Δ',
            'e' => 'Ε',
            'f' => 'Ζ',
            'g' => 'Η',
            'h' => 'Θ',
            'i' => 'Ι',
            'j' => 'Κ',
            'k' => 'Λ',
            'l' => 'Μ',
            'm' => 'Ν',
            'n' => 'Χ',
            'o' => 'Ο',
            'p' => 'Π',
            'q' => 'Ρ',
            'r' => 'Σ',
            's' => 'Τ',
            't' => 'Υ',
            'u' => 'Φ',
            'v' => 'Ξ',
            'w' => 'Ψ',
            'x' => 'Ω',
            'y' | 'z' => '␦',
            _ => ch,
        };
    }

    if charset == Charset::Hebrew {
        return match ch {
            '`' => 'א',
            'a' => 'ב',
            'b' => 'ג',
            'c' => 'ד',
            'd' => 'ה',
            'e' => 'ו',
            'f' => 'ז',
            'g' => 'ח',
            'h' => 'ט',
            'i' => 'י',
            'j' => 'ך',
            'k' => 'כ',
            'l' => 'ל',
            'm' => 'ם',
            'n' => 'מ',
            'o' => 'ן',
            'p' => 'נ',
            'q' => 'ס',
            'r' => 'ע',
            's' => 'ף',
            't' => 'פ',
            'u' => 'ץ',
            'v' => 'צ',
            'w' => 'ק',
            'x' => 'ר',
            'y' => 'ש',
            'z' => 'ת',
            _ => ch,
        };
    }

    if charset == Charset::Turkish {
        return match ch {
            '&' => 'ğ',
            '@' => 'İ',
            '[' => 'Ş',
            '\\' => 'Ö',
            ']' => 'Ç',
            '^' => 'Ü',
            '`' => 'Ğ',
            '{' => 'ş',
            '|' => 'ö',
            '}' => 'ç',
            '~' => 'ü',
            _ => ch,
        };
    }

    if charset == Charset::Russian {
        return match ch {
            '`' => 'Ю',
            'a' => 'А',
            'b' => 'Б',
            'c' => 'Ц',
            'd' => 'Д',
            'e' => 'Е',
            'f' => 'Ф',
            'g' => 'Г',
            'h' => 'Х',
            'i' => 'И',
            'j' => 'Й',
            'k' => 'К',
            'l' => 'Л',
            'm' => 'М',
            'n' => 'Н',
            'o' => 'О',
            'p' => 'П',
            'q' => 'Я',
            'r' => 'Р',
            's' => 'С',
            't' => 'Т',
            'u' => 'У',
            'v' => 'Ж',
            'w' => 'В',
            'x' => 'Ь',
            'y' => 'Ы',
            'z' => 'З',
            '{' => 'Ш',
            '|' => 'Э',
            '}' => 'Щ',
            '~' => 'Ч',
            _ => ch,
        };
    }

    if charset == Charset::SerboCroatian {
        return match ch {
            '@' => 'Ž',
            '[' => 'Š',
            '\\' => 'Đ',
            ']' => 'Ć',
            '^' => 'Č',
            '`' => 'ž',
            '{' => 'š',
            '|' => 'đ',
            '}' => 'ć',
            '~' => 'č',
            _ => ch,
        };
    }

    if charset == Charset::NorwegianDanish {
        return match ch {
            '@' => 'Ä',
            '[' => 'Æ',
            '\\' => 'Ø',
            ']' => 'Å',
            '^' => 'Ü',
            '`' => 'ä',
            '{' => 'æ',
            '|' => 'ø',
            '}' => 'å',
            '~' => 'ü',
            _ => ch,
        };
    }

    if charset == Charset::Portuguese {
        return match ch {
            '[' => 'Ã',
            '\\' => 'Ç',
            ']' => 'Õ',
            '{' => 'ã',
            '|' => 'ç',
            '}' => 'õ',
            _ => ch,
        };
    }

    if charset == Charset::Spanish {
        return match ch {
            '#' => '£',
            '@' => '§',
            '[' => '¡',
            '\\' => 'Ñ',
            ']' => '¿',
            '{' => '°',
            '|' => 'ñ',
            '}' => 'ç',
            _ => ch,
        };
    }

    if charset == Charset::DecSpecialGraphics {
        return match ch {
            '`' => '◆',
            'a' => '▒',
            'f' => '°',
            'g' => '±',
            'h' => '␤',
            'i' => '␋',
            'j' => '┘',
            'k' => '┐',
            'l' => '┌',
            'm' => '└',
            'n' => '┼',
            'o' => '⎺',
            'p' => '⎻',
            'q' => '─',
            'r' => '⎼',
            's' => '⎽',
            't' => '├',
            'u' => '┤',
            'v' => '┴',
            'w' => '┬',
            'x' => '│',
            'y' => '≤',
            'z' => '≥',
            '{' => 'π',
            '|' => '≠',
            '}' => '£',
            '~' => '·',
            _ => ch,
        };
    }

    ch
}

fn is_gr_printable(ch: char) -> bool {
    matches!(ch as u32, 0xa0..=0xff)
}

fn gr_to_gl_printable(ch: char) -> char {
    char::from_u32((ch as u32) - 0x80).unwrap_or(ch)
}

fn parse_osc(params: &[&[u8]]) -> Action {
    if params.len() >= 2 && matches!(params[0], b"0" | b"2") {
        return parse_osc_title(params[1]);
    }

    if params.len() < 3 || params[0] != b"52" {
        return Action::Ignore;
    }

    let payload = params[2];
    if payload == b"?" {
        return parse_osc52_query(params[1]);
    }

    if payload.len() > MAX_OSC52_DECODED_BYTES.saturating_mul(2) {
        return Action::Ignore;
    }

    let Ok(decoded) = STANDARD.decode(payload) else {
        return Action::Ignore;
    };
    if decoded.len() > MAX_OSC52_DECODED_BYTES {
        return Action::Ignore;
    }

    match String::from_utf8(decoded) {
        Ok(text) => Action::SetClipboard(text),
        Err(_) => Action::Ignore,
    }
}

fn parse_osc52_query(selector: &[u8]) -> Action {
    if selector.is_empty() || selector.len() > MAX_OSC52_SELECTOR_BYTES {
        return Action::Ignore;
    }

    match std::str::from_utf8(selector) {
        Ok(selector) => Action::ClipboardQueryDenied(selector.to_string()),
        Err(_) => Action::Ignore,
    }
}

fn parse_osc_title(title: &[u8]) -> Action {
    if title.is_empty() || title.len() > MAX_OSC_TITLE_BYTES {
        return Action::Ignore;
    }

    match std::str::from_utf8(title) {
        Ok(title) => Action::SetWindowTitle(title.to_string()),
        Err(_) => Action::Ignore,
    }
}

fn parse_csi(numbers: &[usize], intermediates: &[u8], final_byte: char) -> Action {
    if intermediates == b"?" {
        return parse_private_csi(numbers, final_byte);
    }

    if intermediates == b">" && final_byte == 'c' {
        return Action::SecondaryDeviceAttributes;
    }

    if intermediates == b" " && final_byte == 'q' {
        return parse_cursor_style(numbers);
    }

    match final_byte {
        '@' => Action::InsertBlankChars(first_or_default(&numbers, 1)),
        'A' => Action::CursorUp(first_or_default(&numbers, 1)),
        'B' => Action::CursorDown(first_or_default(&numbers, 1)),
        'C' => Action::CursorRight(first_or_default(&numbers, 1)),
        'D' => Action::CursorLeft(first_or_default(&numbers, 1)),
        'c' => Action::PrimaryDeviceAttributes,
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
        'L' => Action::InsertLines(first_or_default(&numbers, 1)),
        'M' => Action::DeleteLines(first_or_default(&numbers, 1)),
        'n' => match first_or_default(&numbers, 0) {
            5 => Action::DeviceStatusReport,
            6 => Action::CursorPositionReport,
            _ => Action::Ignore,
        },
        'P' => Action::DeleteChars(first_or_default(&numbers, 1)),
        'X' => Action::EraseChars(first_or_default(&numbers, 1)),
        'r' => parse_scroll_region(&numbers),
        's' => Action::SaveCursor,
        'u' => Action::RestoreCursor,
        'm' => Action::SetGraphicRendition(numbers.to_vec()),
        _ => Action::Ignore,
    }
}

fn parse_cursor_style(numbers: &[usize]) -> Action {
    match first_or_default(numbers, 0) {
        0..=2 => Action::SetCursorStyle(CursorStyle::Block),
        3 | 4 => Action::SetCursorStyle(CursorStyle::Underline),
        5 | 6 => Action::SetCursorStyle(CursorStyle::Bar),
        _ => Action::Ignore,
    }
}

fn parse_scroll_region(numbers: &[usize]) -> Action {
    if numbers.is_empty() {
        return Action::SetScrollRegion(None);
    }

    let top = first_or_default(numbers, 1).saturating_sub(1);
    let bottom = numbers.get(1).copied().unwrap_or(0).saturating_sub(1);
    if bottom <= top {
        Action::Ignore
    } else {
        Action::SetScrollRegion(Some((top, bottom)))
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
        'h' if contains_any(&numbers, &[1000, 1002, 1003]) => Action::SetMouseReporting(true),
        'l' if contains_any(&numbers, &[1000, 1002, 1003]) => Action::SetMouseReporting(false),
        'h' if contains_any(&numbers, &[1006]) => Action::SetSgrMouse(true),
        'l' if contains_any(&numbers, &[1006]) => Action::SetSgrMouse(false),
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
    use crate::cursor::CursorStyle;

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
        assert_eq!(
            parser.advance('\u{07}'),
            Some(Action::SetWindowTitle("title".to_string()))
        );
        assert_eq!(parser.advance('x'), Some(Action::Print('x')));
    }

    #[test]
    fn parses_osc_title_update() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b]2;minimal terminal\x07"),
            vec![Action::SetWindowTitle("minimal terminal".to_string())]
        );
        assert_eq!(parser.advance_bytes(b"\x1b]2;\x07"), vec![Action::Ignore]);
    }

    #[test]
    fn parses_osc52_clipboard_write() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b]52;c;aGVsbG8=\x07"),
            vec![Action::SetClipboard("hello".to_string())]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b]52;c;?\x07"),
            vec![Action::ClipboardQueryDenied("c".to_string())]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b]52;c;not-base64\x07"),
            vec![Action::Ignore]
        );
    }

    #[test]
    fn parses_ascii_charset_designation() {
        let mut parser = Parser::default();
        assert_eq!(parser.advance('\u{1b}'), None);
        assert_eq!(parser.advance('('), None);
        assert_eq!(parser.advance('B'), None);
        assert_eq!(parser.advance('x'), Some(Action::Print('x')));
    }

    #[test]
    fn maps_dec_special_graphics_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(0lqk"),
            vec![Action::Print('┌'), Action::Print('─'), Action::Print('┐')]
        );
        assert_eq!(parser.advance_bytes(b"x"), vec![Action::Print('│')]);
        assert_eq!(parser.advance_bytes(b"\x1b(Bx"), vec![Action::Print('x')]);
    }

    #[test]
    fn maps_dec_supplemental_graphics_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%5!\"#(W]_$w}~\x1b(B!"),
            vec![
                Action::Print('¡'),
                Action::Print('¢'),
                Action::Print('£'),
                Action::Print('¤'),
                Action::Print('Œ'),
                Action::Print('Ÿ'),
                Action::Print('_'),
                Action::Print('␦'),
                Action::Print('œ'),
                Action::Print('ÿ'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%5\x1bNWx"),
            vec![Action::Print('Œ'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%5\x1b~\xc2\xa1"),
            vec![Action::Print('¡')]
        );
    }

    #[test]
    fn maps_dec_technical_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(>!\"#$%&'()*+,-./0<=>?@ABCDV{}~8\x1b(B!"),
            vec![
                Action::Print('⎷'),
                Action::Print('┌'),
                Action::Print('─'),
                Action::Print('⌠'),
                Action::Print('⌡'),
                Action::Print('│'),
                Action::Print('⎡'),
                Action::Print('⎣'),
                Action::Print('⎤'),
                Action::Print('⎦'),
                Action::Print('⎧'),
                Action::Print('⎩'),
                Action::Print('⎫'),
                Action::Print('⎭'),
                Action::Print('⎨'),
                Action::Print('⎬'),
                Action::Print('≤'),
                Action::Print('≠'),
                Action::Print('≥'),
                Action::Print('∫'),
                Action::Print('∴'),
                Action::Print('∝'),
                Action::Print('∞'),
                Action::Print('÷'),
                Action::Print('Δ'),
                Action::Print('√'),
                Action::Print('←'),
                Action::Print('→'),
                Action::Print('↓'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*>\x1bN~x"),
            vec![Action::Print('↓'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)>\x1b~\xc2\xa1"),
            vec![Action::Print('⎷')]
        );
    }

    #[test]
    fn maps_dec_cyrillic_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(&4@ABCXYZ[\\]^_`abcxyz{|}~!\x1b(B!"),
            vec![
                Action::Print('ю'),
                Action::Print('а'),
                Action::Print('б'),
                Action::Print('ц'),
                Action::Print('ь'),
                Action::Print('ы'),
                Action::Print('з'),
                Action::Print('ш'),
                Action::Print('э'),
                Action::Print('щ'),
                Action::Print('ч'),
                Action::Print('ъ'),
                Action::Print('Ю'),
                Action::Print('А'),
                Action::Print('Б'),
                Action::Print('Ц'),
                Action::Print('Ь'),
                Action::Print('Ы'),
                Action::Print('З'),
                Action::Print('Ш'),
                Action::Print('Э'),
                Action::Print('Щ'),
                Action::Print('Ч'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*&4\x1bN@x"),
            vec![Action::Print('ю'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)&4\x1b~\xc3\x80"),
            vec![Action::Print('ю')]
        );
    }

    #[test]
    fn maps_dec_greek_supplemental_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(\"?!\"#%()*+01235679:;<=?@ABCXYZ[\\]^_`abcxyz{|}~\x1b(B!"),
            vec![
                Action::Print('¡'),
                Action::Print('¢'),
                Action::Print('£'),
                Action::Print('¥'),
                Action::Print('¤'),
                Action::Print('©'),
                Action::Print('ª'),
                Action::Print('«'),
                Action::Print('°'),
                Action::Print('±'),
                Action::Print('²'),
                Action::Print('³'),
                Action::Print('µ'),
                Action::Print('¶'),
                Action::Print('·'),
                Action::Print('¹'),
                Action::Print('º'),
                Action::Print('»'),
                Action::Print('¼'),
                Action::Print('½'),
                Action::Print('¿'),
                Action::Print('ϊ'),
                Action::Print('Α'),
                Action::Print('Β'),
                Action::Print('Γ'),
                Action::Print('Ψ'),
                Action::Print('Ω'),
                Action::Print('ά'),
                Action::Print('έ'),
                Action::Print('ή'),
                Action::Print('ί'),
                Action::Print('␦'),
                Action::Print('ό'),
                Action::Print('ϋ'),
                Action::Print('α'),
                Action::Print('β'),
                Action::Print('γ'),
                Action::Print('ψ'),
                Action::Print('ω'),
                Action::Print('ς'),
                Action::Print('ύ'),
                Action::Print('ώ'),
                Action::Print('΄'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*\"?\x1bN@x"),
            vec![Action::Print('ϊ'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)\"?\x1b~\xc3\x80"),
            vec![Action::Print('ϊ')]
        );
    }

    #[test]
    fn maps_dec_hebrew_supplemental_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(\"4!\"#%()*+01235679:;<=?@_`abcxyz{|}~\x1b(B!"),
            vec![
                Action::Print('¡'),
                Action::Print('¢'),
                Action::Print('£'),
                Action::Print('¥'),
                Action::Print('¨'),
                Action::Print('©'),
                Action::Print('×'),
                Action::Print('«'),
                Action::Print('°'),
                Action::Print('±'),
                Action::Print('²'),
                Action::Print('³'),
                Action::Print('µ'),
                Action::Print('¶'),
                Action::Print('·'),
                Action::Print('¹'),
                Action::Print('÷'),
                Action::Print('»'),
                Action::Print('¼'),
                Action::Print('½'),
                Action::Print('¿'),
                Action::Print('␦'),
                Action::Print('␦'),
                Action::Print('א'),
                Action::Print('ב'),
                Action::Print('ג'),
                Action::Print('ד'),
                Action::Print('ר'),
                Action::Print('ש'),
                Action::Print('ת'),
                Action::Print('␦'),
                Action::Print('␦'),
                Action::Print('␦'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*\"4\x1bN`x"),
            vec![Action::Print('א'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)\"4\x1b~\xc3\xa0"),
            vec![Action::Print('א')]
        );
    }

    #[test]
    fn maps_dec_turkish_supplemental_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%0!\"#%.>PW]^_`pw}~$\x1b(B!"),
            vec![
                Action::Print('¡'),
                Action::Print('¢'),
                Action::Print('£'),
                Action::Print('¥'),
                Action::Print('İ'),
                Action::Print('ı'),
                Action::Print('Ğ'),
                Action::Print('Œ'),
                Action::Print('Ÿ'),
                Action::Print('Ş'),
                Action::Print('ß'),
                Action::Print('à'),
                Action::Print('ğ'),
                Action::Print('œ'),
                Action::Print('ÿ'),
                Action::Print('ş'),
                Action::Print('␦'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%0\x1bN.x"),
            vec![Action::Print('İ'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%0\x1b~\xc2\xa1"),
            vec![Action::Print('¡')]
        );
    }

    #[test]
    fn maps_g1_dec_special_graphics_with_locking_shift() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b)0\x0elqk"),
            vec![Action::Print('┌'), Action::Print('─'), Action::Print('┐')]
        );
        assert_eq!(parser.advance_bytes(b"x"), vec![Action::Print('│')]);
        assert_eq!(parser.advance_bytes(b"\x0fx"), vec![Action::Print('x')]);
    }

    #[test]
    fn maps_g2_and_g3_dec_special_graphics_with_locking_shift() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b*0\x1bnlqk"),
            vec![Action::Print('┌'), Action::Print('─'), Action::Print('┐')]
        );
        assert_eq!(parser.advance_bytes(b"\x0fx"), vec![Action::Print('x')]);
        assert_eq!(
            parser.advance_bytes(b"\x1b+0\x1bomqj"),
            vec![Action::Print('└'), Action::Print('─'), Action::Print('┘')]
        );
        assert_eq!(parser.advance_bytes(b"\x0fx"), vec![Action::Print('x')]);
    }

    #[test]
    fn maps_g2_and_g3_dec_special_graphics_with_single_shift() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b*0\x1b+0\x1bNlx"),
            vec![Action::Print('┌'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1bOmx"),
            vec![Action::Print('└'), Action::Print('x')]
        );
    }

    #[test]
    fn maps_right_side_g_sets_with_locking_shift() {
        let mut parser = Parser::default();

        assert_eq!(parser.advance_bytes(b"\xc3\xb1"), vec![Action::Print('ñ')]);
        assert_eq!(
            parser.advance_bytes(b"\x1b)0\x1b~\xc3\xb1q"),
            vec![Action::Print('─'), Action::Print('q')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*0\x1b}\xc3\xac"),
            vec![Action::Print('┌')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b+A\x1b|\xc2\xa3"),
            vec![Action::Print('£')]
        );
    }

    #[test]
    fn maps_british_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(A#x\x1b(B#"),
            vec![Action::Print('£'), Action::Print('x'), Action::Print('#')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*A\x1bN#x"),
            vec![Action::Print('£'), Action::Print('x')]
        );
    }

    #[test]
    fn maps_german_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(K@[\\] {|}~\x1b(B@"),
            vec![
                Action::Print('§'),
                Action::Print('Ä'),
                Action::Print('Ö'),
                Action::Print('Ü'),
                Action::Print(' '),
                Action::Print('ä'),
                Action::Print('ö'),
                Action::Print('ü'),
                Action::Print('ß'),
                Action::Print('@'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*K\x1bN~x"),
            vec![Action::Print('ß'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)K\x1b~\xc3\xbb"),
            vec![Action::Print('ä')]
        );
    }

    #[test]
    fn maps_finnish_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(C[\\]^`{|}~\x1b(B["),
            vec![
                Action::Print('Ä'),
                Action::Print('Ö'),
                Action::Print('Å'),
                Action::Print('Ü'),
                Action::Print('é'),
                Action::Print('ä'),
                Action::Print('ö'),
                Action::Print('å'),
                Action::Print('ü'),
                Action::Print('['),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*C\x1bN~x"),
            vec![Action::Print('ü'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)C\x1b~\xc3\xbb"),
            vec![Action::Print('ä')]
        );
    }

    #[test]
    fn maps_french_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(R#@[\\] {|}~\x1b(B#"),
            vec![
                Action::Print('£'),
                Action::Print('à'),
                Action::Print('°'),
                Action::Print('ç'),
                Action::Print('§'),
                Action::Print(' '),
                Action::Print('é'),
                Action::Print('ù'),
                Action::Print('è'),
                Action::Print('¨'),
                Action::Print('#'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*f\x1bN~x"),
            vec![Action::Print('¨'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)R\x1b~\xc3\xbb"),
            vec![Action::Print('é')]
        );
    }

    #[test]
    fn maps_italian_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(Y#@[\\]`{|}~\x1b(B#"),
            vec![
                Action::Print('£'),
                Action::Print('§'),
                Action::Print('°'),
                Action::Print('ç'),
                Action::Print('é'),
                Action::Print('ù'),
                Action::Print('à'),
                Action::Print('ò'),
                Action::Print('è'),
                Action::Print('ì'),
                Action::Print('#'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*Y\x1bN~x"),
            vec![Action::Print('ì'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)Y\x1b~\xc3\xbb"),
            vec![Action::Print('à')]
        );
    }

    #[test]
    fn maps_jis_roman_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(J\\~\x1b(B\\"),
            vec![Action::Print('¥'), Action::Print('‾'), Action::Print('\\'),]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*J\x1bN~x"),
            vec![Action::Print('‾'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)J\x1b~\xc3\x9c"),
            vec![Action::Print('¥')]
        );
    }

    #[test]
    fn maps_jis_katakana_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(I!&1@Z[]^_\x1b(B!"),
            vec![
                Action::Print('｡'),
                Action::Print('ｦ'),
                Action::Print('ｱ'),
                Action::Print('ﾀ'),
                Action::Print('ﾚ'),
                Action::Print('ﾛ'),
                Action::Print('ﾝ'),
                Action::Print('ﾞ'),
                Action::Print('ﾟ'),
                Action::Print('!'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*I\x1bN_x"),
            vec![Action::Print('ﾟ'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)I\x1b~\xc2\xa1"),
            vec![Action::Print('｡')]
        );
    }

    #[test]
    fn maps_spanish_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(Z#@[\\] {|}~\x1b(B#"),
            vec![
                Action::Print('£'),
                Action::Print('§'),
                Action::Print('¡'),
                Action::Print('Ñ'),
                Action::Print('¿'),
                Action::Print(' '),
                Action::Print('°'),
                Action::Print('ñ'),
                Action::Print('ç'),
                Action::Print('~'),
                Action::Print('#'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*Z\x1bN}x"),
            vec![Action::Print('ç'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)Z\x1b~\xc3\xbb"),
            vec![Action::Print('°')]
        );
    }

    #[test]
    fn maps_dutch_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(4#@[\\] {|}~\x1b(B#"),
            vec![
                Action::Print('£'),
                Action::Print('¾'),
                Action::Print('ĳ'),
                Action::Print('½'),
                Action::Print('|'),
                Action::Print(' '),
                Action::Print('¨'),
                Action::Print('ƒ'),
                Action::Print('¼'),
                Action::Print('´'),
                Action::Print('#'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*4\x1bN~x"),
            vec![Action::Print('´'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)4\x1b~\xc3\xbb"),
            vec![Action::Print('¨')]
        );
    }

    #[test]
    fn maps_swedish_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(H@[\\]^`{|}~\x1b(B@"),
            vec![
                Action::Print('É'),
                Action::Print('Ä'),
                Action::Print('Ö'),
                Action::Print('Å'),
                Action::Print('Ü'),
                Action::Print('é'),
                Action::Print('ä'),
                Action::Print('ö'),
                Action::Print('å'),
                Action::Print('ü'),
                Action::Print('@'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*7\x1bN~x"),
            vec![Action::Print('ü'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)H\x1b~\xc3\xbb"),
            vec![Action::Print('ä')]
        );
    }

    #[test]
    fn maps_norwegian_danish_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(E@[\\]^`{|}~\x1b(B@"),
            vec![
                Action::Print('Ä'),
                Action::Print('Æ'),
                Action::Print('Ø'),
                Action::Print('Å'),
                Action::Print('Ü'),
                Action::Print('ä'),
                Action::Print('æ'),
                Action::Print('ø'),
                Action::Print('å'),
                Action::Print('ü'),
                Action::Print('@'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*6\x1bN~x"),
            vec![Action::Print('ü'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)`\x1b~\xc3\xbb"),
            vec![Action::Print('æ')]
        );
    }

    #[test]
    fn maps_french_canadian_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(Q#@[\\]^`{|}~\x1b(B@"),
            vec![
                Action::Print('#'),
                Action::Print('à'),
                Action::Print('â'),
                Action::Print('ç'),
                Action::Print('ê'),
                Action::Print('î'),
                Action::Print('ô'),
                Action::Print('é'),
                Action::Print('ù'),
                Action::Print('è'),
                Action::Print('û'),
                Action::Print('@'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*9\x1bN~x"),
            vec![Action::Print('û'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)Q\x1b~\xc3\xbb"),
            vec![Action::Print('é')]
        );
    }

    #[test]
    fn maps_swiss_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(=#@[\\]^_`{|}~\x1b(B#"),
            vec![
                Action::Print('ù'),
                Action::Print('à'),
                Action::Print('é'),
                Action::Print('ç'),
                Action::Print('ê'),
                Action::Print('î'),
                Action::Print('è'),
                Action::Print('ô'),
                Action::Print('ä'),
                Action::Print('ö'),
                Action::Print('ü'),
                Action::Print('û'),
                Action::Print('#'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*=\x1bN~x"),
            vec![Action::Print('û'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)=\x1b~\xc3\xbb"),
            vec![Action::Print('ä')]
        );
    }

    #[test]
    fn maps_portuguese_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%6#[\\] {|}~\x1b(B["),
            vec![
                Action::Print('#'),
                Action::Print('Ã'),
                Action::Print('Ç'),
                Action::Print('Õ'),
                Action::Print(' '),
                Action::Print('ã'),
                Action::Print('ç'),
                Action::Print('õ'),
                Action::Print('~'),
                Action::Print('['),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%6\x1bN}x"),
            vec![Action::Print('õ'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%6\x1b~\xc3\xbb"),
            vec![Action::Print('ã')]
        );
    }

    #[test]
    fn maps_greek_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(parser.advance_bytes(b"\x1b(\">"), Vec::<Action>::new());
        assert_eq!(
            parser.advance_bytes(b"abcdefghijklmnopqrstuvwx\x1b(Ba"),
            vec![
                Action::Print('Α'),
                Action::Print('Β'),
                Action::Print('Γ'),
                Action::Print('Δ'),
                Action::Print('Ε'),
                Action::Print('Ζ'),
                Action::Print('Η'),
                Action::Print('Θ'),
                Action::Print('Ι'),
                Action::Print('Κ'),
                Action::Print('Λ'),
                Action::Print('Μ'),
                Action::Print('Ν'),
                Action::Print('Χ'),
                Action::Print('Ο'),
                Action::Print('Π'),
                Action::Print('Ρ'),
                Action::Print('Σ'),
                Action::Print('Τ'),
                Action::Print('Υ'),
                Action::Print('Φ'),
                Action::Print('Ξ'),
                Action::Print('Ψ'),
                Action::Print('Ω'),
                Action::Print('a'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*\">\x1bNxx"),
            vec![Action::Print('Ω'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)\">\x1b~\xc3\xa1"),
            vec![Action::Print('Α')]
        );
    }

    #[test]
    fn maps_hebrew_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%=`abcdefghijklmnopqrstuvwxyz\x1b(B`"),
            vec![
                Action::Print('א'),
                Action::Print('ב'),
                Action::Print('ג'),
                Action::Print('ד'),
                Action::Print('ה'),
                Action::Print('ו'),
                Action::Print('ז'),
                Action::Print('ח'),
                Action::Print('ט'),
                Action::Print('י'),
                Action::Print('ך'),
                Action::Print('כ'),
                Action::Print('ל'),
                Action::Print('ם'),
                Action::Print('מ'),
                Action::Print('ן'),
                Action::Print('נ'),
                Action::Print('ס'),
                Action::Print('ע'),
                Action::Print('ף'),
                Action::Print('פ'),
                Action::Print('ץ'),
                Action::Print('צ'),
                Action::Print('ק'),
                Action::Print('ר'),
                Action::Print('ש'),
                Action::Print('ת'),
                Action::Print('`'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%=\x1bNzx"),
            vec![Action::Print('ת'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%=\x1b~\xc3\xa0"),
            vec![Action::Print('א')]
        );
    }

    #[test]
    fn maps_turkish_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%2&@[\\]^`{|}~\x1b(B&"),
            vec![
                Action::Print('ğ'),
                Action::Print('İ'),
                Action::Print('Ş'),
                Action::Print('Ö'),
                Action::Print('Ç'),
                Action::Print('Ü'),
                Action::Print('Ğ'),
                Action::Print('ş'),
                Action::Print('ö'),
                Action::Print('ç'),
                Action::Print('ü'),
                Action::Print('&'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%2\x1bN~x"),
            vec![Action::Print('ü'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%2\x1b~\xc2\xa6"),
            vec![Action::Print('ğ')]
        );
    }

    #[test]
    fn maps_russian_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(&5`abcdefghijklmnopqrstuvwxyz{|}~\x1b(B`"),
            vec![
                Action::Print('Ю'),
                Action::Print('А'),
                Action::Print('Б'),
                Action::Print('Ц'),
                Action::Print('Д'),
                Action::Print('Е'),
                Action::Print('Ф'),
                Action::Print('Г'),
                Action::Print('Х'),
                Action::Print('И'),
                Action::Print('Й'),
                Action::Print('К'),
                Action::Print('Л'),
                Action::Print('М'),
                Action::Print('Н'),
                Action::Print('О'),
                Action::Print('П'),
                Action::Print('Я'),
                Action::Print('Р'),
                Action::Print('С'),
                Action::Print('Т'),
                Action::Print('У'),
                Action::Print('Ж'),
                Action::Print('В'),
                Action::Print('Ь'),
                Action::Print('Ы'),
                Action::Print('З'),
                Action::Print('Ш'),
                Action::Print('Э'),
                Action::Print('Щ'),
                Action::Print('Ч'),
                Action::Print('`'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*&5\x1bN~x"),
            vec![Action::Print('Ч'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)&5\x1b~\xc3\xa0"),
            vec![Action::Print('Ю')]
        );
    }

    #[test]
    fn maps_serbo_croatian_nrcs_charset() {
        let mut parser = Parser::default();

        assert_eq!(
            parser.advance_bytes(b"\x1b(%3@[\\]^`{|}~\x1b(B@"),
            vec![
                Action::Print('Ž'),
                Action::Print('Š'),
                Action::Print('Đ'),
                Action::Print('Ć'),
                Action::Print('Č'),
                Action::Print('ž'),
                Action::Print('š'),
                Action::Print('đ'),
                Action::Print('ć'),
                Action::Print('č'),
                Action::Print('@'),
            ]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b*%3\x1bN~x"),
            vec![Action::Print('č'), Action::Print('x')]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b)%3\x1b~\xc3\xa0"),
            vec![Action::Print('ž')]
        );
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
    fn parses_application_keypad_modes() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b="),
            vec![Action::SetApplicationKeypad(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b>"),
            vec![Action::SetApplicationKeypad(false)]
        );
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
        assert_eq!(parser.advance('5'), None);
        assert_eq!(parser.advance('h'), Some(Action::Ignore));
    }

    #[test]
    fn parses_tui_private_modes() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[?25l"),
            vec![Action::SetCursorVisible(false)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?25h"),
            vec![Action::SetCursorVisible(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?2004h"),
            vec![Action::SetBracketedPaste(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?1h"),
            vec![Action::SetApplicationCursorKeys(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?1000h"),
            vec![Action::SetMouseReporting(true)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[?1006h"),
            vec![Action::SetSgrMouse(true)]
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

    #[test]
    fn parses_tui_editing_sequences() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[2@"),
            vec![Action::InsertBlankChars(2)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[3P"),
            vec![Action::DeleteChars(3)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[4X"),
            vec![Action::EraseChars(4)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[2L"),
            vec![Action::InsertLines(2)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[2M"),
            vec![Action::DeleteLines(2)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[2;4r"),
            vec![Action::SetScrollRegion(Some((1, 3)))]
        );
    }

    #[test]
    fn parses_device_status_reports() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[5n"),
            vec![Action::DeviceStatusReport]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[6n"),
            vec![Action::CursorPositionReport]
        );
        assert_eq!(parser.advance_bytes(b"\x1b[9n"), vec![Action::Ignore]);
    }

    #[test]
    fn parses_primary_device_attributes() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[c"),
            vec![Action::PrimaryDeviceAttributes]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[0c"),
            vec![Action::PrimaryDeviceAttributes]
        );
    }

    #[test]
    fn parses_secondary_device_attributes() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[>c"),
            vec![Action::SecondaryDeviceAttributes]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[>0c"),
            vec![Action::SecondaryDeviceAttributes]
        );
    }

    #[test]
    fn parses_cursor_style_sequences() {
        let mut parser = Parser::default();
        assert_eq!(
            parser.advance_bytes(b"\x1b[2 q"),
            vec![Action::SetCursorStyle(CursorStyle::Block)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[4 q"),
            vec![Action::SetCursorStyle(CursorStyle::Underline)]
        );
        assert_eq!(
            parser.advance_bytes(b"\x1b[6 q"),
            vec![Action::SetCursorStyle(CursorStyle::Bar)]
        );
        assert_eq!(parser.advance_bytes(b"\x1b[9 q"), vec![Action::Ignore]);
    }
}
