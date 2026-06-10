use std::cell::{Cell, OnceCell, RefCell};
use std::sync::{Arc, Mutex};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyObject, Sel};
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSBackgroundColorAttributeName, NSColor, NSEvent, NSFont, NSFontAttributeName,
    NSForegroundColorAttributeName, NSPasteboard, NSPasteboardTypeString, NSRectFill, NSResponder,
    NSTextInputClient, NSUnderlineStyleAttributeName, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSAttributedString, NSAttributedStringKey, NSMutableDictionary,
    NSNotFound, NSNumber, NSObjectProtocol, NSPoint, NSRange, NSRangePointer, NSRect, NSString,
    NSTimer,
};
use terminal_core::{Color, CursorStyle, Style, StyledLine, TerminalModes, TerminalSnapshot};

use crate::composition::{CompositionState, TextRange};
use crate::input;
use crate::logging;
use crate::mouse;
use crate::paste;
use crate::pty::PtyWriter;
use crate::selection::{selected_text, GridPoint, SelectionRange, SelectionState};
use crate::terminal_buffer::TerminalBuffer;

const PADDING_X: f64 = 12.0;
const PADDING_Y: f64 = 14.0;
const FONT_SIZE: f64 = 14.0;
const LINE_HEIGHT: f64 = 18.0;
const CELL_WIDTH: f64 = 8.4;
const CURSOR_WIDTH: f64 = 8.0;
const CURSOR_HEIGHT: f64 = 16.0;
const KEY_RETURN: u16 = 36;
const KEY_KEYPAD_ENTER: u16 = 76;
const KEY_PAGE_UP: u16 = 116;
const KEY_PAGE_DOWN: u16 = 121;
const TERMINAL_FONT_NAMES: &[&str] = &[
    "JetBrainsMonoNFM-Regular",
    "JetBrainsMonoNF-Regular",
    "JetBrainsMono-Regular",
];

pub(crate) struct TerminalViewIvars {
    buffer: Arc<Mutex<TerminalBuffer>>,
    writer: PtyWriter,
    timer: OnceCell<Retained<NSTimer>>,
    logged_first_key_input: Cell<bool>,
    rows: Cell<usize>,
    cols: Cell<usize>,
    scrollback_offset: Cell<usize>,
    composition: RefCell<CompositionState>,
    selection: RefCell<SelectionState>,
}

