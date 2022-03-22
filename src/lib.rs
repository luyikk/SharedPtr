#![allow(non_snake_case)]

pub mod Arc;
pub mod Rc;
pub mod unsafe_def;

pub trait ISetNullWeak {
    fn set_null(&mut self);
}

impl<T> ISetNullWeak for std::rc::Weak<T> {
    fn set_null(&mut self) {
        *self = Default::default();
    }
}

impl<T> ISetNullWeak for std::sync::Weak<T> {
    fn set_null(&mut self) {
        *self = Default::default();
    }
}

