use std::cell::OnceCell;
use std::sync::{Arc, Mutex};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSColor, NSEvent, NSFont, NSFontAttributeName, NSForegroundColorAttributeName, NSRectFill,
    NSResponder, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSMutableDictionary, NSObjectProtocol, NSPoint, NSRect, NSString, NSTimer,
};

use crate::logging;
use crate::pty::PtyWriter;
use crate::terminal_buffer::TerminalBuffer;

const PADDING_X: f64 = 12.0;
const PADDING_Y: f64 = 14.0;
const FONT_SIZE: f64 = 14.0;
const TERMINAL_FONT_NAMES: &[&str] = &[
    "JetBrainsMonoNFM-Regular",
    "JetBrainsMonoNF-Regular",
    "JetBrainsMono-Regular",
];

pub(crate) struct TerminalViewIvars {
    buffer: Arc<Mutex<TerminalBuffer>>,
    writer: PtyWriter,
    timer: OnceCell<Retained<NSTimer>>,
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
                let Some(characters) = event.characters() else {
                    return;
                };
                let input = unsafe { characters.to_str(pool) };
                if input.is_empty() {
                    return;
                }

                if let Err(error) = self.ivars().writer.write_all(input.as_bytes()) {
                    logging::pty_error(&format!("pty write failed from keyDown: {error}"));
                }
            });
        }

        #[unsafe(method(drawRect:))]
        fn draw_rect(&self, dirty_rect: NSRect) {
            draw_background(dirty_rect);

            let text = self
                .ivars()
                .buffer
                .lock()
                .map(|buffer| buffer.visible_text(200))
                .unwrap_or_else(|_| "terminal buffer unavailable".to_string());

            draw_terminal_text(&text);
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
}

fn draw_background(rect: NSRect) {
    NSColor::blackColor().setFill();
    NSRectFill(rect);
}

fn draw_terminal_text(text: &str) {
    let attributes = terminal_text_attributes();
    let string = NSString::from_str(text);
    let _: () = unsafe {
        msg_send![
            &*string,
            drawAtPoint: NSPoint::new(PADDING_X, PADDING_Y),
            withAttributes: Some(&*attributes)
        ]
    };
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
