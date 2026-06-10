#![deny(unsafe_op_in_unsafe_fn)]

use std::cell::OnceCell;
use std::sync::{Arc, Mutex};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, ClassType, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSSize,
};

mod composition;
mod input;
mod logging;
mod mouse;
mod paste;
mod pty;
mod selection;
mod terminal_buffer;
mod terminal_view;

use terminal_buffer::TerminalBuffer;
use terminal_view::TerminalView;

#[derive(Default)]
struct AppDelegateIvars {
    window: OnceCell<Retained<NSWindow>>,
    terminal_view: OnceCell<Retained<TerminalView>>,
}

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppDelegateIvars]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn application_did_finish_launching(&self, notification: &NSNotification) {
            logging::app_info("applicationDidFinishLaunching started");

            let mtm = self.mtm();
            let app = notification
                .object()
                .expect("application launch notification must have an object")
                .downcast::<NSApplication>()
                .expect("launch notification object must be NSApplication");

            let window = create_main_window(mtm);
            let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(900.0, 560.0));
            let buffer = Arc::new(Mutex::new(TerminalBuffer::new(2_000)));

            let writer = match pty::spawn_login_shell(Arc::clone(&buffer)) {
                Ok(writer) => writer,
                Err(error) => {
                    logging::app_error(&format!("failed to spawn login shell: {error}"));
                    if let Ok(mut buffer) = buffer.lock() {
                        buffer.append_bytes(
                            format!("failed to spawn login shell: {error}\n").as_bytes(),
                        );
                    }
                    return;
                }
            };

            let terminal_view = TerminalView::new(mtm, frame, buffer, writer);
            window.setContentView(Some(terminal_view.as_super()));
            window.center();
            window.setDelegate(Some(ProtocolObject::from_ref(self)));
            window.makeKeyAndOrderFront(None);
            terminal_view.focus(&window);

            self.ivars()
                .window
                .set(window)
                .expect("main window should only be initialized once");
            self.ivars().terminal_view.set(terminal_view).ok();

            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);

            logging::app_info("applicationDidFinishLaunching completed");
        }

        #[unsafe(method(applicationWillTerminate:))]
        fn application_will_terminate(&self, _notification: &NSNotification) {
            logging::app_info("applicationWillTerminate");
        }
    }

    unsafe impl NSWindowDelegate for AppDelegate {
        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
            logging::app_info("windowWillClose");
            NSApplication::sharedApplication(self.mtm()).terminate(None);
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let delegate = Self::alloc(mtm).set_ivars(AppDelegateIvars::default());
        unsafe { msg_send![super(delegate), init] }
    }
}

fn main() {
    logging::app_info("main started");
    let mtm = MainThreadMarker::new().expect("main thread marker must be available");
    let app = NSApplication::sharedApplication(mtm);
    let delegate = AppDelegate::new(mtm);

    app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    logging::app_info("application run loop starting");
    app.run();
}

fn create_main_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
    logging::app_info("create_main_window started");
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(900.0, 560.0));
    let style = NSWindowStyleMask::Titled
        | NSWindowStyleMask::Closable
        | NSWindowStyleMask::Miniaturizable
        | NSWindowStyleMask::Resizable;

    let window = unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            frame,
            style,
            NSBackingStoreType::Buffered,
            false,
        )
    };

    unsafe { window.setReleasedWhenClosed(false) };
    window.setTitle(ns_string!("Minimal Terminal"));
    logging::app_info("create_main_window completed");
    window
}
