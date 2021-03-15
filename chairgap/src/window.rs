use super::platform;
use crate::app::engine;
use crate::geo::{Point, Size};
pub use chairgap_common::window::{Event, Location};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::app::Context;
use crate::fn_utils::{new_context_callback, FnCell};
use chairgap_common::engine::Engine;
#[cfg(target_os = "macos")]
pub use chairgap_common::window::{TitleStyle, VibrancyConfig, VibrancyUpdate};
use std::fmt::Debug;

thread_local! {
    static FOCUSED_WINDOW_REF: RefCell<Option<Weak<WindowRef>>> = RefCell::new(None)
}

#[derive(Debug)]
pub struct WindowRef {
    inner: platform::window::Window,
}

impl WindowRef {
    fn new(handle_evt_fn: impl Fn(&Self, Event) + 'static) -> Rc<Self> {
        let (inner_callback, set_context) = new_context_callback(move |this, evt| {
            match evt {
                Event::Focus => FOCUSED_WINDOW_REF.with(|win_ref| {
                    *win_ref.borrow_mut() = Some(Rc::downgrade(this));
                }),
                Event::Blur => FOCUSED_WINDOW_REF.with(|win_ref| {
                    let mut win_mut_borrow = win_ref.borrow_mut();
                    if let Some(win_weak) = win_mut_borrow.deref() {
                        if win_weak.ptr_eq(&Rc::downgrade(this)) {
                            *win_mut_borrow = None
                        }
                    }
                }),
                _ => {}
            };
            handle_evt_fn(this, evt)
        });

        let inner = platform::window::Window::new(inner_callback);
        let result = Rc::new(Self { inner });

        set_context(Rc::downgrade(&result));

        let inner = &result.inner;
        inner.set_resizable(true);
        inner.set_closable(true);
        inner.set_minimizable(true);
        inner.set_min_size(Size {
            width: 32,
            height: 32,
        });
        inner.set_max_size(Size {
            width: u32::MAX,
            height: u32::MAX,
        });
        inner.set_size(Size {
            width: 800,
            height: 600,
        });
        inner.set_location(Location::Center);

        // unsafe { engine().populate_web_view(inner.web_view_parent_handle()) };

        result
    }

    pub fn handle_event(self: &Rc<Self>, evt: Event) {
        match evt {
            Event::Focus => FOCUSED_WINDOW_REF.with(|win_ref| {
                *win_ref.borrow_mut() = Some(Rc::downgrade(self));
            }),
            Event::Blur => FOCUSED_WINDOW_REF.with(|win_ref| {
                let mut win_mut_borrow = win_ref.borrow_mut();
                if let Some(win_weak) = win_mut_borrow.deref() {
                    if win_weak.ptr_eq(&Rc::downgrade(self)) {
                        *win_mut_borrow = None
                    }
                }
            }),
            _ => {}
        }
    }

    pub(crate) fn set_event_callback(&self, _cb: impl Fn(&Self, Event) + 'static) {}

    pub fn visible(&self) -> bool {
        self.inner.visible()
    }
    pub fn set_visible(&self, visible: bool) {
        self.inner.set_visible(visible)
    }

    pub fn minimizable(&self) -> bool {
        self.inner.minimizable()
    }
    pub fn set_minimizable(&self, minimizable: bool) {
        self.inner.set_minimizable(minimizable)
    }

    pub fn resizable(&self) -> bool {
        self.inner.resizable()
    }
    pub fn set_resizable(&self, resizable: bool) {
        self.inner.set_resizable(resizable)
    }

    pub fn closable(&self) -> bool {
        self.inner.closable()
    }
    pub fn set_closable(&self, closable: bool) {
        self.inner.set_closable(closable)
    }

    pub fn set_location(&self, location: Location) {
        self.inner.set_location(location)
    }
    pub fn position(&self) -> Point {
        self.inner.position()
    }

    pub fn set_size(&self, size: Size) {
        self.inner.set_size(size)
    }
    pub fn size(&self) -> Size {
        self.inner.size()
    }

    pub fn set_min_size(&self, size: Size) {
        self.inner.set_min_size(size)
    }
    pub fn min_size(&self) -> Size {
        self.inner.min_size()
    }

    pub fn set_max_size(&self, size: Size) {
        self.inner.set_max_size(size)
    }
    pub fn max_size(&self) -> Size {
        self.inner.max_size()
    }

    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title)
    }
    pub fn title(&self) -> String {
        self.inner.title()
    }

    #[cfg(target_os = "macos")]
    pub fn set_title_style(&self, title_style: TitleStyle) {
        self.inner.set_title_style(title_style)
    }

    #[cfg(target_os = "macos")]
    pub fn set_traffic_light_visible(&self, visible: bool) {
        self.inner.set_traffic_light_visible(visible)
    }

    #[cfg(target_os = "macos")]
    pub fn update_vibrancy(&self, update: VibrancyUpdate) {
        self.inner.update_vibrancy(update)
    }
}

