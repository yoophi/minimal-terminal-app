#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Cell {
    ch: char,
    wide_continuation: bool,
}

impl Cell {
    pub(crate) fn blank() -> Self {
        Self {
            ch: ' ',
            wide_continuation: false,
        }
    }

    pub(crate) fn ch(&self) -> char {
        self.ch
    }

    pub(crate) fn is_wide_continuation(&self) -> bool {
        self.wide_continuation
    }

    pub(crate) fn set_ch(&mut self, ch: char) {
        self.ch = ch;
        self.wide_continuation = false;
    }

    pub(crate) fn set_wide_continuation(&mut self) {
        self.ch = ' ';
        self.wide_continuation = true;
    }

    pub(crate) fn clear(&mut self) {
        self.ch = ' ';
        self.wide_continuation = false;
    }
}
