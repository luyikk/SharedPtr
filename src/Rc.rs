use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};


pub struct SharedPtr<T:?Sized>{
    rc:MaybeUninit<Rc<T>>
}

impl<T>  SharedPtr<T>{

    pub fn zeroed()->SharedPtr<T>{
        SharedPtr{
            rc:MaybeUninit::zeroed()
        }
    }

    pub fn new(v:T)->SharedPtr<T>{
        SharedPtr{
            rc:MaybeUninit::new(Rc::new(v))
        }
    }


    pub fn write(&mut self, v:T){
        if self.is_null(){
            self.rc.write(Rc::new(v));
        }
        else{
            unsafe{
                self.rc.assume_init_drop();
                self.rc.write(Rc::new(v));
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
            Some(Rc::downgrade(self))
        }else{
            None
        }
    }

    pub fn assume_init(self)->Option<Rc<T>>{
        if self.is_null() {
            None
        }else{
            unsafe {
                Some(std::mem::transmute::<Self,Rc<T>>(self))
            }
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

impl<T:?Sized> Drop for SharedPtr<T>{
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe {
                self.rc.assume_init_drop()
            }
        }
    }
}

impl<T> Deref for SharedPtr<T>{
    type Target = Rc<T>;

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
                let may = MaybeUninit::<Rc<T>>::new(rc);
                SharedPtr {
                    rc: may
                }
            }else{
                SharedPtr {
                    rc:  MaybeUninit::<Rc<T>>::zeroed()
                }
            }
        }
    }
}

impl<T> From<Rc<T>> for SharedPtr<T>{
    fn from(r: Rc<T>) -> Self {
        unsafe {
            std::mem::transmute::<Rc<T>,Self>(r)
        }
    }
}