define_class!(
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = TerminalViewIvars]
    pub(crate) struct TerminalView;

    unsafe impl NSObjectProtocol for TerminalView {}
    unsafe impl NSTextInputClient for TerminalView {
        #[allow(non_snake_case)]
        #[unsafe(method(insertText:replacementRange:))]
        unsafe fn insertText_replacementRange(
            &self,
            string: &AnyObject,
            _replacement_range: NSRange,
        ) {
            autoreleasepool(|pool| {
                let Some(text) = text_from_input_object(string, pool) else {
                    return;
                };

                if text.is_empty() {
                    return;
                }

                self.ivars().composition.borrow_mut().clear();
                self.write_text_to_pty(&text, "insertText");
                self.ivars().scrollback_offset.set(0);
                self.as_super().setNeedsDisplay(true);
            });
        }

        #[allow(non_snake_case)]
        #[unsafe(method(doCommandBySelector:))]
        unsafe fn doCommandBySelector(&self, selector: Sel) {
            if selector == sel!(deleteBackward:) {
                self.ivars().composition.borrow_mut().clear();
                if let Err(error) = self.ivars().writer.write_all(&[0x7f]) {
                    logging::pty_error(&format!("pty write failed from deleteBackward: {error}"));
                }
                self.as_super().setNeedsDisplay(true);
            }
        }

        #[allow(non_snake_case)]
        #[unsafe(method(setMarkedText:selectedRange:replacementRange:))]
        unsafe fn setMarkedText_selectedRange_replacementRange(
            &self,
            string: &AnyObject,
            selected_range: NSRange,
            _replacement_range: NSRange,
        ) {
            autoreleasepool(|pool| {
                let Some(text) = text_from_input_object(string, pool) else {
                    return;
                };

                self.ivars().composition.borrow_mut().set_marked_text(
                    text,
                    TextRange {
                        location: selected_range.location,
                        length: selected_range.length,
                    },
                );
                self.as_super().setNeedsDisplay(true);
            });
        }

        #[allow(non_snake_case)]
        #[unsafe(method(unmarkText))]
        fn unmarkText(&self) {
            self.ivars().composition.borrow_mut().clear();
            self.as_super().setNeedsDisplay(true);
        }

        #[allow(non_snake_case)]
        #[unsafe(method(selectedRange))]
        fn selectedRange(&self) -> NSRange {
            let range = self.ivars().composition.borrow().selected_range();
            NSRange::new(range.location, range.length)
        }

        #[allow(non_snake_case)]
        #[unsafe(method(markedRange))]
        fn markedRange(&self) -> NSRange {
            let range = self.ivars().composition.borrow().marked_range();
            if range.location == usize::MAX {
                NSRange::new(NSNotFound as usize, 0)
            } else {
                NSRange::new(range.location, range.length)
            }
        }

        #[allow(non_snake_case)]
        #[unsafe(method(hasMarkedText))]
        fn hasMarkedText(&self) -> bool {
            self.ivars().composition.borrow().has_marked_text()
        }

        #[allow(non_snake_case)]
        #[unsafe(method_id(attributedSubstringForProposedRange:actualRange:))]
        unsafe fn attributedSubstringForProposedRange_actualRange(
            &self,
            _range: NSRange,
            actual_range: NSRangePointer,
        ) -> Option<Retained<NSAttributedString>> {
            unsafe {
                if !actual_range.is_null() {
                    *actual_range = NSRange::new(NSNotFound as usize, 0);
                }
            }
            None
        }

        #[allow(non_snake_case)]
        #[unsafe(method_id(validAttributesForMarkedText))]
        fn validAttributesForMarkedText(&self) -> Retained<NSArray<NSAttributedStringKey>> {
            NSArray::from_slice(&[])
        }

        #[allow(non_snake_case)]
        #[unsafe(method(firstRectForCharacterRange:actualRange:))]
        unsafe fn firstRectForCharacterRange_actualRange(
            &self,
            range: NSRange,
            actual_range: NSRangePointer,
        ) -> NSRect {
            unsafe {
                if !actual_range.is_null() {
                    *actual_range = range;
                }
            }
            self.cursor_screen_rect()
        }

        #[allow(non_snake_case)]
        #[unsafe(method(characterIndexForPoint:))]
        fn characterIndexForPoint(&self, _point: NSPoint) -> usize {
            0
        }
    }

    impl TerminalView {
        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }

        #[unsafe(method(acceptsFirstResponder))]
        fn accepts_first_responder(&self) -> bool {
            true
        }

        #[unsafe(method(keyDown:))]
        fn key_down(&self, event: &NSEvent) {
            autoreleasepool(|pool| {
                if is_command_c(event) {
                    self.copy_selection_to_clipboard();
                    return;
                }

                if is_command_v(event) {
                    self.paste_text_from_clipboard(pool);
                    return;
                }

                if self.handle_scrollback_key(event) {
                    return;
                }

                if should_use_text_input(event) {
                    self.interpret_text_input_event(event);
                    return;
                }

                let Some(characters) = event.characters() else {
                    return;
                };
                let input = unsafe { characters.to_str(pool) };
                let modes = self.current_modes();
                let bytes = modes
                    .application_keypad
                    .then(|| input::encode_application_keypad_key_event(event))
                    .flatten()
                    .or_else(|| {
                        modes
                            .application_cursor_keys
                            .then(|| input::encode_application_cursor_key_event(event))
                            .flatten()
                    })
                    .or_else(|| input::encode_key_event(event, input));
                let Some(bytes) = bytes else {
                    return;
                };

                if !self.ivars().logged_first_key_input.replace(true) {
                    logging::pty_info(&format!(
                        "terminal view first key input received: bytes={}",
                        bytes.len()
                    ));
                }

                if let Err(error) = self.ivars().writer.write_all(&bytes) {
                    logging::pty_error(&format!("pty write failed from keyDown: {error}"));
                }

                self.ivars().selection.borrow_mut().clear();
                self.ivars().scrollback_offset.set(0);
            });
        }

        #[unsafe(method(mouseDown:))]
        fn mouse_down(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if self.write_mouse_report(mouse::LEFT_BUTTON, point, false) {
                self.ivars().selection.borrow_mut().clear();
                self.ivars().scrollback_offset.set(0);
                return;
            }

            self.ivars().selection.borrow_mut().begin(point);
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if self.write_mouse_report(mouse::LEFT_DRAG, point, false) {
                return;
            }

            self.ivars().selection.borrow_mut().update(point);
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(mouseUp:))]
        fn mouse_up(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if self.write_mouse_report(mouse::LEFT_BUTTON, point, true) {
                return;
            }

            self.ivars().selection.borrow_mut().update(point);
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if event.scrollingDeltaY() > 0.0
                && self.write_mouse_report(mouse::WHEEL_UP, point, false)
            {
                return;
            }
            if event.scrollingDeltaY() < 0.0
                && self.write_mouse_report(mouse::WHEEL_DOWN, point, false)
            {
                return;
            }

            let rows = (event.scrollingDeltaY().abs() / LINE_HEIGHT).ceil().max(1.0) as usize;
            if event.scrollingDeltaY() > 0.0 {
                self.adjust_scrollback(rows as isize);
            } else if event.scrollingDeltaY() < 0.0 {
                self.adjust_scrollback(-(rows as isize));
            }
        }

        #[unsafe(method(paste:))]
        fn paste(&self, _sender: Option<&AnyObject>) {
            autoreleasepool(|pool| {
                self.paste_text_from_clipboard(pool);
            });
        }

        #[unsafe(method(drawRect:))]
        fn draw_rect(&self, dirty_rect: NSRect) {
            draw_background(dirty_rect);
            let bounds = self.as_super().bounds();
            let (rows, cols) = terminal_dimensions(bounds);
            self.apply_resize_if_needed(rows, cols);

            let snapshot = self.current_snapshot(rows);

            draw_selection_highlights(&snapshot, self.ivars().selection.borrow().range());
            draw_terminal_text(&snapshot);
            draw_composition_text(&snapshot, &self.ivars().composition.borrow());
            if self.ivars().scrollback_offset.get() == 0 {
                draw_cursor(&snapshot);
            }
        }

        #[unsafe(method(redrawTimerFired:))]
        fn redraw_timer_fired(&self, _timer: &NSTimer) {
            self.apply_pending_clipboard_writes();
            self.as_super().setNeedsDisplay(true);
        }
    }
);

