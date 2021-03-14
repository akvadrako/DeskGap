use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::rc::StrongPtr;
use objc::*;

// pub(super) trait NSStringExt {
//     unsafe fn to_str(&self) -> &str;
//     unsafe fn to_string(&self) -> String {
//         self.to_str().to_string()
//     }
// }

pub(super) fn ns_string(s: &str) -> StrongPtr {
    unsafe {
        let ns_str = NSString::alloc(nil);
        StrongPtr::new(if s.is_empty() {
            msg_send![ns_str, init]
        } else {
            NSString::init_str(ns_str, s)
        })
    }
}

pub(super) unsafe fn ns_as_str<'a>(obj: id) -> &'a str {
    let len = NSString::len(obj);
    if len == 0 {
        ""
    } else {
        let ptr = NSString::UTF8String(obj) as *const u8;
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
    }
}

pub(super) unsafe fn ns_to_string(obj: id) -> String {
    ns_as_str(obj).to_string()
}
