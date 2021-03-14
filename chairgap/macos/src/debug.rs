use super::strings::ns_as_str;
use objc::rc::{autoreleasepool, StrongPtr};
use objc::runtime::Object;
use objc::*;
use std::fmt::{Debug, Formatter, Result};
use std::ops::Deref;

#[derive(Clone)]
pub(super) struct DebugStrongPtr(StrongPtr);

impl From<StrongPtr> for DebugStrongPtr {
    fn from(strong_ptr: StrongPtr) -> Self {
        DebugStrongPtr { 0: strong_ptr }
    }
}

impl Debug for DebugStrongPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        autoreleasepool(|| {
            let debug_string = unsafe { StrongPtr::retain(msg_send![*self.0, debugDescription]) };
            let debug_str = unsafe { ns_as_str(*debug_string) };
            f.write_str(debug_str)
        })
    }
}
impl Deref for DebugStrongPtr {
    type Target = *mut Object;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
