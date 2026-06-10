#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct CompositionState {
    marked_text: String,
    selected_range: TextRange,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TextRange {
    pub location: usize,
    pub length: usize,
}

impl CompositionState {
    pub(crate) fn set_marked_text(&mut self, text: String, selected_range: TextRange) {
        self.marked_text = text;
        self.selected_range = selected_range;
    }

    pub(crate) fn clear(&mut self) {
        self.marked_text.clear();
        self.selected_range = TextRange::default();
    }

    pub(crate) fn has_marked_text(&self) -> bool {
        !self.marked_text.is_empty()
    }

    pub(crate) fn marked_text(&self) -> &str {
        &self.marked_text
    }

    pub(crate) fn marked_range(&self) -> TextRange {
        if self.has_marked_text() {
            TextRange {
                location: 0,
                length: self.marked_text.chars().count(),
            }
        } else {
            TextRange {
                location: usize::MAX,
                length: 0,
            }
        }
    }

    pub(crate) fn selected_range(&self) -> TextRange {
        self.selected_range
    }
}

#[cfg(test)]
mod tests {
    use super::{CompositionState, TextRange};

    #[test]
    fn tracks_marked_text() {
        let mut state = CompositionState::default();
        assert!(!state.has_marked_text());

        state.set_marked_text(
            "한".to_string(),
            TextRange {
                location: 1,
                length: 0,
            },
        );

        assert!(state.has_marked_text());
        assert_eq!(state.marked_text(), "한");
        assert_eq!(
            state.marked_range(),
            TextRange {
                location: 0,
                length: 1
            }
        );
        assert_eq!(
            state.selected_range(),
            TextRange {
                location: 1,
                length: 0
            }
        );
    }

    #[test]
    fn clear_resets_marked_text() {
        let mut state = CompositionState::default();
        state.set_marked_text("한".to_string(), TextRange::default());
        state.clear();

        assert!(!state.has_marked_text());
        assert_eq!(state.marked_text(), "");
        assert_eq!(
            state.marked_range(),
            TextRange {
                location: usize::MAX,
                length: 0
            }
        );
    }
}