impl TerminalView {
    pub fn new(
        mtm: MainThreadMarker,
        frame: NSRect,
        buffer: Arc<Mutex<TerminalBuffer>>,
        writer: PtyWriter,
    ) -> Retained<Self> {
        let view = Self::alloc(mtm).set_ivars(TerminalViewIvars {
            buffer,
            writer,
            timer: OnceCell::new(),
            logged_first_key_input: Cell::new(false),
            rows: Cell::new(0),
            cols: Cell::new(0),
            scrollback_offset: Cell::new(0),
            composition: RefCell::new(CompositionState::default()),
            selection: RefCell::new(SelectionState::default()),
        });
        let view: Retained<Self> = unsafe { msg_send![super(view), initWithFrame: frame] };

        let timer = unsafe {
            NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
                0.033,
                &view,
                sel!(redrawTimerFired:),
                None,
                true,
            )
        };
        let _ = view.ivars().timer.set(timer);

        view
    }

    pub fn focus(&self, window: &NSWindow) {
        let responder: &NSResponder = self.as_super().as_super();
        window.makeFirstResponder(Some(responder));
    }

    fn paste_text_from_clipboard(&self, pool: objc2::rc::AutoreleasePool<'_>) {
        let pasteboard = NSPasteboard::generalPasteboard();
        let Some(text) = pasteboard.stringForType(unsafe { &*NSPasteboardTypeString }) else {
            return;
        };

        let input = unsafe { text.to_str(pool) };
        if input.is_empty() {
            return;
        }

        logging::pty_info(&format!(
            "terminal view paste received: bytes={}",
            input.len()
        ));

        let bytes = if self.current_modes().bracketed_paste {
            paste::bracketed_paste_bytes(input)
        } else {
            input.as_bytes().to_vec()
        };

        if let Err(error) = self.ivars().writer.write_all(&bytes) {
            logging::pty_error(&format!("pty write failed from paste: {error}"));
        }

        self.ivars().selection.borrow_mut().clear();
        self.ivars().scrollback_offset.set(0);
    }

    fn apply_pending_clipboard_writes(&self) {
        let clipboard_writes = match self.ivars().buffer.lock() {
            Ok(mut buffer) => buffer.take_pending_clipboard_writes(),
            Err(_) => {
                logging::pty_error("terminal buffer lock poisoned while draining OSC 52 clipboard");
                return;
            }
        };

        if clipboard_writes.is_empty() {
            return;
        }

        let pasteboard = NSPasteboard::generalPasteboard();
        for text in clipboard_writes {
            pasteboard.clearContents();
            pasteboard.setString_forType(&NSString::from_str(&text), unsafe {
                &*NSPasteboardTypeString
            });
        }
        logging::pty_info("terminal view applied OSC 52 clipboard write");
    }

    fn write_text_to_pty(&self, text: &str, source: &str) {
        if !self.ivars().logged_first_key_input.replace(true) {
            logging::pty_info(&format!(
                "terminal view first text input received: source={source} bytes={}",
                text.len()
            ));
        }

        if let Err(error) = self.ivars().writer.write_all(text.as_bytes()) {
            logging::pty_error(&format!("pty write failed from {source}: {error}"));
        }
    }

    fn interpret_text_input_event(&self, event: &NSEvent) {
        let events = NSArray::from_slice(&[event]);
        let responder: &NSResponder = self.as_super().as_super();
        responder.interpretKeyEvents(&events);
    }

    fn copy_selection_to_clipboard(&self) {
        let Some(range) = self.ivars().selection.borrow().range() else {
            return;
        };
        let snapshot = self.current_snapshot(self.ivars().rows.get().max(1));
        let text = selected_text(&snapshot.lines, range);
        if text.is_empty() {
            return;
        }

        let pasteboard = NSPasteboard::generalPasteboard();
        pasteboard.clearContents();
        pasteboard.setString_forType(&NSString::from_str(&text), unsafe {
            &*NSPasteboardTypeString
        });
        logging::pty_info(&format!("terminal selection copied: bytes={}", text.len()));
    }

    fn current_snapshot(&self, rows: usize) -> TerminalSnapshot {
        self.ivars()
            .buffer
            .lock()
            .map(|buffer| {
                let offset = self.ivars().scrollback_offset.get();
                buffer.combined_snapshot(offset, rows)
            })
            .unwrap_or_else(|_| TerminalSnapshot {
                lines: vec!["terminal buffer unavailable".to_string()],
                styled_lines: Vec::new(),
                cursor: terminal_core::Cursor::default(),
                modes: TerminalModes::default(),
                scrollback_len: 0,
            })
    }

    fn current_modes(&self) -> TerminalModes {
        self.ivars()
            .buffer
            .lock()
            .map(|buffer| buffer.snapshot(self.ivars().rows.get().max(1)).modes)
            .unwrap_or_default()
    }

    fn write_mouse_report(&self, code: u16, point: GridPoint, release: bool) -> bool {
        let modes = self.current_modes();
        if !modes.mouse_reporting {
            return false;
        }

        let bytes = if modes.sgr_mouse {
            mouse::sgr_mouse_report(code, point.row, point.col, release)
        } else {
            mouse::legacy_mouse_report(code, point.row, point.col, release)
        };
        if let Err(error) = self.ivars().writer.write_all(&bytes) {
            logging::pty_error(&format!("pty write failed from mouse report: {error}"));
        }
        true
    }

    fn grid_point_for_event(&self, event: &NSEvent) -> GridPoint {
        let point = self
            .as_super()
            .convertPoint_fromView(event.locationInWindow(), None);
        let row = ((point.y - PADDING_Y) / LINE_HEIGHT).floor().max(0.0) as usize;
        let col = ((point.x - PADDING_X) / CELL_WIDTH).floor().max(0.0) as usize;

        GridPoint {
            row: row.min(self.ivars().rows.get().saturating_sub(1)),
            col: col.min(self.ivars().cols.get()),
        }
    }

    fn cursor_screen_rect(&self) -> NSRect {
        let snapshot = self
            .ivars()
            .buffer
            .lock()
            .map(|buffer| buffer.snapshot(self.ivars().rows.get().max(1)))
            .unwrap_or_else(|_| TerminalSnapshot {
                lines: Vec::new(),
                styled_lines: Vec::new(),
                cursor: terminal_core::Cursor::default(),
                modes: TerminalModes::default(),
                scrollback_len: 0,
            });
        let view_rect = cursor_rect(&snapshot);
        let Some(window) = self.as_super().window() else {
            return view_rect;
        };
        window.convertRectToScreen(view_rect)
    }

    fn apply_resize_if_needed(&self, rows: usize, cols: usize) {
        if self.ivars().rows.get() == rows && self.ivars().cols.get() == cols {
            return;
        }

        if let Ok(mut buffer) = self.ivars().buffer.lock() {
            buffer.resize(rows, cols);
        }

        if let Err(error) = self.ivars().writer.resize(rows, cols) {
            logging::pty_error(&format!("pty resize failed: {error}"));
        }

        self.ivars().rows.set(rows);
        self.ivars().cols.set(cols);
        logging::pty_info(&format!("terminal resized: rows={rows} cols={cols}"));
    }

    fn handle_scrollback_key(&self, event: &NSEvent) -> bool {
        match event.keyCode() {
            KEY_PAGE_UP => {
                self.scroll_page_up();
                true
            }
            KEY_PAGE_DOWN => {
                self.scroll_page_down();
                true
            }
            _ => false,
        }
    }

    fn scroll_page_up(&self) {
        let rows = self.ivars().rows.get().max(1);
        self.adjust_scrollback(rows as isize);
    }

    fn scroll_page_down(&self) {
        let rows = self.ivars().rows.get().max(1);
        self.adjust_scrollback(-(rows as isize));
    }

    fn adjust_scrollback(&self, delta_rows: isize) {
        let max_offset = self
            .ivars()
            .buffer
            .lock()
            .map(|buffer| buffer.scrollback_len())
            .unwrap_or(0);
        let current = self.ivars().scrollback_offset.get();
        let next_offset = if delta_rows.is_positive() {
            current.saturating_add(delta_rows as usize).min(max_offset)
        } else {
            current.saturating_sub(delta_rows.unsigned_abs())
        };
        self.ivars().scrollback_offset.set(next_offset);
        self.as_super().setNeedsDisplay(true);
    }
}

