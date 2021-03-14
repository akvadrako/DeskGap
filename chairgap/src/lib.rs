macro_rules! ensure_main_thread {
    () => {
        debug_assert!(crate::platform::app::is_main_thread())
    };
}

pub mod app;
mod fn_utils;
mod platform;
pub mod window;

pub use chairgap_common::geo;

// pub mod geo;
// pub mod menu;
// mod platform;
// pub mod webview;
// pub mod window;
//
// #[cfg(test)]
// mod test_utils;
