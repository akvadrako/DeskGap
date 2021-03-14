#[cfg(test)]
mod tests {
    use super::super::super::window::*;
    use crate::test_utils::{autoreleasepool_if_mac, detect_drop};

    #[test]
    fn test_window_dropping_event_handler() {
        let (detection, item) = detect_drop();
        autoreleasepool_if_mac(|| {
            let item = item;
            let win = Window::new(move |_evt| {
                let _item = &item;
            });

            win.set_resizable(true);
            win.set_title("A Test Window");
            let _ = win.title();
        });
        assert!(detection.dropped());
    }
}
