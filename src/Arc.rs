use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc,Weak};


pub struct SharedPtr<T:?Sized>{
    rc:MaybeUninit<Arc<T>>
}

unsafe impl<T: ?Sized + Sync + Send> Send for SharedPtr<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for SharedPtr<T> {}


impl<T>  SharedPtr<T> {
    pub fn zeroed() -> SharedPtr<T> {
        SharedPtr {
            rc: MaybeUninit::zeroed()
        }
    }

    pub fn new(v: T) -> SharedPtr<T> {
        SharedPtr {
            rc: MaybeUninit::new(Arc::new(v))
        }
    }

    pub fn write(&mut self, v:T){
        if self.is_null(){
            self.rc.write(Arc::new(v));
        }
        else{
            unsafe{
                self.rc.assume_init_drop();
                self.rc.write(Arc::new(v));
            }
        }

    }

    pub fn assume_init(self)->Option<Arc<T>>{
        if self.is_null() {
            None
        }else{
            unsafe {
                Some(std::mem::transmute::<Self,Arc<T>>(self))
            }
        }
    }

    pub fn set_null(&mut self){
        unsafe {
            if !self.is_null() {
                self.rc.assume_init_drop();
                self.rc=MaybeUninit::zeroed();
            }
        }
    }



    pub fn weak(&self)->Option<Weak<T>>{
        if !self.is_null() {
            Some(Arc::downgrade(self))
        }else{
            None
        }
    }
}

impl <T:?Sized> SharedPtr<T>{
    pub fn is_null(&self)->bool {
        let p = self.rc.as_ptr() as *const usize;
        unsafe {
            p.read() == 0
        }
    }
}

impl<T: ?Sized> Drop for SharedPtr<T>{
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe {
                self.rc.assume_init_drop()
            }
        }
    }
}

impl<T> Deref for SharedPtr<T>{
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if self.is_null(){
                panic!("null shared deref")
            }

            self.rc.assume_init_ref()
        }
    }
}

impl <T> DerefMut for SharedPtr<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            if self.is_null(){
                panic!("null shared deref mut")
            }

            self.rc.assume_init_mut()
        }
    }
}

impl<T> Clone for SharedPtr<T>{
    fn clone(&self) -> Self {
        unsafe {
            if !self.is_null() {
                let rc = self.rc.assume_init_ref().clone();
                let may = MaybeUninit::<Arc<T>>::new(rc);
                SharedPtr {
                    rc: may
                }
            }else{
                SharedPtr {
                    rc:  MaybeUninit::<Arc<T>>::zeroed()
                }
            }
        }
    }
}

impl<T> From<Arc<T>> for SharedPtr<T>{
    fn from(r: Arc<T>) -> Self {
        unsafe {
            std::mem::transmute::<Arc<T>,Self>(r)
        }
    }
}
