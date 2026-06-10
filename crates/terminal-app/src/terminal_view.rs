use std::cell::OnceCell;
use std::sync::{Arc, Mutex};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSColor, NSEvent, NSRectFill, NSResponder, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSObjectProtocol, NSPoint, NSRect, NSString, NSTimer,
};

use crate::logging;
use crate::pty::PtyWriter;
use crate::terminal_buffer::TerminalBuffer;

const PADDING_X: f64 = 12.0;
const PADDING_Y: f64 = 14.0;

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
    NSColor::whiteColor().set();
    let string = NSString::from_str(text);
    let _: () = unsafe {
        msg_send![
            &*string,
            drawAtPoint: NSPoint::new(PADDING_X, PADDING_Y),
            withAttributes: Option::<&AnyObject>::None
        ]
    };
}
