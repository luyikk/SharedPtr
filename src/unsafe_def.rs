use std::rc::Rc;
use std::sync::Arc;

///# Safety
/// Rc::get_mut Too strict, resulting in limited function,
/// we need an unsafe way to be consistent with the SharedPtr
pub unsafe trait IGetMutUnchecked<T: ?Sized>{
    ///# Safety
    /// no check unique,return &mut T
    /// panic!! ptr is none
    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut_unchecked(&self)->&mut T;
}

unsafe impl <T: ?Sized>  IGetMutUnchecked<T> for crate::Arc::SharedPtr<T> {
    unsafe fn get_mut_unchecked(&self) -> &mut T {
        &mut *(Arc::<T>::as_ptr(&**self) as *mut T)
    }
}

unsafe impl <T: ?Sized> IGetMutUnchecked<T> for crate::Rc::SharedPtr<T> {
    unsafe fn get_mut_unchecked(&self) -> &mut T {
        &mut *(Rc::<T>::as_ptr(&**self) as *mut T)
    }
}