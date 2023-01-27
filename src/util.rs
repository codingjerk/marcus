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
            always!(($index as usize) < $collection.len());
            *$collection.get_unchecked($index as usize)
        }
    }
}

pub(crate) use get_unchecked;

macro_rules! get_unchecked_2d {
    ($collection:expr, $index:expr, $index2:expr) => {
        unsafe {
            always!(($index as usize) < $collection.len());
            always!(($index2 as usize) < $collection[0].len());
            *$collection.get_unchecked($index as usize)
                        .get_unchecked($index2 as usize)
        }
    }
}

pub(crate) use get_unchecked_2d;

macro_rules! set_unchecked {
    ($collection:expr, $index:expr, $value:expr) => {
        unsafe {
            always!(($index as usize) < $collection.len());
            *$collection.get_unchecked_mut($index as usize) = $value;
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
