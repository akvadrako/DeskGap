#[derive(Debug)]
pub(crate) enum Event<Request> {
    DocumentTitleChanged(String),
    CustomSchemeRequest(Request),
    ScriptMessage(String),
}

#[cfg(test)]
mod tests {
    use super::super::super::common;
    use super::super::super::webview;
    use super::super::super::window;
    use crate::geo::Size;
    use crate::test_utils::{autoreleasepool_if_mac, detect_drop, MainLoop};
    use crate::window::Location;
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    // This is function is for debug
    #[allow(dead_code)]
    fn display_window(win: &window::Window) {
        win.set_size(Size {
            width: 800,
            height: 600,
        });
        win.set_location(Location::Center);
        win.set_visible(true);
    }

    #[test]
    fn test_custom_request_and_string_message() {
        let main_loop = Rc::new(MainLoop::new());
        let string_message = Rc::new(RefCell::new(Option::<String>::None));
        let window = window::Window::new(|_| {});
        let web_view = {
            let main_loop = Rc::clone(&main_loop);
            let mut string_message = Rc::clone(&string_message);
            webview::WebView::new(
                &window,
                move |evt| match evt {
                    common::webview::Event::CustomSchemeRequest(req) => {
                        // eprintln!("url {}", req.url());
                        req.respond(
                            "text/html; charset=utf-8",
                            "<script>__deskgap.postStringMessage('a message\\r\\n中文')</script>"
                                .as_bytes(),
                        );
                    }
                    common::webview::Event::ScriptMessage(msg) => {
                        *string_message.deref().borrow_mut() = Some(msg);
                        main_loop.quit();
                    }
                    _ => {}
                },
                vec!["test-scheme"],
            )
        };
        assert!(web_view.load_url("test-scheme://a-host"));
        main_loop.run();
        assert_eq!(
            *string_message.deref().borrow(),
            Some("a message\r\n中文".to_owned())
        );
    }

    #[test]
    fn test_custom_request_url() {
        let main_loop = Rc::new(MainLoop::new());
        let url = Rc::new(RefCell::new(Option::<String>::None));
        let window = window::Window::new(|_| {});
        let web_view = {
            let main_loop = Rc::clone(&main_loop);
            let url = Rc::clone(&url);
            webview::WebView::new(
                &window,
                move |evt| match evt {
                    common::webview::Event::CustomSchemeRequest(req) => {
                        *url.deref().borrow_mut() = Some(req.url());
                        req.respond("text/html; charset=utf-8", "".as_bytes());
                        main_loop.quit();
                    }
                    _ => {}
                },
                vec!["test-scheme"],
            )
        };
        assert!(web_view.load_url("test-scheme://a-host/a/b?x=y"));
        main_loop.run();
        assert_eq!(
            *url.deref().borrow(),
            Some("test-scheme://a-host/a/b?x=y".to_owned())
        );
    }

    #[test]
    fn test_title_change() {
        let main_loop = Rc::new(MainLoop::new());
        let titles = Rc::new(RefCell::new(Vec::<String>::new()));
        let window = window::Window::new(|_| {});
        let web_view = {
            let main_loop = Rc::clone(&main_loop);
            let titles = Rc::clone(&titles);
            webview::WebView::new(
                &window,
                move |evt| match evt {
                    common::webview::Event::CustomSchemeRequest(req) => {
                        req.respond(
                            "text/html; charset=utf-8",
                            "<script>document.title='a-title 中文'; document.title='another title'</script>".as_bytes(),
                        );
                    }
                    common::webview::Event::DocumentTitleChanged(new_title) => {
                        let mut titles = titles.deref().borrow_mut();
                        titles.push(new_title);
                        if titles.len() == 2 {
                            main_loop.quit();
                        }
                    }
                    _ => {}
                },
                vec!["test-scheme"],
            )
        };
        assert!(web_view.load_url("test-scheme://a-host/"));
        main_loop.run();
        assert_eq!(
            *titles.deref().borrow(),
            vec!["a-title 中文".to_owned(), "another title".to_owned()]
        );
    }

    #[test]
    fn test_execute_script() {
        let main_loop = Rc::new(MainLoop::new());
        let posted_message = Rc::new(RefCell::new(Option::<String>::None));
        let mut web_view_cell = Rc::new(RefCell::new(Option::<webview::WebView>::None));

        let window = window::Window::new(|_| {});
        let web_view = {
            let main_loop = Rc::clone(&main_loop);
            let posted_message = Rc::clone(&posted_message);
            let web_view_cell = Rc::clone(&web_view_cell);
            webview::WebView::new(
                &window,
                move |evt| match evt {
                    common::webview::Event::CustomSchemeRequest(req) => {
                        req.respond(
                            "text/html; charset=utf-8",
                            "<script>__deskgap.postStringMessage('')</script>".as_bytes(),
                        );
                    }
                    common::webview::Event::ScriptMessage(message) => {
                        if message.is_empty() {
                            let web_view = web_view_cell.deref().borrow();
                            let web_view = (*web_view).as_ref().unwrap();
                            web_view.execute_script(
                                "__deskgap.postStringMessage('hello from execute_script')",
                            );
                        } else {
                            *posted_message.deref().borrow_mut() = Some(message);
                            main_loop.quit();
                        }
                    }
                    _ => {}
                },
                vec!["test-scheme"],
            )
        };
        assert!(web_view.load_url("test-scheme://a-host/"));
        *web_view_cell.as_ref().borrow_mut() = Some(web_view);
        main_loop.run();
        assert_eq!(
            *posted_message.deref().borrow(),
            Some("hello from execute_script".to_owned())
        );
    }
}
