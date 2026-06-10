use std::cell::{Cell, OnceCell};
use std::sync::{Arc, Mutex};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSColor, NSEvent, NSFont, NSFontAttributeName, NSForegroundColorAttributeName, NSPasteboard,
    NSPasteboardTypeString, NSRectFill, NSResponder, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSMutableDictionary, NSObjectProtocol, NSPoint, NSRect, NSString, NSTimer,
};
use terminal_core::TerminalSnapshot;

use crate::input;
use crate::logging;
use crate::pty::PtyWriter;
use crate::terminal_buffer::TerminalBuffer;

const PADDING_X: f64 = 12.0;
const PADDING_Y: f64 = 14.0;
const FONT_SIZE: f64 = 14.0;
const LINE_HEIGHT: f64 = 18.0;
const CELL_WIDTH: f64 = 8.4;
const CURSOR_WIDTH: f64 = 8.0;
const CURSOR_HEIGHT: f64 = 16.0;
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
}

define_class!(
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = TerminalViewIvars]
    pub(crate) struct TerminalView;

    unsafe impl NSObjectProtocol for TerminalView {}

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
                if is_command_v(event) {
                    self.paste_text_from_clipboard(pool);
                    return;
                }

                if self.handle_scrollback_key(event) {
                    return;
                }

                let Some(characters) = event.characters() else {
                    return;
                };
                let input = unsafe { characters.to_str(pool) };
                let Some(bytes) = input::encode_key_event(event, input) else {
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

                self.ivars().scrollback_offset.set(0);
            });
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
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

            let snapshot = self
                .ivars()
                .buffer
                .lock()
                .map(|buffer| {
                    let offset = self.ivars().scrollback_offset.get();
                    if offset == 0 {
                        buffer.snapshot(rows)
                    } else {
                        buffer.scrollback_snapshot(offset.saturating_sub(rows), rows)
                    }
                })
                .unwrap_or_else(|_| TerminalSnapshot {
                    lines: vec!["terminal buffer unavailable".to_string()],
                    cursor: terminal_core::Cursor::default(),
                    scrollback_len: 0,
                });

            draw_terminal_text(&snapshot);
            if self.ivars().scrollback_offset.get() == 0 {
                draw_cursor(&snapshot);
            }
        }

        #[unsafe(method(redrawTimerFired:))]
        fn redraw_timer_fired(&self, _timer: &NSTimer) {
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

        if let Err(error) = self.ivars().writer.write_all(input.as_bytes()) {
            logging::pty_error(&format!("pty write failed from paste: {error}"));
        }

        self.ivars().scrollback_offset.set(0);
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

fn draw_background(rect: NSRect) {
    NSColor::blackColor().setFill();
    NSRectFill(rect);
}

fn draw_terminal_text(snapshot: &TerminalSnapshot) {
    let attributes = terminal_text_attributes();

    for (index, line) in snapshot.lines.iter().enumerate() {
        let string = NSString::from_str(line);
        let y = PADDING_Y + (index as f64 * LINE_HEIGHT);
        let _: () = unsafe {
            msg_send![
                &*string,
                drawAtPoint: NSPoint::new(PADDING_X, y),
                withAttributes: Some(&*attributes)
            ]
        };
    }
}

fn draw_cursor(snapshot: &TerminalSnapshot) {
    let x = PADDING_X + (snapshot.cursor.col as f64 * CELL_WIDTH);
    let y = PADDING_Y + (snapshot.cursor.row as f64 * LINE_HEIGHT) + 2.0;
    NSColor::whiteColor().setFill();
    NSRectFill(NSRect::new(
        NSPoint::new(x, y),
        objc2_foundation::NSSize::new(CURSOR_WIDTH, CURSOR_HEIGHT),
    ));
}

fn terminal_text_attributes() -> Retained<NSMutableDictionary> {
    let attributes = NSMutableDictionary::new();
    let color = NSColor::whiteColor();
    let color_object: &AnyObject = color.as_ref();

    let _: () = unsafe {
        msg_send![
            &*attributes,
            setObject: color_object,
            forKey: &*NSForegroundColorAttributeName
        ]
    };

    if let Some(font) = terminal_font() {
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

fn terminal_font() -> Option<Retained<NSFont>> {
    for font_name in TERMINAL_FONT_NAMES {
        if let Some(font) = NSFont::fontWithName_size(&NSString::from_str(font_name), FONT_SIZE) {
            return Some(font);
        }
    }

    NSFont::userFixedPitchFontOfSize(FONT_SIZE)
}

fn terminal_dimensions(bounds: NSRect) -> (usize, usize) {
    let available_height = (bounds.size.height - (PADDING_Y * 2.0)).max(LINE_HEIGHT);
    let available_width = (bounds.size.width - (PADDING_X * 2.0)).max(CELL_WIDTH);
    let rows = (available_height / LINE_HEIGHT).floor().max(1.0) as usize;
    let cols = (available_width / CELL_WIDTH).floor().max(1.0) as usize;
    (rows, cols)
}
