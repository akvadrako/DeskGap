use block::{Block, ConcreteBlock};
use chairgap_common::app::App;
use chairgap_macos_src::*;
use cocoa::appkit::{NSApp, NSApplication, NSApplicationTerminateReply};
use objc::rc::StrongPtr;
use objc::runtime::{BOOL, NO};
use objc::*;
use std::cell::RefCell;
use std::os::raw::c_void;

pub fn init<C: Clone + 'static, AppType: App<C>>(ctx: C) -> impl FnOnce() {
    objc::rc::autoreleasepool(|| {
        let app: RefCell<Option<AppType>> = RefCell::new(None);
        let block =
            ConcreteBlock::new(
                move |evt_type: DGAppEventType, evt_data: *mut c_void| match evt_type {
                    DGAppEventType::WillLaunch => {
                        let mut existing_app = app.borrow_mut();
                        if existing_app.is_some() {
                            panic!("DGAppEventType_WillLaunch emitted more than once")
                        }
                        let _ = existing_app.replace(AppType::new(ctx.clone()));
                    }
                    DGAppEventType::ShouldClose => {
                        let evt_data = unsafe {
                            evt_data
                                .cast::<NSApplicationTerminateReply>()
                                .as_mut()
                                .unwrap()
                        };
                        *evt_data = NSApplicationTerminateReply::NSTerminateCancel
                    }
                    DGAppEventType::Reopen => {
                        let app_borrow = app.borrow();
                        app_borrow.as_ref().unwrap().activate()
                    }
                    _ => panic!("Illegal DGWindowEventType value: {:?}", evt_type),
                },
            )
            .copy();
        let block: &Block<(DGAppEventType, *mut c_void), ()> = &block;

        let delegate = unsafe { StrongPtr::retain(DGAppDelegateNew(block)) };
        let _: () = unsafe { msg_send![NSApp(), setDelegate: *delegate] };

        move || {
            let _delegate = delegate; // Keep delegate alive since it's weakly referenced by NSApp
            unsafe { NSApplication::run(NSApp()) };
        }
    })
}

pub fn is_main_thread() -> bool {
    let is_main: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
    is_main != NO
}
//
// pub(crate) fn run_nested_loop() {
//     check_main_thread!();
//     unsafe { core_foundation::runloop::CFRunLoopRun() };
// }
//
// pub(crate) fn quit_nested_loop() {
//     check_main_thread!();
//     use core_foundation::runloop;
//     unsafe { runloop::CFRunLoopStop(runloop::CFRunLoopGetMain()) };
// }

pub fn tick() {
    unsafe { DGAppTick() }
}

pub fn set_menu() {}