#[derive(Debug)]
pub struct Window {
    win_ref: Rc<WindowRef>,
    auto_hide: Rc<RefCell<bool>>,
    handle_event_fn: FnCell<WindowRef, Event>,
}

impl Deref for Window {
    type Target = WindowRef;
    fn deref(&self) -> &Self::Target {
        self.win_ref.deref()
    }
}

impl Window {
    pub fn with_focused_window<T>(func: impl FnOnce(Option<&WindowRef>) -> T) -> T {
        FOCUSED_WINDOW_REF.with(|win_ref| {
            let win = {
                let win_ref = &*win_ref.borrow();
                win_ref.as_ref().and_then(|w| w.upgrade())
            };
            func(win.as_deref())
        })
    }
    pub fn new() -> Self {
        let handle_event_fn = FnCell::<WindowRef, Event>::new();
        let auto_hide = Rc::new(RefCell::new(false));

        let win_ref = {
            let handle_event_fn = handle_event_fn.clone();
            let auto_hide = auto_hide.clone();
            WindowRef::new(move |win_ref, evt| {
                if evt == Event::Close {
                    if *auto_hide.borrow() {
                        win_ref.set_visible(false)
                    }
                }
                handle_event_fn.call(win_ref, evt)
            })
        };
        Self {
            win_ref,
            auto_hide,
            handle_event_fn,
        }
    }
    pub fn set_event_callback(&self, callback: impl Fn(&WindowRef, Event) + 'static) {
        self.handle_event_fn.update(callback)
    }

    pub fn set_auto_hide(&self, auto_hide: bool) {
        *self.auto_hide.borrow_mut() = auto_hide
    }
    pub fn auto_hide(&self) -> bool {
        *self.auto_hide.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizable() {
        let window = Window::new();
        assert_eq!(window.resizable(), true);
        window.set_resizable(false);
        assert_eq!(window.resizable(), false);
    }

    #[test]
    fn test_closable() {
        let window = Window::new();
        assert_eq!(window.closable(), true);
        window.set_closable(false);
        assert_eq!(window.closable(), false);
    }

    #[test]
    fn test_minimizable() {
        let win = Window::new();
        assert_eq!(win.minimizable(), true);
        win.set_minimizable(false);
        assert_eq!(win.minimizable(), false);
    }

    #[test]
    fn test_visible() {
        let win = Window::new();
        assert_eq!(win.visible(), false);
        win.set_visible(true);
        assert_eq!(win.visible(), true);
        win.set_visible(false);
        assert_eq!(win.visible(), false);
    }

    #[test]
    fn test_size() {
        let win = Window::new();
        assert_eq!(
            win.size(),
            Size {
                width: 800,
                height: 600
            }
        );
        win.set_size(Size {
            width: 123,
            height: 456,
        });
        assert_eq!(
            win.size(),
            Size {
                width: 123,
                height: 456
            }
        );
    }

    #[test]
    fn test_min_size() {
        let win = Window::new();
        assert_eq!(
            win.min_size(),
            Size {
                width: 32,
                height: 32
            }
        );
        win.set_min_size(Size {
            width: 50,
            height: 60,
        });
        assert_eq!(
            win.min_size(),
            Size {
                width: 50,
                height: 60
            }
        );
    }

    #[test]
    fn test_max_size() {
        let win = Window::new();
        assert_eq!(
            win.max_size(),
            Size {
                width: u32::max_value(),
                height: u32::max_value()
            }
        );
        win.set_max_size(Size {
            width: 5000,
            height: 6000,
        });
        assert_eq!(
            win.max_size(),
            Size {
                width: 5000,
                height: 6000
            }
        );
    }

    #[test]
    fn test_location_exact() {
        let win = Window::new();
        win.set_location(Location::Exact(Point { x: -10, y: 88 }));
        assert_eq!(win.position(), Point { x: -10, y: 88 });
    }

    #[test]
    fn test_location_center() {
        let win = Window::new();
        let center_position = win.position();

        win.set_location(Location::Exact(Point { x: 0, y: 0 }));
        assert_ne!(win.position(), center_position);

        win.set_location(Location::Center);
        assert_eq!(win.position(), center_position);
    }

    #[test]
    fn test_set_size_not_changing_position() {
        let win = Window::new();
        let position = win.position();
        win.set_size(Size {
            width: 123,
            height: 456,
        });
        assert_eq!(position, win.position());
    }

    #[test]
    fn test_set_position_not_changing_size() {
        let win = Window::new();
        let size = win.size();
        win.set_location(Location::Exact(Point { x: 123, y: 456 }));
        assert_eq!(size, win.size());
    }

    #[test]
    fn test_title() {
        let win = Window::new();
        assert_eq!(win.title(), "");
        win.set_title("eng \0中文");
        assert_eq!(win.title(), "eng \0中文");
        win.set_title("");
        assert_eq!(win.title(), "");
    }
}
