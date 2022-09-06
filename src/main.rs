#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// TODO: disable these before 1.0.0
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#![feature(const_for)]
#![feature(const_slice_index)]
#![feature(generic_arg_infer)]
#![feature(unchecked_math)]

#![feature(test)]
extern crate test;

mod board;
mod buffer;
mod castling;
mod chess_move;
mod color;
mod hint;
mod piece;
mod prelude;
mod settings;
mod square;
mod util;

fn main() {
}
