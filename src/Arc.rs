use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct SharedPtr<T: ?Sized> {
    rc: MaybeUninit<Arc<T>>,
}

unsafe impl<T: ?Sized + Sync + Send> Send for SharedPtr<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for SharedPtr<T> {}

impl<T> SharedPtr<T> {
    #[inline]
    pub fn new(v: T) -> SharedPtr<T> {
        SharedPtr {
            rc: MaybeUninit::new(Arc::new(v)),
        }
    }
    #[inline]
    pub fn write(&mut self, v: T) {
        if self.is_null() {
            self.rc.write(Arc::new(v));
        } else {
            unsafe {
                self.rc.assume_init_drop();
                self.rc.write(Arc::new(v));
            }
        }
    }
    #[inline]
    pub fn assume_init(self) -> Option<Arc<T>> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(std::mem::transmute::<Self, Arc<T>>(self)) }
        }
    }
}
impl<T: ?Sized> SharedPtr<T> {
    #[inline]
    pub fn zeroed() -> SharedPtr<T> {
        SharedPtr {
            rc: MaybeUninit::zeroed(),
        }
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        let p = self.rc.as_ptr() as *const usize;
        unsafe { p.read() == 0 }
    }
    #[inline]
    pub fn set_null(&mut self) {
        unsafe {
            if !self.is_null() {
                self.rc.assume_init_drop();
                self.rc = MaybeUninit::zeroed();
            }
        }
    }
    #[inline]
    pub fn weak(&self) -> Option<Weak<T>> {
        if !self.is_null() {
            Some(Arc::downgrade(self))
        } else {
            None
        }
    }
    ///# Safety
    #[inline]
    pub unsafe fn get_mut_ref(&self) -> &mut T {
        &mut *(self.as_ref() as *const T as *mut T)
    }
}

impl<T: ?Sized> Drop for SharedPtr<T> {
    #[inline]
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { self.rc.assume_init_drop() }
        }
    }
}

impl<T: ?Sized> Deref for SharedPtr<T> {
    type Target = Arc<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            if self.is_null() {
                panic!("null shared deref")
            }

            self.rc.assume_init_ref()
        }
    }
}

impl<T: ?Sized> DerefMut for SharedPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            if self.is_null() {
                panic!("null shared deref mut")
            }

            self.rc.assume_init_mut()
        }
    }
}

impl<T: ?Sized> Clone for SharedPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            if !self.is_null() {
                let rc = self.rc.assume_init_ref().clone();
                let may = MaybeUninit::<Arc<T>>::new(rc);
                SharedPtr { rc: may }
            } else {
                SharedPtr {
                    rc: MaybeUninit::<Arc<T>>::zeroed(),
                }
            }
        }
    }
}

impl<T: ?Sized> From<Arc<T>> for SharedPtr<T> {
    #[inline]
    fn from(r: Arc<T>) -> Self {
        unsafe {
            let ptr = &r as *const Arc<T> as *const SharedPtr<T>;
            std::mem::forget(r);
            ptr.read()
        }
    }
}

impl<T: ?Sized> Default for SharedPtr<T> {
    #[inline]
    fn default() -> Self {
        SharedPtr::<T>::zeroed()
    }
}
