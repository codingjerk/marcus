use std::mem::MaybeUninit;

use crate::prelude::*;

pub struct StaticBuffer<E, const SIZE: usize> {
    data: [E; SIZE],
    cursor: usize,
}

impl<E, const SIZE: usize> StaticBuffer<E, SIZE> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            // SAFETY
            // We return slice to only initialized values,
            // see `add` and `as_slice()` method
            data: unsafe {
                MaybeUninit::uninit().assume_init()
            },
            cursor: 0,
        }
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    pub const fn get(&self, index: usize) -> E
        where E: Copy
    {
        always!(index < self.cursor);

        self.data[index]
    }

    #[inline(always)]
    pub fn add(&mut self, value: E) {
        always!(self.cursor < SIZE);

        self.data[self.cursor] = value;
        self.cursor += 1;
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &[E] {
        &self.data[..self.cursor]
    }

    #[inline(always)]
    pub fn contains(&self, expected: E) -> bool
        where E: PartialEq
    {
        for value in self.as_slice() {
            if *value == expected {
                return true;
            }
        }

        false
    }

    #[inline(always)]
    pub fn restore_cursor(&mut self, value: usize) {
        self.cursor = value;
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.restore_cursor(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut buffer = StaticBuffer::<u8, 1024>::new();
        assert_eq!(buffer.len(), 0);

        buffer.add(99);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.get(0), 99);

        buffer.add(42);
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.get(0), 99);
        assert_eq!(buffer.get(1), 42);

        assert!(buffer.contains(99));
        assert!(buffer.contains(42));

        assert_eq!(buffer.as_slice(), &[99, 42]);

        buffer.reset();
        assert_eq!(buffer.len(), 0);
    }
}
