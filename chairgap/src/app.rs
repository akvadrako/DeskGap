use super::platform;
pub use chairgap_common::app::App;
use chairgap_common::engine::Engine;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Context(pub(crate) Rc<dyn Engine>);

thread_local! {
    static ENGINE: RefCell<Option<&'static dyn Engine>> = RefCell::new(None);
}
//
pub(super) fn engine() -> &'static dyn Engine {
    ENGINE.with(|engine_local| engine_local.borrow().unwrap())
}

pub fn run<AppType: App, E: Engine>(engine: E) {
    ensure_main_thread!();
    let engine = ENGINE.with(move |engine_local| {
        assert_eq!(engine_local.borrow().is_none(), true);
        let engine: &'static dyn Engine = Box::leak(Box::new(engine));
        *engine_local.borrow_mut() = Some(engine);
        engine
    });
    // let ctx = Context(Rc::new(engine));
    let default_run = platform::app::init::<AppType>();
    if unsafe { !engine.run_event_loop() } {
        default_run()
    }
}