fn is_command_v(event: &NSEvent) -> bool {
    let flags = event.modifierFlags();
    let key_code = event.keyCode();
    key_code == 9 && flags.contains(objc2_app_kit::NSEventModifierFlags::Command)
}

fn is_command_c(event: &NSEvent) -> bool {
    let flags = event.modifierFlags();
    let key_code = event.keyCode();
    key_code == 8 && flags.contains(objc2_app_kit::NSEventModifierFlags::Command)
}

fn should_use_text_input(event: &NSEvent) -> bool {
    let flags = event.modifierFlags();
    if flags.intersects(
        objc2_app_kit::NSEventModifierFlags::Command
            | objc2_app_kit::NSEventModifierFlags::Control
            | objc2_app_kit::NSEventModifierFlags::Option,
    ) {
        return false;
    }

    !matches!(
        event.keyCode(),
        KEY_RETURN
            | KEY_KEYPAD_ENTER
            | KEY_PAGE_UP
            | KEY_PAGE_DOWN
            | 51
            | 117
            | 115
            | 119
            | 123
            | 124
            | 125
            | 126
    )
}

fn draw_background(rect: NSRect) {
    NSColor::blackColor().setFill();
    NSRectFill(rect);
}

fn draw_terminal_text(snapshot: &TerminalSnapshot) {
    if snapshot.styled_lines.is_empty() {
        draw_plain_terminal_text(&snapshot.lines);
        return;
    }

    for (index, line) in snapshot.styled_lines.iter().enumerate() {
        let y = PADDING_Y + (index as f64 * LINE_HEIGHT);
        draw_styled_line(line, y);
    }
}

