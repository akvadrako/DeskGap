use std::ffi::c_void;
use std::fmt::Debug;

pub trait Engine: Debug + 'static {
    unsafe fn run_event_loop(&self) -> bool;
    unsafe fn populate_web_view(&self, parent_handle: *mut c_void);
}
