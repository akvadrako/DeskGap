pub enum Event {
    TitleChanged(String),
    PageLoaded,
    ScriptMessage(String)
}

pub struct WebViewConfig<'a, HandleEvent: Fn(Event) + 'static> {
    preload_script: &'a str,
    handle_event_fn: HandleEvent,
}

pub trait WebView {

}