fn draw_selection_highlights(snapshot: &TerminalSnapshot, range: Option<SelectionRange>) {
    let Some(range) = range else {
        return;
    };

    NSColor::selectedTextBackgroundColor().setFill();
    for row in range.start.row..=range.end.row {
        let Some(line) = snapshot.lines.get(row) else {
            continue;
        };
        let line_width = display_width(line);
        let start_col = if row == range.start.row {
            range.start.col.min(line_width)
        } else {
            0
        };
        let end_col = if row == range.end.row {
            range.end.col.min(line_width)
        } else {
            line_width
        };
        if end_col <= start_col {
            continue;
        }

        let x = PADDING_X + (start_col as f64 * CELL_WIDTH);
        let y = PADDING_Y + (row as f64 * LINE_HEIGHT);
        let width = (end_col - start_col) as f64 * CELL_WIDTH;
        NSRectFill(NSRect::new(
            NSPoint::new(x, y),
            objc2_foundation::NSSize::new(width, LINE_HEIGHT),
        ));
    }
}

fn draw_composition_text(snapshot: &TerminalSnapshot, composition: &CompositionState) {
    if !composition.has_marked_text() {
        return;
    }

    let mut style = Style {
        underline: true,
        ..Style::default()
    };
    style.foreground = Some(Color::Indexed(11));
    let attributes = terminal_text_attributes(style);
    let rect = cursor_rect(snapshot);
    draw_text_cells_at(
        composition.marked_text(),
        rect.origin.x,
        rect.origin.y - 2.0,
        &attributes,
    );
}

