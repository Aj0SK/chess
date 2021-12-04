extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate impl_ops;

mod chess;

use chess::position::*;
// use std::io;

fn main() {
    for _ in 0..100 {
        let pos: Position = rand::random();
        println!("Pos is:\n{}", pos);
    }

    return;

    /*let mut pos: Position = Position::default();
    loop {
        println!("Pos is:\n{}", pos);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let inputs: Vec<usize> = line
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        pos.make_move(inputs[0], inputs[1], inputs[2], inputs[3]);
    }*/
}
