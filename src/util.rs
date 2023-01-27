use std::mem::MaybeUninit;

#[inline(always)]
pub const unsafe fn undefined<T>() -> T {
    MaybeUninit::uninit().assume_init()
}

#[inline(always)]
pub unsafe fn undefined_box<T>() -> Box<T> {
    Box::<T>::new_uninit().assume_init()
}

macro_rules! get_unchecked {
    ($collection:expr, $index:expr) => {
        unsafe {
            let index = $index as usize;
            always!(index < $collection.len());

            *$collection.get_unchecked(index)
        }
    }
}

pub(crate) use get_unchecked;

macro_rules! get_unchecked_2d {
    ($collection:expr, $index:expr, $index2:expr) => {
        unsafe {
            let index = $index as usize;
            always!(index < $collection.len());

            let index2 = $index2 as usize;
            always!(index2 < $collection[0].len());

            *$collection.get_unchecked(index)
                        .get_unchecked(index2)
        }
    }
}

pub(crate) use get_unchecked_2d;

macro_rules! set_unchecked {
    ($collection:expr, $index:expr, $value:expr) => {
        unsafe {
            let index = $index as usize;
            always!(index < $collection.len());

            *$collection.get_unchecked_mut(index) = $value;
        }
    }
}

pub(crate) use set_unchecked;

macro_rules! unwrap_unchecked {
    ($value:expr) => {
        unsafe {
            always!($value.is_some());
            $value.unwrap_unchecked()
        }
    }
}

pub(crate) use unwrap_unchecked;
