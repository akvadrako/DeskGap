use chairgap_cef_sys::bindings::{cef_string_t, char16};
use widestring::U16CString;

pub(crate) fn new_cef_string(s: &str) -> cef_string_t {
    let u16_string = U16CString::from_str(s).unwrap();
    unsafe extern "C" fn free_u16_str(str_: *mut char16) {
        U16CString::from_raw(str_);
    }
    cef_string_t {
        length: u16_string.len() as u64,
        str_: u16_string.into_raw(),
        dtor: Some(free_u16_str),
    }
}

// fn string_from_cef(cef_str: &cef_string_t) {}
