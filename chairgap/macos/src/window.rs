use crate::convert::ConvertTo;
use crate::debug::DebugStrongPtr;
use crate::strings::{ns_string, ns_to_string};
use block::{Block, ConcreteBlock, RcBlock};
use chairgap_common::engine::Engine;
use chairgap_common::geo::{Point, Size};
use chairgap_common::window::{Event, Location, TitleStyle, VibrancyUpdate};
use chairgap_macos_src::*;
use cocoa::foundation::{NSPoint, NSSize};
use objc::rc::StrongPtr;
use objc::runtime::{BOOL, NO, YES};
use objc::*;
use std::cell::Cell;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Window(DebugStrongPtr);

impl Drop for Window {
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![*self.0, close] };
    }
}

impl Window {
    pub fn new(handle_event: impl Fn(Event) + 'static) -> Self {
        let event_callback = ConcreteBlock::new(move |evt_type: DGWindowEventType| {
            handle_event(match evt_type {
                DGWindowEventType::Blur => Event::Blur,
                DGWindowEventType::Focus => Event::Focus,
                DGWindowEventType::Move => Event::Move,
                DGWindowEventType::Close => Event::Close,
                DGWindowEventType::Resize => Event::Resize,
                _ => panic!("Illegal DGWindowEventType value: {:?}", evt_type),
            });
        })
        .copy();
        let event_callback: &Block<(DGWindowEventType,), ()> = &event_callback;

        let window_controller: DebugStrongPtr =
            { unsafe { StrongPtr::retain(DGWindowControllerNew(event_callback)).into() } };

        // unsafe { engine.populate_web_view(msg_send![*window_controller, contentView]) }

        Self {
            0: window_controller,
        }
    }

    pub fn web_view_container_handle(&self) -> *mut c_void {
        unsafe { msg_send![*self.0, webViewContainer] }
    }

    pub fn set_visible(&self, visible: bool) {
        let _: () = unsafe { msg_send![*self.0, setVisible: visible as BOOL] };
    }

    pub fn update_vibrancy(&self, update: VibrancyUpdate) {
        let top: DGVisualEffectViewConfig = update.top.convert();
        let left: DGVisualEffectViewConfig = update.left.convert();
        let bottom: DGVisualEffectViewConfig = update.bottom.convert();
        let right: DGVisualEffectViewConfig = update.right.convert();
        unsafe {
            let _: () = msg_send![*self.0,
                setVisualEffectViewTop: top
                left: left
                bottom: bottom
                right: right
            ];
        }
    }

    pub fn visible(&self) -> bool {
        let oc_visible: BOOL = unsafe { msg_send![*self.0, visible] };
        oc_visible != NO
    }

