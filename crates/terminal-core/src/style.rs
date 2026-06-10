#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
}

impl Style {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn set_foreground(&mut self, color: Option<Color>) {
        self.foreground = color;
    }

    pub(crate) fn set_background(&mut self, color: Option<Color>) {
        self.background = color;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyledSpan {
    pub text: String,
    pub style: Style,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyledLine {
    pub spans: Vec<StyledSpan>,
}
