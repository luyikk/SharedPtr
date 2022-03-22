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
    #[allow(clippy::cast_ref_to_mut)]
    unsafe fn get_mut_unchecked(&self) -> &mut T {
        &mut *(self.as_ref() as *const T as *mut T)
    }
}

unsafe impl <T: ?Sized> IGetMutUnchecked<T> for crate::Rc::SharedPtr<T> {
    #[allow(clippy::cast_ref_to_mut)]
    unsafe fn get_mut_unchecked(&self) -> &mut T {
        &mut *(self.as_ref() as *const T as *mut T)
    }
}