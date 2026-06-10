#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GridPoint {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct SelectionState {
    anchor: Option<GridPoint>,
    active: Option<GridPoint>,
}

impl SelectionState {
    pub(crate) fn begin(&mut self, point: GridPoint) {
        self.anchor = Some(point);
        self.active = Some(point);
    }

    pub(crate) fn update(&mut self, point: GridPoint) {
        if self.anchor.is_some() {
            self.active = Some(point);
        }
    }

    pub(crate) fn shift_rows(&mut self, delta: isize, max_row: usize) {
        self.anchor = self
            .anchor
            .map(|point| shift_point_row(point, delta, max_row));
        self.active = self
            .active
            .map(|point| shift_point_row(point, delta, max_row));
    }

    pub(crate) fn clear(&mut self) {
        self.anchor = None;
        self.active = None;
    }

    pub(crate) fn range(&self) -> Option<SelectionRange> {
        let anchor = self.anchor?;
        let active = self.active?;
        if anchor == active {
            return None;
        }

        Some(SelectionRange::new(anchor, active))
    }
}

fn shift_point_row(point: GridPoint, delta: isize, max_row: usize) -> GridPoint {
    let row = if delta.is_positive() {
        point.row.saturating_add(delta as usize)
    } else {
        point.row.saturating_sub(delta.unsigned_abs())
    };
    GridPoint {
        row: row.min(max_row),
        col: point.col,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SelectionRange {
    pub start: GridPoint,
    pub end: GridPoint,
}

impl SelectionRange {
    fn new(a: GridPoint, b: GridPoint) -> Self {
        if (a.row, a.col) <= (b.row, b.col) {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        }
    }
}

pub(crate) fn selected_text(lines: &[String], range: SelectionRange) -> String {
    let mut output = String::new();

    for row in range.start.row..=range.end.row {
        let Some(line) = lines.get(row) else {
            continue;
        };

        let start_col = if row == range.start.row {
            range.start.col
        } else {
            0
        };
        let end_col = if row == range.end.row {
            range.end.col
        } else {
            display_width(line)
        };

        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&slice_display_width(line, start_col, end_col));
    }

    output
}

fn slice_display_width(text: &str, start_col: usize, end_col: usize) -> String {
    let mut output = String::new();
    let mut col = 0;

    for ch in text.chars() {
        let width = char_width(ch);
        let next_col = col + width;
        if width == 0 {
            if col > start_col && col <= end_col {
                output.push(ch);
            }
        } else if next_col > start_col && col < end_col {
            output.push(ch);
        }
        col = next_col;
    }

    output
}

pub(crate) fn display_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

pub(crate) fn char_width(ch: char) -> usize {
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

#[cfg(test)]
mod tests {
    use super::{selected_text, GridPoint, SelectionRange, SelectionState};

    #[test]
    fn normalizes_selection_range() {
        let range = SelectionRange::new(GridPoint { row: 2, col: 3 }, GridPoint { row: 1, col: 4 });

        assert_eq!(range.start, GridPoint { row: 1, col: 4 });
        assert_eq!(range.end, GridPoint { row: 2, col: 3 });
    }

    #[test]
    fn extracts_selected_text_across_lines() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 1 }, GridPoint { row: 1, col: 3 }),
        );

        assert_eq!(text, "ello\nwor");
    }

    #[test]
    fn preserves_wide_characters_in_selected_text() {
        let lines = vec!["A한B".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 1 }, GridPoint { row: 0, col: 3 }),
        );

        assert_eq!(text, "한");
    }

    #[test]
    fn preserves_wide_character_when_boundary_splits_cell() {
        let lines = vec!["한글".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 1 }, GridPoint { row: 0, col: 2 }),
        );

        assert_eq!(text, "한");
    }

    #[test]
    fn preserves_combining_marks_attached_to_selected_base() {
        let lines = vec!["e\u{301}x".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 0 }, GridPoint { row: 0, col: 1 }),
        );

        assert_eq!(text, "e\u{301}");
    }

    #[test]
    fn extracts_text_across_empty_lines() {
        let lines = vec!["one".to_string(), String::new(), "three".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 1 }, GridPoint { row: 2, col: 2 }),
        );

        assert_eq!(text, "ne\n\nth");
    }

    #[test]
    fn clamps_selection_past_line_width() {
        let lines = vec!["short".to_string()];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 1 }, GridPoint { row: 0, col: 99 }),
        );

        assert_eq!(text, "hort");
    }

    #[test]
    fn extracts_text_across_scrollback_live_boundary_snapshot() {
        let lines = vec![
            "scrollback".to_string(),
            "live-one".to_string(),
            "live-two".to_string(),
        ];
        let text = selected_text(
            &lines,
            SelectionRange::new(GridPoint { row: 0, col: 6 }, GridPoint { row: 2, col: 4 }),
        );

        assert_eq!(text, "back\nlive-one\nlive");
    }

    #[test]
    fn inactive_until_drag_has_extent() {
        let mut state = SelectionState::default();
        state.begin(GridPoint { row: 0, col: 0 });
        assert!(state.range().is_none());

        state.update(GridPoint { row: 0, col: 2 });
        assert!(state.range().is_some());
    }

    #[test]
    fn shifts_selection_rows_when_viewport_scrolls() {
        let mut state = SelectionState::default();
        state.begin(GridPoint { row: 2, col: 1 });
        state.update(GridPoint { row: 4, col: 3 });

        state.shift_rows(1, 9);
        assert_eq!(
            state.range(),
            Some(SelectionRange::new(
                GridPoint { row: 3, col: 1 },
                GridPoint { row: 5, col: 3 },
            ))
        );

        state.shift_rows(-10, 9);
        assert_eq!(
            state.range(),
            Some(SelectionRange::new(
                GridPoint { row: 0, col: 1 },
                GridPoint { row: 0, col: 3 },
            ))
        );
    }
}
