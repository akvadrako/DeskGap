use super::platform;
use crate::platform::common::webview::Event;
use crate::platform::webview::Request;
use crate::platform::window::Window as PlatformWindow;
use percent_encoding::percent_decode_str;
use std::cell::RefCell;
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

const DESKGAP_SCHEME: &'static str = "deskgap";
const DESKGAP_LOCAL_FILE_URL_PREFIX: &'static str = "deskgap://local-file/";

#[derive(Debug)]
struct LocalFileHandler {
    root_dir: RefCell<Option<PathBuf>>,
}
impl LocalFileHandler {
    fn set_root_dir(&self, root_dir: Option<PathBuf>) {
        *self.root_dir.borrow_mut() = root_dir
    }
    fn handle(&self, req: platform::webview::Request<'_>) {
        let abs_path = self._get_file_abs_path(req.url().as_str());
        let extension_and_content = abs_path.as_ref().and_then(|abs_path| {
            let file_content = fs::read(abs_path.as_path()).ok()?;
            let file_extension = abs_path.extension().and_then(|a| a.to_str()).unwrap_or("");
            Some((file_extension, file_content))
        });
        if let Some((extension, content)) = extension_and_content {
            req.respond("hello", content.as_slice());
        }
    }
    fn _get_file_abs_path(&self, url: &str) -> Option<PathBuf> {
        let root_dir = self.root_dir.borrow().clone()?;
        let mut path = url.strip_prefix(DESKGAP_LOCAL_FILE_URL_PREFIX)?;
        path = path.trim_start_matches('/');
        if let Some(question_mark_pos) = path.find('?') {
            path = &path[..question_mark_pos];
        }
        let decoded_path = percent_encoding::percent_decode_str(path)
            .decode_utf8()
            .ok()?;
        if decoded_path.contains("..") {
            return None;
        }
        let abs_path = root_dir.join(decoded_path.as_ref()).canonicalize().ok()?;
        Some(abs_path)
    }
}

#[derive(Debug)]
pub struct WebView {
    inner: platform::webview::WebView,
    local_file_handler: Rc<LocalFileHandler>,
}

#[derive(Debug)]
pub struct WebViewLoadURLError;
impl Display for WebViewLoadURLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid format of URL")
    }
}
impl Error for WebViewLoadURLError {}

impl WebView {
    pub(crate) fn new(platform_window: &PlatformWindow) -> Self {
        let local_file_handler = Rc::new(LocalFileHandler {
            root_dir: RefCell::new(Some("".into())),
        });
        let inner = {
            let local_file_handler = Rc::clone(&local_file_handler);
            platform::webview::WebView::new(
                platform_window,
                move |evt| match evt {
                    Event::DocumentTitleChanged(_) => {}
                    Event::CustomSchemeRequest(req) => local_file_handler.handle(req),
                    Event::ScriptMessage(_) => {}
                },
                vec!["deskgap"],
            )
        };
        inner.set_devtools_enabled(true);
        Self {
            inner,
            local_file_handler,
        }
    }
    pub fn set_root_dir(&self, root_dir: Option<PathBuf>) {
        self.local_file_handler.set_root_dir(root_dir)
    }

    pub fn load_url(&self, url: &str) -> Result<(), WebViewLoadURLError> {
        if self.inner.load_url(url) {
            Ok(())
        } else {
            Err(WebViewLoadURLError)
        }
    }
    pub fn load_file(&self, path: &str) {
        // self.inner.load_url(url)
    }
}
