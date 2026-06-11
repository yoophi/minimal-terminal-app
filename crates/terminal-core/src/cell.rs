use crate::style::Style;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Cell {
    ch: char,
    wide_continuation: bool,
    style: Style,
}

impl Cell {
    pub(crate) fn blank() -> Self {
        Self {
            ch: ' ',
            wide_continuation: false,
            style: Style::default(),
        }
    }

    pub(crate) fn blank_with_style(style: Style) -> Self {
        Self {
            ch: ' ',
            wide_continuation: false,
            style,
        }
    }

    pub(crate) fn ch(&self) -> char {
        self.ch
    }

    pub(crate) fn is_wide_continuation(&self) -> bool {
        self.wide_continuation
    }

    pub(crate) fn style(&self) -> Style {
        self.style
    }

    pub(crate) fn set_ch(&mut self, ch: char, style: Style) {
        self.ch = ch;
        self.wide_continuation = false;
        self.style = style;
    }

    pub(crate) fn set_wide_continuation(&mut self, style: Style) {
        self.ch = ' ';
        self.wide_continuation = true;
        self.style = style;
    }

    pub(crate) fn clear(&mut self) {
        self.ch = ' ';
        self.wide_continuation = false;
        self.style = Style::default();
    }

    pub(crate) fn clear_with_style(&mut self, style: Style) {
        self.ch = ' ';
        self.wide_continuation = false;
        self.style = style;
    }
}
