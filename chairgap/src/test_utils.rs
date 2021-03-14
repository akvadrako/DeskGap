use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub(crate) struct DropItem {
    is_dropped: Arc<Mutex<bool>>,
}
impl Drop for DropItem {
    fn drop(&mut self) {
        let mut lock = self.is_dropped.lock().unwrap();
        *lock = true
    }
}

pub(crate) struct DropDetection {
    is_dropped: Arc<Mutex<bool>>,
}
impl DropDetection {
    pub(crate) fn dropped(&self) -> bool {
        *self.is_dropped.lock().unwrap()
    }
}

pub(crate) fn detect_drop() -> (DropDetection, DropItem) {
    let is_dropped = Arc::new(Mutex::new(false));
    (
        DropDetection {
            is_dropped: is_dropped.clone(),
        },
        DropItem { is_dropped },
    )
}

pub(crate) fn autoreleasepool_if_mac<T, F: FnOnce() -> T>(f: F) -> T {
    if cfg!(target_os = "macos") {
        objc::rc::autoreleasepool(f)
    } else {
        f()
    }
}

pub(crate) struct MainLoop {
    should_quit: RefCell<bool>,
}
impl MainLoop {
    pub fn new() -> Self {
        Self {
            should_quit: RefCell::new(false),
        }
    }
    pub fn run(&self) {
        use crate::platform::app::tick;
        while !*self.should_quit.borrow() {
            tick()
        }
    }
    pub fn quit(&self) {
        *self.should_quit.borrow_mut() = true
    }
}
