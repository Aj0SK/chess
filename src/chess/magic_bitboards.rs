use super::bitboard::Bitboard;
use lazy_static::lazy_static;
use std::cmp;
use std::collections::HashMap;

lazy_static! {
    pub static ref ROOK_BLOCKER_MASKS: [Bitboard; 64] = get_rook_moves();
    pub static ref BISHOP_BLOCKER_MASKS: [Bitboard; 64] = get_bishop_moves();
    pub static ref ROOK_MAP: HashMap<(usize, Bitboard), Bitboard> = get_rook_magic_bitboards();
    pub static ref BISHOP_MAP: HashMap<(usize, Bitboard), Bitboard> = get_bishop_magic_bitboards();
}

pub fn get_rook_moves() -> [Bitboard; 64] {
    let mut res: [Bitboard; 64] = [Bitboard::default(); 64];

    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                res[i * 8 + j].set(i, k);
                res[i * 8 + j].set(k, j);
            }
            res[i * 8 + j].unset(i, j);
            res[i * 8 + j].unset(i, 0);
            res[i * 8 + j].unset(i, 7);
            res[i * 8 + j].unset(0, j);
            res[i * 8 + j].unset(7, j);
        }
    }

    res
}

pub fn get_bishop_moves() -> [Bitboard; 64] {
    let mut res: [Bitboard; 64] = [Bitboard::default(); 64];

    for i in 0..8 {
        for j in 0..8 {
            let m = cmp::min(i, j);
            let ni = i - m;
            let nj = j - m;
            let diagonal_length = cmp::min(8 - ni, 8 - nj);
            for k in 0..diagonal_length {
                res[i * 8 + j].set(ni + k, nj + k);
            }
            res[i * 8 + j].unset(i, j);
            res[i * 8 + j].unset(ni, nj);
            res[i * 8 + j].unset(ni + diagonal_length - 1, nj + diagonal_length - 1);
        }
    }

    for i in 0..8 {
        for j in 0..8 {
            let m = cmp::min(7 - i, j);
            let ni = i + m;
            let nj = j - m;
            let diagonal_length = cmp::min(ni + 1, 8 - nj);
            for k in 0..diagonal_length {
                res[i * 8 + j].set(ni - k, nj + k);
            }
            res[i * 8 + j].unset(i, j);
            res[i * 8 + j].unset(ni, nj);
            res[i * 8 + j].unset(ni + 1 - diagonal_length, nj + diagonal_length - 1);
        }
    }

    res
}

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

fn get_rook_magic_bitboards() -> HashMap<(usize, Bitboard), Bitboard> {
    let mut rook_map: HashMap<(usize, Bitboard), Bitboard> = HashMap::new();
    for (ind, blocker_mask) in ROOK_BLOCKER_MASKS.iter().enumerate() {
        for &blocker_board in blocker_mask.generate_subsets().iter() {
            let move_board = f(blocker_board, (ind / 8) as i32, (ind % 8) as i32, 0);
            rook_map.insert((ind, blocker_board), move_board);
        }
    }
    rook_map.shrink_to_fit();
    rook_map
}

fn get_bishop_magic_bitboards() -> HashMap<(usize, Bitboard), Bitboard> {
    let mut bishop_map: HashMap<(usize, Bitboard), Bitboard> = HashMap::new();
    for (ind, blocker_mask) in BISHOP_BLOCKER_MASKS.iter().enumerate() {
        for &blocker_board in blocker_mask.generate_subsets().iter() {
            let move_board = f(blocker_board, (ind / 8) as i32, (ind % 8) as i32, 1);
            bishop_map.insert((ind, blocker_board), move_board);
        }
    }

    bishop_map.shrink_to_fit();
    bishop_map
}
