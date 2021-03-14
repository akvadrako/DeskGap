use crate::engine::Engine;
use std::rc::Rc;

pub trait App: 'static {
    fn new() -> Self;
    fn activate(&self);
}
