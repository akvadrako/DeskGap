use super::super::common::webview::Event;
use super::debug::DebugStrongPtr;
use super::strings::ns_string;
use super::window::Window;
use crate::platform::appkit::strings::{ns_as_str, ns_to_string};
use block::{Block, ConcreteBlock};
use chairgap_sys::*;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSData, NSInteger, NSUInteger, NSURL};
use objc::rc::{autoreleasepool, StrongPtr};
use objc::runtime::{Object, BOOL, NO, YES};
use objc::*;
use once_cell;
use once_cell::sync::Lazy;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_void;

struct SyncNSString(StrongPtr);
unsafe impl Send for SyncNSString {}
unsafe impl Sync for SyncNSString {}
impl SyncNSString {
    fn id(&self) -> id {
        *self.0
    }
}

static PRELOAD_SCRIPT_NS_STRING: Lazy<SyncNSString> = Lazy::new(|| {
    static PRELOAD_STR: &'static str = include_str!("preload.js");
    SyncNSString(ns_string(PRELOAD_STR))
});

// TODO: Make Request Send-able, Add req headers, resp body streaming
#[derive(Debug)]
pub(crate) struct Request<'a> {
    url_scheme_task: DebugStrongPtr,
    phantom_data: PhantomData<&'a Object>,
    finished: bool,
}
impl Request<'_> {
    fn new(url_scheme_task: StrongPtr) -> Self {
        Self {
            url_scheme_task: url_scheme_task.into(),
            phantom_data: PhantomData,
            finished: false,
        }
    }
    pub(crate) fn url(&self) -> String {
        autoreleasepool(|| {
            unsafe {
                let request = StrongPtr::retain(msg_send![*self.url_scheme_task, request]);

                // TODO: check if WKURLSchemeTask.request.URL is guaranteed non-null.
                let url = StrongPtr::retain(msg_send![*request, URL]);

                let url_string = StrongPtr::retain(msg_send![*url, absoluteString]);
                ns_to_string(*url_string)
            }
        })
    }
    pub(crate) fn respond(mut self, content_type: &str, body: &[u8]) {
        self.finished = true;
        autoreleasepool(|| unsafe {
            let content_type = ns_string(content_type);
            let body = StrongPtr::retain(NSData::dataWithBytes_length_(
                nil,
                body.as_ptr() as *const c_void,
                body.len() as NSUInteger,
            ));
            chairgap_sys::DGWKURLSchemeTaskSetResponse(*self.url_scheme_task, *content_type, *body);
        })
    }
}

impl Drop for Request<'_> {
    fn drop(&mut self) {
        if !self.finished {
            let domain = ns_string("deskgap.appkit.webview.request");
            let err = unsafe {
                StrongPtr::retain(msg_send![class!(NSError),
                    errorWithDomain: *domain
                    code: -1 as NSInteger
                    userInfo: nil
                ])
            };
            let _: () = unsafe { msg_send![*self.url_scheme_task, didFailWithError: *err] };
        }
    }
}

#[derive(Debug)]
pub(crate) struct WebView(DebugStrongPtr);

impl WebView {
    pub fn new<'a>(
        window: &Window,
        handle_event: impl Fn(Event<Request<'_>>) + 'static,
        custom_schemes: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        check_main_thread!();
        let web_view_container_ptr: id = unsafe { msg_send![*window.0, webViewContainer] };
        let event_callback =
            ConcreteBlock::new(move |evt_type: DGWebViewEventType, evt_data: id| {
                handle_event(match evt_type {
                    DGWebViewEventType_StartURLSchemeTask => Event::CustomSchemeRequest(unsafe {
                        Request::new(StrongPtr::retain(evt_data))
                    }),
                    DGWebViewEventType_EndURLSchemeTask => return,
                    DGWebViewEventType_TitleChanged => {
                        Event::DocumentTitleChanged(unsafe { ns_to_string(evt_data) })
                    }
                    DGWebViewEventType_MessageHandler => {
                        let message_name = unsafe { ns_to_string(msg_send![evt_data, name]) };
                        let message_body = unsafe { StrongPtr::retain(msg_send![evt_data, body]) };
                        let is_string: BOOL =
                            unsafe { msg_send![*message_body, isKindOfClass: class!(NSString)] };
                        if is_string == NO {
                            panic!("WKScriptMessage.body is not an NSString")
                        }
                        Event::ScriptMessage(unsafe { ns_to_string(*message_body) })
                    }
                    _ => panic!("Illegal DGWindowEventType value: {}", evt_type),
                });
            })
            .copy();
        let event_callback: &Block<(DGWindowEventType, id), ()> = &event_callback;

        let schemes = unsafe { StrongPtr::new(msg_send![class!(NSMutableArray), new]) };
        for scheme in custom_schemes {
            let scheme = ns_string(scheme);
            let _: () = unsafe { msg_send![*schemes, addObject: *scheme] };
        }
        let message_handler_names = unsafe {
            StrongPtr::retain(NSArray::arrayWithObjects(
                nil,
                &[*ns_string("stringMessage"), *ns_string("windowDrag")],
            ))
        };

        let inner = unsafe {
            StrongPtr::retain(DGWebViewControllerNew(
                web_view_container_ptr,
                event_callback,
                *schemes,
                *message_handler_names,
            ))
        };

        let error_message_ptr: id = unsafe { msg_send![*inner, errorMessage] };
        if error_message_ptr != nil {
            let error_message = unsafe { ns_to_string(error_message_ptr) };
            panic!("{}", error_message);
        }

        unsafe { msg_send![*inner, addPreloadScript: PRELOAD_SCRIPT_NS_STRING.id()] }

        WebView { 0: inner.into() }
    }

    // TODO: load request with custom methods and/or headers
    pub fn load_url<'a>(&self, url: &str) -> bool {
        let url_string = ns_string(url);
        let url = unsafe { NSURL::initWithString_(NSURL::alloc(nil), *url_string) };
        if url == nil {
            return false;
        }
        let url = unsafe { StrongPtr::new(url) };
        let _: () = unsafe { msg_send![*self.0, loadURL:*url] };
        true
    }

    pub fn set_devtools_enabled(&self, enabled: bool) {
        unsafe { msg_send![*self.0, setDevToolsEnabled: if enabled { YES } else { NO }] }
    }

    // TODO: add callback with evaluated value or error
    pub fn execute_script(&self, script: &str) {
        unsafe { msg_send![*self.0, executeScript: *ns_string(script)] }
    }
}