fn draw_plain_terminal_text(lines: &[String]) {
    let attributes = terminal_text_attributes(Style::default());

    for (index, line) in lines.iter().enumerate() {
        draw_text_cells_at(
            line,
            PADDING_X,
            PADDING_Y + (index as f64 * LINE_HEIGHT),
            &attributes,
        );
    }
}

fn draw_styled_line(line: &StyledLine, y: f64) {
    let mut x = PADDING_X;

    for span in &line.spans {
        let attributes = terminal_text_attributes(span.style);
        draw_text_cells_at(&span.text, x, y, &attributes);
        x += display_width(&span.text) as f64 * CELL_WIDTH;
    }
}

fn draw_text_cells_at(text: &str, x: f64, y: f64, attributes: &NSMutableDictionary) {
    let mut current_x = x;
    let mut previous_x = x;

    for ch in text.chars() {
        let width = char_width(ch);
        let draw_x = if width == 0 { previous_x } else { current_x };
        draw_text_at(&ch.to_string(), draw_x, y, attributes);

        if width > 0 {
            previous_x = current_x;
            current_x += width as f64 * CELL_WIDTH;
        }
    }
}

fn draw_text_at(text: &str, x: f64, y: f64, attributes: &NSMutableDictionary) {
    let string = NSString::from_str(text);
    let _: () = unsafe {
        msg_send![
            &*string,
            drawAtPoint: NSPoint::new(x, y),
            withAttributes: Some(attributes)
        ]
    };
}

fn draw_cursor(snapshot: &TerminalSnapshot) {
    if !snapshot.modes.cursor_visible {
        return;
    }

    let rect = cursor_rect(snapshot);
    NSColor::whiteColor().setFill();
    NSRectFill(rect);
}

fn cursor_rect(snapshot: &TerminalSnapshot) -> NSRect {
    let x = PADDING_X + (snapshot.cursor.col as f64 * CELL_WIDTH);
    let y = PADDING_Y + (snapshot.cursor.row as f64 * LINE_HEIGHT) + 2.0;
    let (x, y, width, height) = match snapshot.modes.cursor_style {
        CursorStyle::Block => (x, y, CURSOR_WIDTH, CURSOR_HEIGHT),
        CursorStyle::Bar => (x, y, 2.0, CURSOR_HEIGHT),
        CursorStyle::Underline => (x, y + CURSOR_HEIGHT - 2.0, CURSOR_WIDTH, 2.0),
    };
    NSRect::new(
        NSPoint::new(x, y),
        objc2_foundation::NSSize::new(width, height),
    )
}

fn terminal_text_attributes(style: Style) -> Retained<NSMutableDictionary> {
    let attributes = NSMutableDictionary::new();
    let foreground = terminal_foreground_color(style);
    let foreground_object: &AnyObject = foreground.as_ref();

    let _: () = unsafe {
        msg_send![
            &*attributes,
            setObject: foreground_object,
            forKey: &*NSForegroundColorAttributeName
        ]
    };

    if let Some(background) = terminal_background_color(style) {
        let background_object: &AnyObject = background.as_ref();
        let _: () = unsafe {
            msg_send![
                &*attributes,
                setObject: background_object,
                forKey: &*NSBackgroundColorAttributeName
            ]
        };
    }

    if style.underline {
        let underline = NSNumber::new_i32(1);
        let underline_object: &AnyObject = underline.as_ref();
        let _: () = unsafe {
            msg_send![
                &*attributes,
                setObject: underline_object,
                forKey: &*NSUnderlineStyleAttributeName
            ]
        };
    }

    if let Some(font) = terminal_font(style.bold) {
        let font_object: &AnyObject = font.as_ref();
        let _: () = unsafe {
            msg_send![
                &*attributes,
                setObject: font_object,
                forKey: &*NSFontAttributeName
            ]
        };
    }

    attributes
}

