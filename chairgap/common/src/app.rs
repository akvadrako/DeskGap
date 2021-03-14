use crate::engine::Engine;
use std::rc::Rc;


pub trait App<C>: 'static {
    fn new(ctx: C) -> Self;
    fn activate(&self);
}
