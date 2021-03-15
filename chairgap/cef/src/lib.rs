use chairgap_cef_binding::{
    dgcef_browser_free, dgcef_browser_new, dgcef_init, dgcef_run_message_loop, dummy_fn,
};
use chairgap_common::engine::Engine;
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int};
use std::path::Path;

fn leak_main_args(args: impl IntoIterator<Item = impl AsRef<str>>) -> (c_int, *mut *mut c_char) {
    let c_args = args
        .into_iter()
        .map(|arg| CString::new(arg.as_ref()).unwrap());
    let c_arg_ptrs = c_args
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();

    (c_arg_ptrs.len() as _, c_arg_ptrs.leak().as_mut_ptr())
}

#[derive(Debug)]
pub struct CefEngine;

impl CefEngine {
    pub unsafe fn load(cef_path: &Path) -> Option<Self> {
        dummy_fn();
        let init_ret = if cfg!(target_os = "windows") {
            unimplemented!()
        } else {
            use std::os::unix::ffi::OsStrExt;
            let cef_path = CString::new(cef_path.as_os_str().as_bytes()).unwrap();
            let (argc, argv) = leak_main_args(std::env::args());
            dgcef_init(cef_path.as_ptr() as *const c_void, argc, argv)
        };
        if init_ret == 1 {
            Some(CefEngine)
        } else {
            None
        }
    }
}

impl Engine for CefEngine {
    unsafe fn run_event_loop(&self) -> bool {
        dgcef_run_message_loop();
        true
    }

    unsafe fn populate_web_view(&self, parent_handle: *mut c_void) {
        dgcef_browser_new(parent_handle);
    }
}
