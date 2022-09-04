use core::hint::unreachable_unchecked;

#[inline(always)]
pub const unsafe fn always(condition: bool) {
    if !condition {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            unreachable_unchecked()
        }
    }
}
