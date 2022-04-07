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

    #[inline]
    pub fn into_inner(mut self)->Result<T,Self>{
        if let Some(ref mut rc)=self.rc{
            // check unique
            if Rc::get_mut(rc ).is_some(){
                let ptr= Rc::into_raw(self.rc.take().unwrap());
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

    /// if ptr is no unique,none will be returned, or Some(&mut T) be returned.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if let Some(ref mut rc)=self.rc {
            Rc::get_mut(rc)
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
            if Rc::get_mut(rc ).is_some(){
                let ptr= Rc::into_raw(self.rc.take().unwrap());
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