fn terminal_font(bold: bool) -> Option<Retained<NSFont>> {
    if bold {
        for font_name in [
            "JetBrainsMonoNFM-Bold",
            "JetBrainsMonoNF-Bold",
            "JetBrainsMono-Bold",
        ] {
            if let Some(font) = NSFont::fontWithName_size(&NSString::from_str(font_name), FONT_SIZE)
            {
                return Some(font);
            }
        }
    }

    for font_name in TERMINAL_FONT_NAMES {
        if let Some(font) = NSFont::fontWithName_size(&NSString::from_str(font_name), FONT_SIZE) {
            return Some(font);
        }
    }

    NSFont::userFixedPitchFontOfSize(FONT_SIZE)
}

fn terminal_foreground_color(style: Style) -> Retained<NSColor> {
    let color = if style.inverse {
        style.background.or(Some(Color::Indexed(0)))
    } else {
        style.foreground
    };
    ns_color(color, true)
}

fn terminal_background_color(style: Style) -> Option<Retained<NSColor>> {
    let color = if style.inverse {
        style.foreground.or(Some(Color::Indexed(7)))
    } else {
        style.background
    };
    color.map(|color| ns_color(Some(color), false))
}

fn ns_color(color: Option<Color>, foreground: bool) -> Retained<NSColor> {
    match color {
        None if foreground => NSColor::whiteColor(),
        None => NSColor::blackColor(),
        Some(Color::Indexed(index)) => indexed_color(index),
        Some(Color::Rgb(red, green, blue)) => rgb_color(red, green, blue),
    }
}

fn indexed_color(index: u8) -> Retained<NSColor> {
    let (red, green, blue) = match index {
        0 => (0x1d, 0x1f, 0x21),
        1 => (0xcc, 0x24, 0x1d),
        2 => (0x98, 0x97, 0x1a),
        3 => (0xd7, 0x99, 0x21),
        4 => (0x45, 0x85, 0x88),
        5 => (0xb1, 0x62, 0x86),
        6 => (0x68, 0x9d, 0x6a),
        7 => (0xeb, 0xdb, 0xb2),
        8 => (0x92, 0x83, 0x74),
        9 => (0xfb, 0x49, 0x34),
        10 => (0xb8, 0xbb, 0x26),
        11 => (0xfa, 0xbd, 0x2f),
        12 => (0x83, 0xa5, 0x98),
        13 => (0xd3, 0x86, 0x9b),
        14 => (0x8e, 0xc0, 0x7c),
        15 => (0xfb, 0xf1, 0xc7),
        16..=231 => color_cube(index - 16),
        232..=255 => {
            let value = 8 + ((index - 232) * 10);
            (value, value, value)
        }
    };

    rgb_color(red, green, blue)
}

fn color_cube(index: u8) -> (u8, u8, u8) {
    let red = index / 36;
    let green = (index % 36) / 6;
    let blue = index % 6;
    (
        cube_component(red),
        cube_component(green),
        cube_component(blue),
    )
}

fn cube_component(value: u8) -> u8 {
    if value == 0 {
        0
    } else {
        55 + (value * 40)
    }
}

fn rgb_color(red: u8, green: u8, blue: u8) -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(
        red as f64 / 255.0,
        green as f64 / 255.0,
        blue as f64 / 255.0,
        1.0,
    )
}

fn display_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
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

fn terminal_dimensions(bounds: NSRect) -> (usize, usize) {
    let available_height = (bounds.size.height - (PADDING_Y * 2.0)).max(LINE_HEIGHT);
    let available_width = (bounds.size.width - (PADDING_X * 2.0)).max(CELL_WIDTH);
    let rows = (available_height / LINE_HEIGHT).floor().max(1.0) as usize;
    let cols = (available_width / CELL_WIDTH).floor().max(1.0) as usize;
    (rows, cols)
}

fn text_from_input_object(
    object: &AnyObject,
    pool: objc2::rc::AutoreleasePool<'_>,
) -> Option<String> {
    if let Some(string) = object.downcast_ref::<NSString>() {
        return Some(unsafe { string.to_str(pool) }.to_string());
    }

    if let Some(attributed) = object.downcast_ref::<NSAttributedString>() {
        let string = attributed.string();
        return Some(unsafe { string.to_str(pool) }.to_string());
    }

    None
}