    pub fn set_frame<CallBackFn: FnOnce() + 'static>(
        &self,
        location: Option<Location>,
        size: Option<Size>,
        animation_callback: Option<CallBackFn>,
    ) {
        // TODO: Investigate whether it's ok to just pass Option<&...> to msg_send;
        let mut _ns_size = NSSize::new(0.0, 0.0);
        let ns_size_ptr = if let Some(size) = size {
            _ns_size = size.convert();
            &_ns_size as *const NSSize
        } else {
            null()
        };

        let mut _ns_position = NSPoint::new(0.0, 0.0);
        let (ns_location_ptr, center) = match location {
            None => (null(), false),
            Some(Location::Center) => (null(), true),
            Some(Location::Exact(position)) => {
                _ns_position = position.convert();
                (&_ns_position as *const NSPoint, false)
            }
        };

        let block: Option<RcBlock<(), ()>> = animation_callback.map(|cb| {
            let cb = Cell::new(Some(cb));
            ConcreteBlock::new(move || {
                let cb = cb
                    .take()
                    .expect("animation_callback got called more than once");
                cb()
            })
            .copy()
        });
        let block_ref = block.as_ref().map(|block_ref| block_ref as &Block<(), ()>);

        unsafe {
            let _: () = msg_send![*self.0, setLocation: ns_location_ptr center: center size: ns_size_ptr withAnimationFinished: block_ref];
        }
    }
    pub fn set_location(&self, location: Location) {
        self.set_frame::<Box<dyn FnOnce()>>(Some(location), None, None);
    }
    pub fn position(&self) -> Point {
        let ns_position: NSPoint = unsafe { msg_send![*self.0, position] };
        ns_position.convert()
    }

    pub fn set_size(&self, size: Size) {
        self.set_frame::<Box<dyn FnOnce()>>(None, Some(size), None);
    }
    pub fn size(&self) -> Size {
        let ns_size: NSSize = unsafe { msg_send![*self.0, size] };
        ns_size.convert()
    }

    pub fn set_max_size(&self, max_size: Size) {
        let ns_size: NSSize = max_size.convert();
        let _: () = unsafe { msg_send![*self.0, setMaxSize: ns_size] };
    }
    pub fn max_size(&self) -> Size {
        let ns_size: NSSize = unsafe { msg_send![*self.0, maxSize] };
        ns_size.convert()
    }

    pub fn set_min_size(&self, min_size: Size) {
        let ns_size: NSSize = min_size.convert();
        let _: () = unsafe { msg_send![*self.0, setMinSize: ns_size] };
    }
    pub fn min_size(&self) -> Size {
        let ns_size: NSSize = unsafe { msg_send![*self.0, minSize] };
        ns_size.convert()
    }

    pub fn set_closable(&self, closable: bool) {
        let _: () = unsafe { msg_send![*self.0, setClosable: closable as BOOL] };
    }
    pub fn closable(&self) -> bool {
        let oc_closable: BOOL = unsafe { msg_send![*self.0, closable] };
        oc_closable != NO
    }

    pub fn set_resizable(&self, resizable: bool) {
        let _: () = unsafe { msg_send![*self.0, setResizable: resizable as BOOL] };
    }
    pub fn resizable(&self) -> bool {
        let oc_resizable: BOOL = unsafe { msg_send![*self.0, resizable] };
        oc_resizable != NO
    }

    pub fn set_minimizable(&self, minimizable: bool) {
        let _: () = unsafe { msg_send![*self.0, setMinimizable: minimizable as BOOL] };
    }
    pub fn minimizable(&self) -> bool {
        let oc_minimizable: BOOL = unsafe { msg_send![*self.0, minimizable] };
        oc_minimizable != NO
    }

    pub fn set_title(&self, title: &str) {
        let oc_title = ns_string(title);
        let _: () = unsafe { msg_send![*self.0, setTitle: *oc_title] };
    }

    fn set_title_bar_visible(&self, visible: bool) {
        let _: () =
            unsafe { msg_send![*self.0, setTitleBarVisible: if visible { YES } else { NO }] };
    }
    pub fn set_traffic_light_visible(&self, visible: bool) {
        let _: () =
            unsafe { msg_send![*self.0, setTrafficLightVisible: if visible { YES } else { NO }] };
    }
    fn set_customized_traffic_light_position(&self, position: Option<Point>) {
        let (customized, position) = match position {
            None => (NO, NSPoint::new(0.0, 0.0)),
            Some(position) => (YES, position.convert()),
        };
        let _: () =
            unsafe { msg_send![*self.0, setCustomized: customized trafficLightPosition: position] };
    }

    pub fn set_title_style(&self, title_style: TitleStyle) {
        match title_style {
            TitleStyle::Visible => self.set_title_bar_visible(true),
            TitleStyle::Hidden {
                traffic_light_position: window_control_position,
            } => {
                self.set_title_bar_visible(false);
                self.set_customized_traffic_light_position(window_control_position);
            }
        }
    }

    pub fn title(&self) -> String {
        let oc_title = unsafe { StrongPtr::retain(msg_send![*self.0, title]) };
        unsafe { ns_to_string(*oc_title) }
    }
    pub unsafe fn native_handle(&self) -> *mut c_void {
        *self.0 as *mut c_void
    }
}
