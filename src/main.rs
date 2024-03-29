#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]

// TODO: disable these before 1.0.0
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

#![allow(clippy::uninit_assumed_init)]

#![feature(const_cmp)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(const_slice_index)]
#![feature(derive_const)]
#![feature(generic_arg_infer)]
#![feature(new_uninit)]
#![feature(unchecked_math)]

#![feature(test)]
extern crate test;

mod board;
mod buffer;
mod castling;
mod chess_move;
mod color;
mod hint;
mod move_generator;
mod perft;
mod piece;
mod prelude;
mod rand;
mod settings;
mod square;
mod transposition_table;
mod util;

#[cfg(feature = "perft")]
fn perft() {
    let fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let nodes = perft::perft_threaded(fen, 8);
    println!("Perft is {nodes}");
}

#[cfg(not(feature = "perft"))]
fn perft() {
    // pass
}

fn main() {
    perft();
}
