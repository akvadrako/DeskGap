use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub(crate) fn new_context_callback<C, A>(
    f: impl Fn(&Rc<C>, A),
) -> (impl Fn(A), impl FnOnce(Weak<C>)) {
    let context: Rc<RefCell<Option<Weak<C>>>> = Rc::new(RefCell::new(None));

    let context_clone = Rc::clone(&context);
    let callback_inner = move |arg: A| {
        let context = {
            let context_borrow = context_clone.borrow();
            context_borrow
                .as_ref()
                .expect("Callback is called before context is set")
                .clone()
        };
        let context = match context.upgrade() {
            Some(ctx) => ctx,
            None => return,
        };
        f(&context, arg)
    };

    let set_context = move |ctx: Weak<C>| *context.borrow_mut() = Some(ctx);
    (callback_inner, set_context)
}

pub(crate) struct FnCell<C, A>(Rc<RefCell<Option<Rc<dyn Fn(&C, A)>>>>);

impl<C, A> Clone for FnCell<C, A> {
    fn clone(&self) -> Self {
        FnCell(self.0.clone())
    }
}

impl<C, A> Debug for FnCell<C, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnCell").finish()
    }
}

impl<C, A> FnCell<C, A> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
    pub fn update(&self, f: impl Fn(&C, A) + 'static) {
        *self.0.borrow_mut() = Some(Rc::new(f))
    }
    pub fn call(&self, ctx: &C, arg: A) {
        if let Some(inner_fn) = self.0.borrow().clone() {
            inner_fn(ctx, arg)
        }
    }
}
