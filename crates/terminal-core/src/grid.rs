use crate::cell::Cell;
use crate::cursor::Cursor;

#[derive(Clone, Debug)]
pub(crate) struct Grid {
    rows: usize,
    cols: usize,
    lines: Vec<Vec<Cell>>,
    scrollback: Vec<Vec<Cell>>,
    max_scrollback_lines: usize,
}

impl Grid {
    pub(crate) fn new(rows: usize, cols: usize) -> Self {
        let rows = rows.max(1);
        let cols = cols.max(1);
        Self {
            rows,
            cols,
            lines: vec![blank_line(cols); rows],
            scrollback: Vec::new(),
            max_scrollback_lines: 2_000,
        }
    }

    pub(crate) fn rows(&self) -> usize {
        self.rows
    }

    pub(crate) fn cols(&self) -> usize {
        self.cols
    }

    pub(crate) fn scrollback_len(&self) -> usize {
        self.scrollback.len()
    }

    pub(crate) fn resize(&mut self, rows: usize, cols: usize, cursor: &mut Cursor) {
        let rows = rows.max(1);
        let cols = cols.max(1);

        for line in &mut self.lines {
            resize_line(line, cols);
        }

        if rows > self.rows {
            self.lines
                .extend((0..(rows - self.rows)).map(|_| blank_line(cols)));
        } else if rows < self.rows {
            self.lines.truncate(rows);
        }

        self.rows = rows;
        self.cols = cols;
        cursor.row = cursor.row.min(self.rows - 1);
        cursor.col = cursor.col.min(self.cols - 1);
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
        let line = self.lines.remove(0);
        self.push_scrollback(line);
        self.lines.push(blank_line(self.cols));
    }

    fn push_scrollback(&mut self, line: Vec<Cell>) {
        self.scrollback.push(line);
        if self.scrollback.len() > self.max_scrollback_lines {
            let overflow = self.scrollback.len() - self.max_scrollback_lines;
            self.scrollback.drain(0..overflow);
        }
    }
}

fn blank_line(cols: usize) -> Vec<Cell> {
    vec![Cell::blank(); cols]
}

fn resize_line(line: &mut Vec<Cell>, cols: usize) {
    if line.len() > cols {
        line.truncate(cols);
    } else {
        line.extend((0..(cols - line.len())).map(|_| Cell::blank()));
    }
}

fn trim_trailing_blanks(mut text: String) -> String {
    while text.ends_with(' ') {
        text.pop();
    }
    text
}
