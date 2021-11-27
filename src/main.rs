extern crate num;
#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate impl_ops;

mod chess;

use chess::bitboard::*;
use chess::position::*;

use std::io;

use std::collections::HashMap;

const DIR_I: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DIR_J: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

fn f(blocker_board: Bitboard, i: i32, j: i32, offset: i32) -> Bitboard {
    let mut res = Bitboard::default();
    for dir in (offset..8).step_by(2) {
        for steps in 0..8 {
            let ni = i + steps * DIR_I[dir as usize];
            let nj = j + steps * DIR_J[dir as usize];
            if ni < 0 || nj < 0 || ni >= 8 || nj >= 8 {
                break;
            }
            res.set(ni as usize, nj as usize);
            if blocker_board.is_set(ni as usize, nj as usize) {
                break;
            }
        }
    }
    res.unset(i as usize, j as usize);
    res
}

fn main() {
    let rook_blocker_masks = get_rook_moves();
    let bishop_blocker_masks = get_bishop_moves();

    let mut rook_map: HashMap<(usize, Bitboard), Bitboard> = HashMap::new();
    let mut bishop_map: HashMap<(usize, Bitboard), Bitboard> = HashMap::new();

    for (ind, blocker_mask) in rook_blocker_masks.iter().enumerate() {
        for &blocker_board in blocker_mask.generate_subsets().iter() {
            let move_board = f(blocker_board, (ind / 8) as i32, (ind % 8) as i32, 0);
            rook_map.insert((ind, *blocker_mask), move_board);
        }
    }

    for (ind, blocker_mask) in bishop_blocker_masks.iter().enumerate() {
        for &blocker_board in blocker_mask.generate_subsets().iter() {
            let move_board = f(blocker_board, (ind / 8) as i32, (ind % 8) as i32, 1);
            bishop_map.insert((ind, *blocker_mask), move_board);
        }
    }

    let mut pos = Position::default();
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
    }
}
