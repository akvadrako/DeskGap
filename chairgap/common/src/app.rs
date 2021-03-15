pub trait App: 'static {
    fn new() -> Self;
    fn activate(&self);
}
