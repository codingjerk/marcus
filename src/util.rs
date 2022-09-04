use std::mem::MaybeUninit;

#[inline(always)]
pub unsafe fn undefined<T>() -> T {
    MaybeUninit::uninit().assume_init()
}
