use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct SharedPtr<T: ?Sized> {
    rc: Option<Arc<T>>,
}

impl<T> SharedPtr<T> {
    #[inline]
    pub fn new(v: T) -> SharedPtr<T> {
        SharedPtr {
            rc: Some(Arc::new(v)),
        }
    }
    #[inline]
    pub fn write(&mut self, v: T) {
        self.rc = Some(Arc::new(v));
    }
    #[inline]
    pub fn assume_init(self) -> Option<Arc<T>> {
        self.rc
    }

    #[inline]
    pub fn into_inner(mut self)->Result<T,Self>{
        if let Some(ref mut rc)=self.rc{
            // check unique
            if Arc::get_mut(rc ).is_some(){
                let ptr= Arc::into_raw(self.rc.take().unwrap());
                unsafe {
                    Ok(ptr.read())
                }
            }else{
                Err(self)
            }
        }else{
            Err(self)
        }
    }
}
impl<T: ?Sized> SharedPtr<T> {
    #[inline]
    pub fn zeroed() -> SharedPtr<T> {
        SharedPtr { rc: None }
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        self.rc.is_none()
    }
    #[inline]
    pub fn set_null(&mut self) {
        self.rc = None;
    }
    #[inline]
    pub fn weak(&self) -> Option<Weak<T>> {
        self.rc.as_ref().map(Arc::downgrade)
    }
    /// if ptr is no unique,none will be returned, or Some(&mut T) be returned.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if let Some(ref mut rc)=self.rc {
            Arc::get_mut(rc)
        }else{
            None
        }
    }

}

impl <T:Clone> SharedPtr<T> {
    #[inline]
    pub fn into_or_clone_inner(mut self)->Option<T>{
        if let Some(ref mut rc)=self.rc{
            // check unique
            if Arc::get_mut(rc ).is_some(){
                let ptr= Arc::into_raw(self.rc.take().unwrap());
                unsafe {
                    Some(ptr.read())
                }
            }else{
                Some(rc.as_ref().clone())
            }
        }else{
            None
        }
    }
}

impl<T: ?Sized> Deref for SharedPtr<T> {
    type Target = Arc<T>;
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

impl<T: ?Sized> From<Arc<T>> for SharedPtr<T> {
    #[inline]
    fn from(r: Arc<T>) -> Self {
        Self { rc: Some(r) }
    }
}

impl<T: ?Sized> Default for SharedPtr<T> {
    #[inline]
    fn default() -> Self {
        SharedPtr::<T>::zeroed()
    }
}

#[test]
fn assert_send_sync() {
    fn asserts<T: Send + Sync>() {}

    asserts::<SharedPtr<u32>>();
    asserts::<SharedPtr<str>>();
}
