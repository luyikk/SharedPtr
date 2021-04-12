#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_ref)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]
#![allow(non_snake_case)]

pub mod Rc;
pub mod Arc;


pub trait ISetNullWeak{
    fn set_null(&mut self);
}

impl <T> ISetNullWeak for std::rc::Weak<T>{
    fn set_null(&mut self) {
        *self=Default::default();
    }
}

impl <T> ISetNullWeak for std::sync::Weak<T>{
    fn set_null(&mut self) {
        *self=Default::default();
    }
}