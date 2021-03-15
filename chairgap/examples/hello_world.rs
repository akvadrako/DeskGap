use chairgap::app::{run, App, Context};
use chairgap::geo::{Point, Size};
use chairgap::window::{Event, TitleStyle, VibrancyConfig, VibrancyUpdate, Window};
use chairgap_cef::CefEngine;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

struct DemoApp {
    main_window: Window,
}

impl App for DemoApp {
    fn new() -> Self {
        let main_window = Window::new();
        main_window.set_auto_hide(true);
        main_window.set_title("Hello ChairGap");
        main_window.set_min_size(Size {
            width: 600,
            height: 360,
        });
        main_window.update_vibrancy(VibrancyUpdate {
            top: None,
            left: Some(VibrancyConfig { size: 200 }),
            bottom: None,
            right: None,
        });
        main_window.set_title_style(TitleStyle::Hidden {
            traffic_light_position: Some(Point { x: 18, y: 36 }),
        });
        main_window.set_visible(true);

        DemoApp { main_window }
    }
    fn activate(&self) {
        if !self.main_window.visible() {
            self.main_window.set_visible(true)
        } else {
            println!("I am already visible!")
        }
    }
}

fn main() {
    let engine =
        unsafe { CefEngine::load(Path::new("/Users/Shared/cefclient.app/Contents/Frameworks")) };

    run::<DemoApp, _>(engine.unwrap());
}
