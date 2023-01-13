#[cfg(debug_assertions)]
macro_rules! never {
    () => {
        panic!("unreachable condition met")
    }
}

#[cfg(not(debug_assertions))]
macro_rules! never {
    () => {
        unsafe { std::hint::unreachable_unchecked() }
    }
}

pub(crate) use never;

macro_rules! always {
    ( $condition:expr ) => {
        if !$condition {
            never!();
        }
    }
}

pub(crate) use always;
