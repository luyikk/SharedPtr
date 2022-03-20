use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct SharedPtr<T: ?Sized> {
    rc: Option<Rc<T>>,
}

impl<T> SharedPtr<T> {
    #[inline]
    pub fn new(v: T) -> SharedPtr<T> {
        SharedPtr {
            rc: Some(Rc::new(v)),
        }
    }
    #[inline]
    pub fn write(&mut self, v: T) {
        self.rc = Some(Rc::new(v));
    }
    #[inline]
    pub fn assume_init(self) -> Option<Rc<T>> {
        self.rc
    }
}

impl<T: ?Sized> SharedPtr<T> {
    #[inline]
    pub fn zeroed() -> SharedPtr<T> {
        SharedPtr { rc: None }
    }

    #[inline]
    pub fn weak(&self) -> Option<Weak<T>> {
        self.rc.as_ref().map(Rc::downgrade)
    }
    #[inline]
    pub fn set_null(&mut self) {
        self.rc = None;
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        self.rc.is_none()
    }
    #[inline]
    pub fn get_mut_ref(&mut self) -> &mut T {
        Rc::get_mut(&mut *self).expect("shared get_mut_ref")
    }
}

impl<T: ?Sized> Deref for SharedPtr<T> {
    type Target = Rc<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.rc.as_ref().expect("null shared deref")
    }
}

impl<T: ?Sized> DerefMut for SharedPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rc.as_mut().expect("null shared deref mut")
    }
}

impl<T: ?Sized> Clone for SharedPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            rc: self.rc.clone(),
        }
    }
}

impl<T: ?Sized> From<Rc<T>> for SharedPtr<T> {
    #[inline]
    fn from(r: Rc<T>) -> Self {
        Self { rc: Some(r) }
    }
}

impl<T: ?Sized> Default for SharedPtr<T> {
    #[inline]
    fn default() -> Self {
        SharedPtr::<T>::zeroed()
    }
}
