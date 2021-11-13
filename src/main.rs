#[macro_use]
extern crate impl_ops;

mod chess;

use chess::position::Position;

fn main() {
    let pos = Position::default();
    println!("Pos is:\n{}", pos);
}
