use std::cell::{Cell, OnceCell, RefCell};
use std::env;
use std::sync::{Arc, Mutex};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyObject, Sel};
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackgroundColorAttributeName, NSColor, NSEvent, NSEventModifierFlags,
    NSEventType, NSFont, NSFontAttributeName, NSForegroundColorAttributeName, NSPasteboard,
    NSPasteboardTypeString, NSRectFill, NSResponder, NSTextInputClient,
    NSUnderlineStyleAttributeName, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSAttributedString, NSAttributedStringKey, NSInteger,
    NSMutableDictionary, NSNotFound, NSNumber, NSObjectProtocol, NSPoint, NSRange, NSRangePointer,
    NSRect, NSSize, NSString, NSTimer,
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
const FALLBACK_CELL_WIDTH: f64 = 8.4;
const CURSOR_HEIGHT: f64 = 16.0;
const KEY_RETURN: u16 = 36;
const KEY_TAB: u16 = 48;
const KEY_ESCAPE: u16 = 53;
const KEY_KEYPAD_ENTER: u16 = 76;
const KEY_Q: u16 = 12;
const KEY_W: u16 = 13;
const KEY_N: u16 = 45;
const KEY_F1: u16 = 122;
const KEY_F2: u16 = 120;
const KEY_F3: u16 = 99;
const KEY_F4: u16 = 118;
const KEY_F5: u16 = 96;
const KEY_F6: u16 = 97;
const KEY_F7: u16 = 98;
const KEY_F8: u16 = 100;
const KEY_F9: u16 = 101;
const KEY_F10: u16 = 109;
const KEY_F11: u16 = 103;
const KEY_F12: u16 = 111;
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
    native_mouse_smoke_sent: Cell<bool>,
    native_window_resize_smoke_sent: Cell<bool>,
    native_key_smoke_sent: Cell<bool>,
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
                if self.handle_app_command_key(event) {
                    return;
                }

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
            if self.write_mouse_report(event, mouse::LEFT_BUTTON, point, false) {
                self.ivars().selection.borrow_mut().clear();
                self.ivars().scrollback_offset.set(0);
                return;
            }

            self.ivars()
                .selection
                .borrow_mut()
                .begin(self.absolute_grid_point_for_event(event));
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if self.write_mouse_report(event, mouse::LEFT_DRAG, point, false) {
                return;
            }

            let event_point = self
                .as_super()
                .convertPoint_fromView(event.locationInWindow(), None);
            let delta = selection_drag_autoscroll_delta(event_point.y, self.ivars().rows.get());
            if delta != 0 {
                self.adjust_scrollback(delta);
            }

            self.ivars()
                .selection
                .borrow_mut()
                .update(self.absolute_grid_point_for_event(event));
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(mouseUp:))]
        fn mouse_up(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if self.write_mouse_report(event, mouse::LEFT_BUTTON, point, true) {
                return;
            }

            self.ivars()
                .selection
                .borrow_mut()
                .update(self.absolute_grid_point_for_event(event));
            self.as_super().setNeedsDisplay(true);
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
            let point = self.grid_point_for_event(event);
            if event.scrollingDeltaY() > 0.0
                && self.write_mouse_report(event, mouse::WHEEL_UP, point, false)
            {
                return;
            }
            if event.scrollingDeltaY() < 0.0
                && self.write_mouse_report(event, mouse::WHEEL_DOWN, point, false)
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

            let selection_range = self
                .ivars()
                .selection
                .borrow()
                .viewport_range(snapshot.viewport_start_absolute_row, snapshot.lines.len());
            draw_selection_highlights(&snapshot, selection_range);
            draw_terminal_text(&snapshot);
            draw_composition_text(&snapshot, &self.ivars().composition.borrow());
            if self.ivars().scrollback_offset.get() == 0 {
                draw_cursor(&snapshot);
            }
        }

        #[unsafe(method(redrawTimerFired:))]
        fn redraw_timer_fired(&self, _timer: &NSTimer) {
            self.apply_pending_clipboard_writes();
            self.apply_pending_title_writes();
            self.apply_native_mouse_smoke_if_requested();
            self.apply_native_window_resize_smoke_if_requested();
            self.apply_native_key_smoke_if_requested();
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
            native_mouse_smoke_sent: Cell::new(false),
            native_window_resize_smoke_sent: Cell::new(false),
            native_key_smoke_sent: Cell::new(false),
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

    fn apply_pending_title_writes(&self) {
        let title_writes = match self.ivars().buffer.lock() {
            Ok(mut buffer) => buffer.take_pending_title_writes(),
            Err(_) => {
                logging::pty_error("terminal buffer lock poisoned while draining OSC title");
                return;
            }
        };

        let Some(title) = title_writes.last() else {
            return;
        };
        let Some(window) = self.as_super().window() else {
            return;
        };

        window.setTitle(&NSString::from_str(title));
        logging::pty_info("terminal view applied OSC title update");
    }

    fn apply_native_mouse_smoke_if_requested(&self) {
        if self.ivars().native_mouse_smoke_sent.get() {
            return;
        }
        if env::var("MINIMAL_TERMINAL_SMOKE_NATIVE_MOUSE_REPORT")
            .ok()
            .as_deref()
            != Some("left-press")
        {
            return;
        }
        if !self.current_modes().mouse_reporting {
            return;
        }

        let Some(window) = self.as_super().window() else {
            return;
        };
        let location = NSPoint::new(
            PADDING_X + (terminal_cell_width() * 2.0),
            PADDING_Y + LINE_HEIGHT,
        );
        let event: Option<Retained<NSEvent>> = unsafe {
            msg_send![
                NSEvent::class(),
                mouseEventWithType: NSEventType::LeftMouseDown,
                location: location,
                modifierFlags: NSEventModifierFlags::empty(),
                timestamp: 0.0,
                windowNumber: window.windowNumber() as NSInteger,
                context: Option::<&AnyObject>::None,
                eventNumber: 0 as NSInteger,
                clickCount: 1 as NSInteger,
                pressure: 1.0f32
            ]
        };
        let Some(event) = event else {
            logging::pty_error("native mouse smoke skipped: failed to create NSEvent");
            self.ivars().native_mouse_smoke_sent.set(true);
            return;
        };

        self.ivars().native_mouse_smoke_sent.set(true);
        let _: () = unsafe { msg_send![self, mouseDown: &*event] };
    }

    fn apply_native_window_resize_smoke_if_requested(&self) {
        if self.ivars().native_window_resize_smoke_sent.get() {
            return;
        }
        let Some((rows, cols)) = env::var("MINIMAL_TERMINAL_SMOKE_NATIVE_WINDOW_RESIZE")
            .ok()
            .and_then(|value| parse_rows_by_cols(&value))
        else {
            return;
        };
        let Some(window) = self.as_super().window() else {
            return;
        };

        let content_width = (PADDING_X * 2.0) + (cols as f64 * terminal_cell_width());
        let content_height = (PADDING_Y * 2.0) + (rows as f64 * LINE_HEIGHT);
        window.setContentSize(NSSize::new(content_width, content_height));
        self.ivars().native_window_resize_smoke_sent.set(true);
        logging::pty_info(&format!(
            "native window resize smoke applied: rows={rows} cols={cols}"
        ));
    }

    fn apply_native_key_smoke_if_requested(&self) {
        if self.ivars().native_key_smoke_sent.get() {
            return;
        }
        let Some(events) = env::var("MINIMAL_TERMINAL_SMOKE_NATIVE_KEY")
            .ok()
            .and_then(|value| native_key_smoke_events(&value))
        else {
            return;
        };
        if !self.snapshot_contains_text("native-key-ready") {
            return;
        }

        let Some(window) = self.as_super().window() else {
            return;
        };
        self.ivars().native_key_smoke_sent.set(true);
        for (flags, key_code) in events {
            let empty = NSString::from_str("");
            let event: Option<Retained<NSEvent>> = unsafe {
                msg_send![
                    NSEvent::class(),
                    keyEventWithType: NSEventType::KeyDown,
                    location: NSPoint::new(0.0, 0.0),
                    modifierFlags: flags,
                    timestamp: 0.0,
                    windowNumber: window.windowNumber() as NSInteger,
                    context: Option::<&AnyObject>::None,
                    characters: &*empty,
                    charactersIgnoringModifiers: &*empty,
                    isARepeat: false,
                    keyCode: key_code
                ]
            };
            let Some(event) = event else {
                logging::pty_error("native key smoke event skipped: failed to create NSEvent");
                continue;
            };

            let _: () = unsafe { msg_send![self, keyDown: &*event] };
        }
    }

    fn snapshot_contains_text(&self, needle: &str) -> bool {
        self.ivars()
            .buffer
            .lock()
            .map(|buffer| {
                buffer
                    .combined_snapshot(0, usize::MAX)
                    .lines
                    .iter()
                    .any(|line| line.contains(needle))
            })
            .unwrap_or(false)
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
        let Some(snapshot) = self.full_snapshot() else {
            return;
        };
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
                viewport_start_absolute_row: 0,
            })
    }

    fn full_snapshot(&self) -> Option<TerminalSnapshot> {
        self.ivars()
            .buffer
            .lock()
            .ok()
            .map(|buffer| buffer.combined_snapshot(0, usize::MAX))
    }

    fn current_modes(&self) -> TerminalModes {
        self.ivars()
            .buffer
            .lock()
            .map(|buffer| buffer.snapshot(self.ivars().rows.get().max(1)).modes)
            .unwrap_or_default()
    }

    fn write_mouse_report(
        &self,
        event: &NSEvent,
        code: u16,
        point: GridPoint,
        release: bool,
    ) -> bool {
        let modes = self.current_modes();
        if !modes.mouse_reporting {
            return false;
        }

        let modifiers = mouse_modifier_mask(event.modifierFlags());
        let bytes = if modes.sgr_mouse {
            mouse::sgr_mouse_report(code, modifiers, point.row, point.col, release)
        } else {
            mouse::legacy_mouse_report(code, modifiers, point.row, point.col, release)
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
        let cell_width = terminal_cell_width();
        let col = ((point.x - PADDING_X) / cell_width).floor().max(0.0) as usize;

        GridPoint {
            row: row.min(self.ivars().rows.get().saturating_sub(1)),
            col: col.min(self.ivars().cols.get()),
        }
    }

    fn absolute_grid_point_for_event(&self, event: &NSEvent) -> GridPoint {
        let point = self.grid_point_for_event(event);
        let snapshot = self.current_snapshot(self.ivars().rows.get().max(1));
        GridPoint {
            row: snapshot.viewport_start_absolute_row + point.row,
            col: point.col,
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
                viewport_start_absolute_row: 0,
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

    fn handle_app_command_key(&self, event: &NSEvent) -> bool {
        if is_command_key(event, KEY_W) {
            if let Some(window) = self.as_super().window() {
                window.performClose(None);
            }
            return true;
        }

        if is_command_key(event, KEY_Q) {
            NSApplication::sharedApplication(self.mtm()).terminate(None);
            return true;
        }

        if is_command_key(event, KEY_N) {
            let app = NSApplication::sharedApplication(self.mtm());
            if let Some(delegate) = app.delegate() {
                unsafe {
                    let _: () = msg_send![&*delegate, newTerminalWindow: self];
                }
            }
            return true;
        }

        false
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
    is_command_key(event, 9)
}

fn is_command_c(event: &NSEvent) -> bool {
    is_command_key(event, 8)
}

fn is_command_key(event: &NSEvent, key_code: u16) -> bool {
    is_command_key_parts(event.keyCode(), event.modifierFlags(), key_code)
}

fn is_command_key_parts(
    event_key_code: u16,
    flags: NSEventModifierFlags,
    expected_key_code: u16,
) -> bool {
    event_key_code == expected_key_code
        && flags.contains(objc2_app_kit::NSEventModifierFlags::Command)
        && !flags.intersects(
            objc2_app_kit::NSEventModifierFlags::Control
                | objc2_app_kit::NSEventModifierFlags::Option,
        )
}

fn should_use_text_input(event: &NSEvent) -> bool {
    should_use_text_input_parts(event.keyCode(), event.modifierFlags())
}

fn should_use_text_input_parts(key_code: u16, flags: NSEventModifierFlags) -> bool {
    if flags.intersects(
        objc2_app_kit::NSEventModifierFlags::Command
            | objc2_app_kit::NSEventModifierFlags::Control
            | objc2_app_kit::NSEventModifierFlags::Option,
    ) {
        return false;
    }

    !matches!(
        key_code,
        KEY_RETURN
            | KEY_TAB
            | KEY_ESCAPE
            | KEY_KEYPAD_ENTER
            | KEY_F1
            | KEY_F2
            | KEY_F3
            | KEY_F4
            | KEY_F5
            | KEY_F6
            | KEY_F7
            | KEY_F8
            | KEY_F9
            | KEY_F10
            | KEY_F11
            | KEY_F12
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

fn selection_drag_autoscroll_delta(y: f64, rows: usize) -> isize {
    let bottom = PADDING_Y + (rows.max(1) as f64 * LINE_HEIGHT);
    if y < PADDING_Y {
        1
    } else if y > bottom {
        -1
    } else {
        0
    }
}

fn mouse_modifier_mask(flags: NSEventModifierFlags) -> u16 {
    let mut mask = 0;
    if flags.contains(NSEventModifierFlags::Shift) {
        mask |= mouse::SHIFT_MODIFIER;
    }
    if flags.contains(NSEventModifierFlags::Option) {
        mask |= mouse::META_MODIFIER;
    }
    if flags.contains(NSEventModifierFlags::Control) {
        mask |= mouse::CONTROL_MODIFIER;
    }
    mask
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

        let cell_width = terminal_cell_width();
        let x = PADDING_X + (start_col as f64 * cell_width);
        let y = PADDING_Y + (row as f64 * LINE_HEIGHT);
        let width = (end_col - start_col) as f64 * cell_width;
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
    draw_styled_line_backgrounds(line, y);

    let mut x = PADDING_X;
    let cell_width = terminal_cell_width();

    for span in &line.spans {
        let attributes = terminal_text_attributes(span.style);
        draw_text_cells_at(&span.text, x, y, &attributes);
        x += display_width(&span.text) as f64 * cell_width;
    }
}

fn draw_styled_line_backgrounds(line: &StyledLine, y: f64) {
    let mut x = PADDING_X;
    let cell_width = terminal_cell_width();

    for span in &line.spans {
        let width = display_width(&span.text);
        if width > 0 {
            if let Some(background) = terminal_background_color(span.style) {
                background.setFill();
                NSRectFill(NSRect::new(
                    NSPoint::new(x, y),
                    objc2_foundation::NSSize::new(width as f64 * cell_width, LINE_HEIGHT),
                ));
            }
        }
        x += width as f64 * cell_width;
    }
}

fn draw_text_cells_at(text: &str, x: f64, y: f64, attributes: &NSMutableDictionary) {
    let mut current_x = x;
    let mut previous_x = x;
    let cell_width = terminal_cell_width();

    for ch in text.chars() {
        let width = char_width(ch);
        let draw_x = if width == 0 { previous_x } else { current_x };
        draw_text_at(&ch.to_string(), draw_x, y, attributes);

        if width > 0 {
            previous_x = current_x;
            current_x += width as f64 * cell_width;
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
    let cell_width = terminal_cell_width();
    let x = PADDING_X + (snapshot.cursor.col as f64 * cell_width);
    let y = PADDING_Y + (snapshot.cursor.row as f64 * LINE_HEIGHT) + 2.0;
    let (x, y, width, height) = match snapshot.modes.cursor_style {
        CursorStyle::Block => (x, y, cell_width, CURSOR_HEIGHT),
        CursorStyle::Bar => (x, y, 2.0, CURSOR_HEIGHT),
        CursorStyle::Underline => (x, y + CURSOR_HEIGHT - 2.0, cell_width, 2.0),
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

fn terminal_cell_width() -> f64 {
    let regular_width = terminal_font(false)
        .and_then(|font| measured_cell_width_for_font(&font))
        .unwrap_or(FALLBACK_CELL_WIDTH);
    let bold_width = terminal_font(true)
        .and_then(|font| measured_cell_width_for_font(&font))
        .unwrap_or(regular_width);

    regular_width
        .max(bold_width)
        .ceil()
        .max(FALLBACK_CELL_WIDTH)
}

fn measured_cell_width_for_font(font: &NSFont) -> Option<f64> {
    let attributes: Retained<NSMutableDictionary<NSAttributedStringKey, AnyObject>> =
        NSMutableDictionary::new();
    let font_object: &AnyObject = font;
    let _: () = unsafe {
        msg_send![
            &*attributes,
            setObject: font_object,
            forKey: &*NSFontAttributeName
        ]
    };

    let string = NSString::from_str("M");
    let size: objc2_foundation::NSSize = unsafe {
        msg_send![
            &*string,
            sizeWithAttributes: Some(&*attributes)
        ]
    };
    if size.width.is_finite() && size.width > 0.0 {
        Some(size.width)
    } else {
        None
    }
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
    let cell_width = terminal_cell_width();
    let available_width = (bounds.size.width - (PADDING_X * 2.0)).max(cell_width);
    let rows = (available_height / LINE_HEIGHT).floor().max(1.0) as usize;
    let cols = (available_width / cell_width).floor().max(1.0) as usize;
    (rows, cols)
}

fn parse_rows_by_cols(value: &str) -> Option<(usize, usize)> {
    let (rows, cols) = value.split_once('x')?;
    let rows = rows.parse().ok()?;
    let cols = cols.parse().ok()?;
    if rows == 0 || cols == 0 {
        return None;
    }
    Some((rows, cols))
}

fn native_key_smoke_event(value: &str) -> Option<(NSEventModifierFlags, u16)> {
    match value {
        "control-f5" => Some((NSEventModifierFlags::Control, 96)),
        "shift-f5" => Some((NSEventModifierFlags::Shift, 96)),
        "option-f5" => Some((NSEventModifierFlags::Option, 96)),
        "shift-option-f5" => Some((
            NSEventModifierFlags::Shift | NSEventModifierFlags::Option,
            96,
        )),
        "shift-control-f5" => Some((
            NSEventModifierFlags::Shift | NSEventModifierFlags::Control,
            96,
        )),
        "control-option-f5" => Some((
            NSEventModifierFlags::Control | NSEventModifierFlags::Option,
            96,
        )),
        "shift-control-option-f5" => Some((
            NSEventModifierFlags::Shift
                | NSEventModifierFlags::Control
                | NSEventModifierFlags::Option,
            96,
        )),
        "shift-up" => Some((NSEventModifierFlags::Shift, 126)),
        "option-up" => Some((NSEventModifierFlags::Option, 126)),
        "shift-option-up" => Some((
            NSEventModifierFlags::Shift | NSEventModifierFlags::Option,
            126,
        )),
        "control-up" => Some((NSEventModifierFlags::Control, 126)),
        "shift-control-up" => Some((
            NSEventModifierFlags::Shift | NSEventModifierFlags::Control,
            126,
        )),
        "control-option-up" => Some((
            NSEventModifierFlags::Control | NSEventModifierFlags::Option,
            126,
        )),
        "shift-control-option-up" => Some((
            NSEventModifierFlags::Shift
                | NSEventModifierFlags::Control
                | NSEventModifierFlags::Option,
            126,
        )),
        "control-option-right" => Some((
            NSEventModifierFlags::Control | NSEventModifierFlags::Option,
            124,
        )),
        _ => None,
    }
}

fn native_key_smoke_events(value: &str) -> Option<Vec<(NSEventModifierFlags, u16)>> {
    let events = value
        .split(',')
        .map(str::trim)
        .map(native_key_smoke_event)
        .collect::<Option<Vec<_>>>()?;
    (!events.is_empty()).then_some(events)
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

#[cfg(test)]
mod tests {
    use objc2_app_kit::NSEventModifierFlags;

    #[test]
    fn selection_drag_autoscrolls_outside_vertical_bounds() {
        assert_eq!(super::selection_drag_autoscroll_delta(0.0, 10), 1);
        assert_eq!(super::selection_drag_autoscroll_delta(20.0, 10), 0);
        assert_eq!(super::selection_drag_autoscroll_delta(220.0, 10), -1);
    }

    #[test]
    fn mouse_modifier_mask_tracks_xterm_bits() {
        assert_eq!(
            super::mouse_modifier_mask(NSEventModifierFlags::Shift | NSEventModifierFlags::Control),
            20
        );
        assert_eq!(super::mouse_modifier_mask(NSEventModifierFlags::Option), 8);
    }

    #[test]
    fn app_command_shortcuts_require_command_without_control_or_option() {
        assert!(super::is_command_key_parts(
            super::KEY_N,
            NSEventModifierFlags::Command,
            super::KEY_N
        ));
        assert!(super::is_command_key_parts(
            super::KEY_W,
            NSEventModifierFlags::Command,
            super::KEY_W
        ));
        assert!(super::is_command_key_parts(
            super::KEY_Q,
            NSEventModifierFlags::Command,
            super::KEY_Q
        ));
        assert!(!super::is_command_key_parts(
            super::KEY_N,
            NSEventModifierFlags::empty(),
            super::KEY_N
        ));
        assert!(!super::is_command_key_parts(
            super::KEY_N,
            NSEventModifierFlags::Command | NSEventModifierFlags::Option,
            super::KEY_N
        ));
        assert!(!super::is_command_key_parts(
            super::KEY_N,
            NSEventModifierFlags::Command | NSEventModifierFlags::Control,
            super::KEY_N
        ));
    }

    #[test]
    fn text_input_path_excludes_special_keys_even_with_shift_only() {
        assert!(super::should_use_text_input_parts(
            0,
            NSEventModifierFlags::Shift
        ));
        assert!(!super::should_use_text_input_parts(
            super::KEY_F5,
            NSEventModifierFlags::Shift
        ));
        assert!(!super::should_use_text_input_parts(
            126,
            NSEventModifierFlags::Shift
        ));
    }

    #[test]
    fn parse_rows_by_cols_accepts_valid_terminal_size() {
        assert_eq!(super::parse_rows_by_cols("24x80"), Some((24, 80)));
    }

    #[test]
    fn parse_rows_by_cols_rejects_invalid_terminal_size() {
        assert_eq!(super::parse_rows_by_cols("24:80"), None);
        assert_eq!(super::parse_rows_by_cols("0x80"), None);
        assert_eq!(super::parse_rows_by_cols("24x0"), None);
    }

    #[test]
    fn native_key_smoke_event_maps_known_cases() {
        assert_eq!(
            super::native_key_smoke_event("control-f5"),
            Some((NSEventModifierFlags::Control, 96))
        );
        assert_eq!(
            super::native_key_smoke_event("shift-control-option-f5"),
            Some((
                NSEventModifierFlags::Shift
                    | NSEventModifierFlags::Control
                    | NSEventModifierFlags::Option,
                96
            ))
        );
        assert_eq!(
            super::native_key_smoke_event("shift-option-up"),
            Some((
                NSEventModifierFlags::Shift | NSEventModifierFlags::Option,
                126
            ))
        );
        assert_eq!(
            super::native_key_smoke_event("control-option-right"),
            Some((
                NSEventModifierFlags::Control | NSEventModifierFlags::Option,
                124
            ))
        );
        assert_eq!(super::native_key_smoke_event("unknown"), None);
    }

    #[test]
    fn native_key_smoke_events_maps_comma_separated_cases() {
        assert_eq!(
            super::native_key_smoke_events("shift-up, control-up"),
            Some(vec![
                (NSEventModifierFlags::Shift, 126),
                (NSEventModifierFlags::Control, 126)
            ])
        );
        assert_eq!(super::native_key_smoke_events("shift-up,unknown"), None);
        assert_eq!(super::native_key_smoke_events(""), None);
    }

    #[test]
    fn terminal_cell_width_covers_regular_and_bold_fonts() {
        let cell_width = super::terminal_cell_width();
        assert!(cell_width >= super::FALLBACK_CELL_WIDTH);
        assert_eq!(cell_width.fract(), 0.0);

        if let Some(font) = super::terminal_font(false) {
            let measured = super::measured_cell_width_for_font(&font).unwrap();
            assert!(cell_width >= measured);
        }
        if let Some(font) = super::terminal_font(true) {
            let measured = super::measured_cell_width_for_font(&font).unwrap();
            assert!(cell_width >= measured);
        }
    }
}
