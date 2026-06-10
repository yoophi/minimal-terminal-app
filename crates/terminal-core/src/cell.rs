#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Cell {
    ch: char,
}

impl Cell {
    pub(crate) fn blank() -> Self {
        Self { ch: ' ' }
    }

    pub(crate) fn ch(&self) -> char {
        self.ch
    }

    pub(crate) fn set_ch(&mut self, ch: char) {
        self.ch = ch;
    }
}
