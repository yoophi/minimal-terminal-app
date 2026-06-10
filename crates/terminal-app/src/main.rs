#![deny(unsafe_op_in_unsafe_fn)]

use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSSize,
};

#[derive(Debug, Default)]
struct AppDelegateIvars {
    window: OnceCell<Retained<NSWindow>>,
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
            let mtm = self.mtm();
            let app = notification
                .object()
                .expect("application launch notification must have an object")
                .downcast::<NSApplication>()
                .expect("launch notification object must be NSApplication");

            let window = create_main_window(mtm);
            window.center();
            window.setDelegate(Some(ProtocolObject::from_ref(self)));
            window.makeKeyAndOrderFront(None);

            self.ivars()
                .window
                .set(window)
                .expect("main window should only be initialized once");

            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);
        }
    }

    unsafe impl NSWindowDelegate for AppDelegate {
        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
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
    let mtm = MainThreadMarker::new().expect("main thread marker must be available");
    let app = NSApplication::sharedApplication(mtm);
    let delegate = AppDelegate::new(mtm);

    app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    app.run();
}

fn create_main_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
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
    window
}
