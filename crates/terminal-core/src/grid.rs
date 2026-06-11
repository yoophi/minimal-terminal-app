use crate::cell::Cell;
use crate::cursor::Cursor;
use crate::style::{Style, StyledLine, StyledSpan};

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

    pub(crate) fn put_char(&mut self, cursor: &mut Cursor, ch: char, style: Style) -> bool {
        let width = char_width(ch);
        if width == 0 {
            return false;
        }

        if cursor.row >= self.rows {
            cursor.row = self.rows - 1;
        }
        if cursor.col + width > self.cols {
            self.newline(cursor);
        }

        self.lines[cursor.row][cursor.col].set_ch(ch, style);
        if width == 2 && cursor.col + 1 < self.cols {
            self.lines[cursor.row][cursor.col + 1].set_wide_continuation(style);
        }

        if cursor.col + width >= self.cols {
            cursor.col = self.cols - 1;
            true
        } else {
            cursor.col += width;
            false
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

    pub(crate) fn newline_in_region(
        &mut self,
        cursor: &mut Cursor,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some(region) = region else {
            self.newline(cursor);
            return;
        };
        let Some((top, bottom)) = normalized_region(Some(region), self.rows) else {
            self.newline(cursor);
            return;
        };

        cursor.col = 0;
        if cursor.row == bottom {
            self.scroll_up_region(top, bottom, style);
        } else if cursor.row >= top && cursor.row < bottom {
            cursor.row += 1;
        } else {
            self.newline(cursor);
        }
    }

    pub(crate) fn index_in_region(
        &mut self,
        cursor: &mut Cursor,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            self.index(cursor, style);
            return;
        };

        if cursor.row == bottom {
            self.scroll_up_region(top, bottom, style);
        } else if cursor.row >= top && cursor.row < bottom {
            cursor.row += 1;
        } else {
            self.index(cursor, style);
        }
    }

    pub(crate) fn reverse_index_in_region(
        &mut self,
        cursor: &mut Cursor,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            self.reverse_index(cursor, style);
            return;
        };

        if cursor.row == top {
            self.scroll_down_region(top, bottom, style);
        } else if cursor.row > top && cursor.row <= bottom {
            cursor.row -= 1;
        } else {
            self.reverse_index(cursor, style);
        }
    }

    pub(crate) fn backspace(&mut self, cursor: &mut Cursor) {
        if cursor.col > 0 {
            cursor.col -= 1;
            if self.lines[cursor.row][cursor.col].is_wide_continuation() && cursor.col > 0 {
                self.lines[cursor.row][cursor.col].clear();
                cursor.col -= 1;
            }
            self.lines[cursor.row][cursor.col].clear();
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

    pub(crate) fn clear_line_from_cursor(&mut self, cursor: Cursor, style: Style) {
        for col in cursor.col..self.cols {
            self.lines[cursor.row][col].clear_with_style(style);
        }
    }

    pub(crate) fn clear_line_to_cursor(&mut self, cursor: Cursor, style: Style) {
        for col in 0..=cursor.col.min(self.cols - 1) {
            self.lines[cursor.row][col].clear_with_style(style);
        }
    }

    pub(crate) fn clear_entire_line(&mut self, row: usize, style: Style) {
        let row = row.min(self.rows - 1);
        for col in 0..self.cols {
            self.lines[row][col].clear_with_style(style);
        }
    }

    pub(crate) fn insert_blank_chars(&mut self, cursor: Cursor, count: usize, style: Style) {
        let row = cursor.row.min(self.rows - 1);
        let col = cursor.col.min(self.cols - 1);
        let count = count.max(1).min(self.cols - col);
        for _ in 0..count {
            self.lines[row].insert(col, Cell::blank_with_style(style));
            self.lines[row].pop();
        }
    }

    pub(crate) fn delete_chars(&mut self, cursor: Cursor, count: usize, style: Style) {
        let row = cursor.row.min(self.rows - 1);
        let col = cursor.col.min(self.cols - 1);
        let count = count.max(1).min(self.cols - col);
        for _ in 0..count {
            self.lines[row].remove(col);
            self.lines[row].push(Cell::blank_with_style(style));
        }
    }

    pub(crate) fn erase_chars(&mut self, cursor: Cursor, count: usize, style: Style) {
        let row = cursor.row.min(self.rows - 1);
        let col = cursor.col.min(self.cols - 1);
        let end = (col + count.max(1)).min(self.cols);
        for cell in &mut self.lines[row][col..end] {
            cell.clear_with_style(style);
        }
    }

    pub(crate) fn insert_blank_lines(
        &mut self,
        cursor: Cursor,
        count: usize,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            return;
        };
        if cursor.row < top || cursor.row > bottom {
            return;
        }

        let count = count.max(1).min(bottom - cursor.row + 1);
        for _ in 0..count {
            self.lines
                .insert(cursor.row, blank_line_with_style(self.cols, style));
            self.lines.remove(bottom + 1);
        }
    }

    pub(crate) fn delete_lines(
        &mut self,
        cursor: Cursor,
        count: usize,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            return;
        };
        if cursor.row < top || cursor.row > bottom {
            return;
        }

        let count = count.max(1).min(bottom - cursor.row + 1);
        for _ in 0..count {
            self.lines.remove(cursor.row);
            self.lines
                .insert(bottom, blank_line_with_style(self.cols, style));
        }
    }

    pub(crate) fn scroll_up_lines(
        &mut self,
        count: usize,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            return;
        };
        for _ in 0..count.max(1).min(bottom - top + 1) {
            self.scroll_up_region(top, bottom, style);
        }
    }

    pub(crate) fn scroll_down_lines(
        &mut self,
        count: usize,
        region: Option<(usize, usize)>,
        style: Style,
    ) {
        let Some((top, bottom)) = normalized_region(region, self.rows) else {
            return;
        };
        for _ in 0..count.max(1).min(bottom - top + 1) {
            self.scroll_down_region(top, bottom, style);
        }
    }

    pub(crate) fn clear_screen_from_cursor(&mut self, cursor: Cursor, style: Style) {
        self.clear_line_from_cursor(cursor, style);
        for row in (cursor.row + 1)..self.rows {
            self.clear_entire_line(row, style);
        }
    }

    pub(crate) fn clear_screen_to_cursor(&mut self, cursor: Cursor, style: Style) {
        for row in 0..cursor.row {
            self.clear_entire_line(row, style);
        }
        self.clear_line_to_cursor(cursor, style);
    }

    pub(crate) fn clear_screen(&mut self, cursor: &mut Cursor, style: Style) {
        for line in &mut self.lines {
            for cell in line {
                cell.clear_with_style(style);
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
            .map(|line| render_line(line))
            .collect()
    }

    pub(crate) fn visible_styled_lines_from(
        &self,
        start: usize,
        max_visible_lines: usize,
    ) -> Vec<StyledLine> {
        let max_visible_lines = max_visible_lines.max(1);
        let start = start.min(self.rows.saturating_sub(1));
        let end = (start + max_visible_lines).min(self.rows);
        self.lines[start..end]
            .iter()
            .map(|line| render_styled_line(line))
            .collect()
    }

    pub(crate) fn scrollback_lines(
        &self,
        offset_from_bottom: usize,
        max_lines: usize,
    ) -> Vec<String> {
        if self.scrollback.is_empty() || max_lines == 0 {
            return Vec::new();
        }

        let end = self.scrollback.len().saturating_sub(offset_from_bottom);
        let start = end.saturating_sub(max_lines);
        self.scrollback[start..end]
            .iter()
            .map(|line| render_line(line))
            .collect()
    }

    pub(crate) fn scrollback_styled_lines(
        &self,
        offset_from_bottom: usize,
        max_lines: usize,
    ) -> Vec<StyledLine> {
        if self.scrollback.is_empty() || max_lines == 0 {
            return Vec::new();
        }

        let end = self.scrollback.len().saturating_sub(offset_from_bottom);
        let start = end.saturating_sub(max_lines);
        self.scrollback[start..end]
            .iter()
            .map(|line| render_styled_line(line))
            .collect()
    }

    fn scroll_up(&mut self) {
        let line = self.lines.remove(0);
        self.push_scrollback(line);
        self.lines.push(blank_line(self.cols));
    }

    fn index(&mut self, cursor: &mut Cursor, style: Style) {
        if cursor.row + 1 >= self.rows {
            self.scroll_up_with_style(style);
        } else {
            cursor.row += 1;
        }
    }

    fn reverse_index(&mut self, cursor: &mut Cursor, style: Style) {
        if cursor.row == 0 {
            self.scroll_down_region(0, self.rows - 1, style);
        } else {
            cursor.row -= 1;
        }
    }

    fn scroll_up_with_style(&mut self, style: Style) {
        let line = self.lines.remove(0);
        self.push_scrollback(line);
        self.lines.push(blank_line_with_style(self.cols, style));
    }

    fn scroll_up_region(&mut self, top: usize, bottom: usize, style: Style) {
        self.lines.remove(top);
        self.lines
            .insert(bottom, blank_line_with_style(self.cols, style));
    }

    fn scroll_down_region(&mut self, top: usize, bottom: usize, style: Style) {
        self.lines.remove(bottom);
        self.lines
            .insert(top, blank_line_with_style(self.cols, style));
    }

    fn push_scrollback(&mut self, line: Vec<Cell>) {
        self.scrollback.push(line);
        if self.scrollback.len() > self.max_scrollback_lines {
            let overflow = self.scrollback.len() - self.max_scrollback_lines;
            self.scrollback.drain(0..overflow);
        }
    }
}

fn normalized_region(region: Option<(usize, usize)>, rows: usize) -> Option<(usize, usize)> {
    let (top, bottom) = region.unwrap_or((0, rows.saturating_sub(1)));
    if rows == 0 {
        return None;
    }
    let top = top.min(rows - 1);
    let bottom = bottom.min(rows - 1);
    (bottom > top).then_some((top, bottom))
}

fn blank_line(cols: usize) -> Vec<Cell> {
    vec![Cell::blank(); cols]
}

fn blank_line_with_style(cols: usize, style: Style) -> Vec<Cell> {
    vec![Cell::blank_with_style(style); cols]
}

fn resize_line(line: &mut Vec<Cell>, cols: usize) {
    if line.len() > cols {
        line.truncate(cols);
    } else {
        line.extend((0..(cols - line.len())).map(|_| Cell::blank()));
    }
}

fn render_line(line: &[Cell]) -> String {
    let mut text = String::with_capacity(line.len());
    for cell in line {
        if !cell.is_wide_continuation() {
            text.push(cell.ch());
        }
    }
    trim_trailing_blanks(text)
}

fn render_styled_line(line: &[Cell]) -> StyledLine {
    let end = last_nonblank_cell_index(line);
    let mut spans: Vec<StyledSpan> = Vec::new();

    for cell in &line[..end] {
        if cell.is_wide_continuation() {
            continue;
        }

        if let Some(last) = spans.last_mut() {
            if last.style == cell.style() {
                last.text.push(cell.ch());
                continue;
            }
        }

        spans.push(StyledSpan {
            text: cell.ch().to_string(),
            style: cell.style(),
        });
    }

    StyledLine { spans }
}

fn last_nonblank_cell_index(line: &[Cell]) -> usize {
    line.iter()
        .rposition(|cell| {
            cell.ch() != ' ' || cell.is_wide_continuation() || cell_has_visible_style(cell)
        })
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn cell_has_visible_style(cell: &Cell) -> bool {
    cell.style().background.is_some() || cell.style().inverse
}

fn trim_trailing_blanks(mut text: String) -> String {
    while text.ends_with(' ') {
        text.pop();
    }
    text
}

fn char_width(ch: char) -> usize {
    if is_combining_mark(ch) {
        0
    } else if is_wide_char(ch) {
        2
    } else {
        1
    }
}

fn is_combining_mark(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0300..=0x036F
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x20D0..=0x20FF
            | 0xFE20..=0xFE2F
    )
}

fn is_wide_char(ch: char) -> bool {
    matches!(
        ch as u32,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1FAFF
    )
}
