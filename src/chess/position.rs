use super::bitboard::Bitboard;
use std::fmt;

const WHITE_PIECES: [&str; 6] = ["♙", "♖", "♘", "♗", "♕", "♔"];
const BLACK_PIECES: [&str; 6] = ["♟︎", "♜", "♞", "♝", "♛", "♚"];

#[derive(Debug, Copy, Clone)]
pub struct Position {
    white: Bitboard,
    pawn: Bitboard,
    rook: Bitboard,
    knight: Bitboard,
    bishop: Bitboard,
    queen: Bitboard,
    king: Bitboard,
    other: u64,
}

impl Position {
    fn get_taken_bitboard(&self) -> Bitboard {
        self.pawn | self.rook | self.knight | self.bishop | self.queen | self.king
    }

    fn get_free_bitboard(&self) -> Bitboard {
        !self.get_taken_bitboard()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            white: Bitboard::new(0xffff),
            pawn: Bitboard::new((0xff << 8) | (0xff << 48)),
            rook: Bitboard::new((0b10_000_001) | (0b10_000_001 << 56)),
            knight: Bitboard::new((0b01_000_010) | (0b01_000_010 << 56)),
            bishop: Bitboard::new((0b00_100_100) | (0b00_100_100 << 56)),
            queen: Bitboard::new((0b00_001_000) | (0b00_001_000 << 56)),
            king: Bitboard::new((0b00_010_000) | (0b00_010_000 << 56)),
            other: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let free = self.get_free_bitboard();

        if self.other == 0 {
            write!(f, "White on move.\n").unwrap();
        } else {
            write!(f, "Black on move.\n").unwrap();
        }

        for i in (0..8).rev() {
            write!(f, "{} ", i + 1).unwrap();
            for j in 0..8 {
                let color = if (j + i % 2) % 2 != 0 { "'" } else { " " };
                if free.is_set(i, j) {
                    write!(f, "[ {}]", color).unwrap();
                    continue;
                }

                let pieces_to_use = if self.white.is_set(i, j) {
                    WHITE_PIECES
                } else {
                    BLACK_PIECES
                };

                for (ind, &piece_bitfield) in [
                    self.pawn,
                    self.rook,
                    self.knight,
                    self.bishop,
                    self.queen,
                    self.king,
                ]
                .iter()
                .enumerate()
                {
                    if piece_bitfield.is_set(i, j) {
                        write!(f, "[{}{}]", pieces_to_use[ind], color).unwrap();
                        break;
                    }
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "   a   b   c   d   e   f   g   h\n")
    }
}
