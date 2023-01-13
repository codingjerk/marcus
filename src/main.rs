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

mod buffer;
mod castling;
mod chess_move;
mod color;
mod hint;
mod piece;
mod prelude;
mod rand;
mod settings;
mod square;
mod util;

#[cfg(feature = "mailbox")]
mod mailbox;

fn main() {
    if cfg!(feature = "perft") {
        let fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let nodes = mailbox::perft::perft(fen, 6);
        println!("Perft is {}", nodes);
    }
}
