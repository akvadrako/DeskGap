use super::platform;
pub use chairgap_common::app::App;
use chairgap_common::engine::Engine;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Context(pub(crate) Rc<dyn Engine>);

pub fn run<AppType: App<Context>, E: Engine>(engine: E) {
    ensure_main_thread!();
    let ctx = Context(Rc::new(engine));
    let default_run = platform::app::init::<Context, AppType>(ctx.clone());
    if unsafe { !ctx.0.run_event_loop() } {
        default_run()
    }
}
