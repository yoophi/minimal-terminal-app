use crate::cell::Cell;
use crate::cursor::Cursor;

#[derive(Clone, Debug)]
pub(crate) struct Grid {
    rows: usize,
    cols: usize,
    lines: Vec<Vec<Cell>>,
}

impl Grid {
    pub(crate) fn new(rows: usize, cols: usize) -> Self {
        let rows = rows.max(1);
        let cols = cols.max(1);
        Self {
            rows,
            cols,
            lines: vec![blank_line(cols); rows],
        }
    }

    pub(crate) fn rows(&self) -> usize {
        self.rows
    }

    pub(crate) fn cols(&self) -> usize {
        self.cols
    }

    pub(crate) fn put_char(&mut self, cursor: &mut Cursor, ch: char) {
        if cursor.row >= self.rows {
            cursor.row = self.rows - 1;
        }
        if cursor.col >= self.cols {
            self.newline(cursor);
        }

        self.lines[cursor.row][cursor.col].set_ch(ch);
        cursor.col += 1;
        if cursor.col >= self.cols {
            self.newline(cursor);
        }
    }

    pub(crate) fn carriage_return(&mut self, cursor: &mut Cursor) {
        cursor.col = 0;
    }

    pub(crate) fn newline(&mut self, cursor: &mut Cursor) {
        cursor.col = 0;
        if cursor.row + 1 >= self.rows {
            self.scroll_up();
        } else {
            cursor.row += 1;
        }
    }

    pub(crate) fn backspace(&mut self, cursor: &mut Cursor) {
        if cursor.col > 0 {
            cursor.col -= 1;
            self.lines[cursor.row][cursor.col].set_ch(' ');
        }
    }

    pub(crate) fn move_cursor(&self, cursor: &mut Cursor, row: usize, col: usize) {
        cursor.row = row.min(self.rows - 1);
        cursor.col = col.min(self.cols - 1);
    }

    pub(crate) fn move_up(&self, cursor: &mut Cursor, count: usize) {
        cursor.row = cursor.row.saturating_sub(count.max(1));
    }

    pub(crate) fn move_down(&self, cursor: &mut Cursor, count: usize) {
        cursor.row = (cursor.row + count.max(1)).min(self.rows - 1);
    }

    pub(crate) fn move_right(&self, cursor: &mut Cursor, count: usize) {
        cursor.col = (cursor.col + count.max(1)).min(self.cols - 1);
    }

    pub(crate) fn move_left(&self, cursor: &mut Cursor, count: usize) {
        cursor.col = cursor.col.saturating_sub(count.max(1));
    }

    pub(crate) fn clear_line_from_cursor(&mut self, cursor: Cursor) {
        for col in cursor.col..self.cols {
            self.lines[cursor.row][col].set_ch(' ');
        }
    }

    pub(crate) fn clear_screen(&mut self, cursor: &mut Cursor) {
        for line in &mut self.lines {
            for cell in line {
                cell.set_ch(' ');
            }
        }
        *cursor = Cursor::default();
    }

    pub(crate) fn visible_lines_from(&self, start: usize, max_visible_lines: usize) -> Vec<String> {
        let max_visible_lines = max_visible_lines.max(1);
        let start = start.min(self.rows.saturating_sub(1));
        let end = (start + max_visible_lines).min(self.rows);
        self.lines[start..end]
            .iter()
            .map(|line| trim_trailing_blanks(line.iter().map(Cell::ch).collect()))
            .collect()
    }

    fn scroll_up(&mut self) {
        self.lines.remove(0);
        self.lines.push(blank_line(self.cols));
    }
}

fn blank_line(cols: usize) -> Vec<Cell> {
    vec![Cell::blank(); cols]
}

fn trim_trailing_blanks(mut text: String) -> String {
    while text.ends_with(' ') {
        text.pop();
    }
    text
}
