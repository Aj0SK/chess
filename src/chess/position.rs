use super::bitboard::Bitboard;
use super::magic_bitboards::{BISHOP_BLOCKER_MASKS, BISHOP_MAP, ROOK_BLOCKER_MASKS, ROOK_MAP};
use std::fmt;

const WHITE_PIECES: [&str; 6] = ["♙", "♖", "♘", "♗", "♕", "♔"];
const BLACK_PIECES: [&str; 6] = ["♟︎", "♜", "♞", "♝", "♛", "♚"];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PlayerOnMove {
    White = 0,
    Black = 1,
}

#[derive(ToPrimitive, FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
enum ChessPiece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

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

    fn get_white_pieces(&self) -> Bitboard {
        self.white
    }

    fn get_black_pieces(&self) -> Bitboard {
        self.get_taken_bitboard() - self.white
    }

    fn get_free_bitboard(&self) -> Bitboard {
        !self.get_taken_bitboard()
    }

    fn get_player_on_move(&self) -> PlayerOnMove {
        if self.other & 1 == 1 {
            PlayerOnMove::Black
        } else {
            PlayerOnMove::White
        }
    }

    // Returns type of chess piece standing on [i, j]
    // Function assumes that some piece is standing on it
    fn get_piece_on_position(&self, i: usize, j: usize) -> ChessPiece {
        let ind = [
            self.pawn,
            self.rook,
            self.knight,
            self.bishop,
            self.queen,
            self.king,
        ]
        .iter()
        .position(|&piece_bitfield| piece_bitfield.is_set(i, j))
        .unwrap();

        num::FromPrimitive::from_usize(ind).unwrap()
    }

    fn get_valid_pawn_moves(&self, i: usize, j: usize) -> Bitboard {
        let player_on_move = self.get_player_on_move();

        let my_pawn = match player_on_move {
            PlayerOnMove::White => self.pawn & (1 << (i * 8 + j)) & self.white,
            PlayerOnMove::Black => self.pawn & (1 << (i * 8 + j)) & (!self.white),
        };

        assert_eq!(my_pawn.is_set(i, j), true);

        let classic_move = if player_on_move == PlayerOnMove::White {
            my_pawn << 8
        } else {
            my_pawn >> 8
        };

        let diagonal_move = if player_on_move == PlayerOnMove::White {
            ((my_pawn << 7) | (my_pawn << 9)) & self.get_black_pieces()
        } else {
            ((my_pawn >> 7) | (my_pawn >> 9)) & self.get_white_pieces()
        };

        let starting_move = if player_on_move == PlayerOnMove::White {
            ((my_pawn << 8) & self.get_free_bitboard()) << 8
        } else {
            ((my_pawn >> 8) & self.get_free_bitboard()) >> 8
        };

        let all_moves = classic_move | diagonal_move | starting_move;
        match player_on_move {
            PlayerOnMove::White => all_moves & (!self.get_white_pieces()),
            PlayerOnMove::Black => all_moves & (!self.get_black_pieces()),
        }
    }

    fn get_valid_knight_moves(&self, i: usize, j: usize) -> Bitboard {
        let player_on_move = self.get_player_on_move();

        let my_knight = Bitboard::new(1 << (i * 8 + j));

        assert_eq!(self.knight.is_set(i, j), true);

        let all_moves =
            (my_knight << 15) | (my_knight << 17) | (my_knight >> 15) | (my_knight >> 17);

        match player_on_move {
            PlayerOnMove::White => all_moves & (!self.get_white_pieces()),
            PlayerOnMove::Black => all_moves & (!self.get_black_pieces()),
        }
    }

    fn get_valid_rook_moves(&self, i: usize, j: usize) -> Bitboard {
        let all_moves = ROOK_MAP
            .get(&(
                i * 8 + j,
                self.get_taken_bitboard() & ROOK_BLOCKER_MASKS[i * 8 + j],
            ))
            .unwrap();
        match self.get_player_on_move() {
            PlayerOnMove::White => *all_moves & (!self.get_white_pieces()),
            PlayerOnMove::Black => *all_moves & (!self.get_black_pieces()),
        }
    }

    fn get_valid_bishop_moves(&self, i: usize, j: usize) -> Bitboard {
        let all_moves = BISHOP_MAP
            .get(&(
                i * 8 + j,
                self.get_taken_bitboard() & BISHOP_BLOCKER_MASKS[i * 8 + j],
            ))
            .unwrap();
        match self.get_player_on_move() {
            PlayerOnMove::White => *all_moves & (!self.get_white_pieces()),
            PlayerOnMove::Black => *all_moves & (!self.get_black_pieces()),
        }
    }

    fn is_valid_move(
        &self,
        chess_piece: ChessPiece,
        i: usize,
        j: usize,
        k: usize,
        l: usize,
    ) -> bool {
        match chess_piece {
            ChessPiece::Pawn => self.get_valid_pawn_moves(i, j).is_set(k, l),
            ChessPiece::Rook => self.get_valid_rook_moves(i, j).is_set(k, l),
            ChessPiece::Knight => self.get_valid_knight_moves(i, j).is_set(k, l),
            ChessPiece::Bishop => self.get_valid_bishop_moves(i, j).is_set(k, l),
            ChessPiece::Queen => {
                (self.get_valid_knight_moves(i, j) | self.get_valid_bishop_moves(i, j)).is_set(k, l)
            }
            ChessPiece::King => false,
        }
    }

    pub fn make_move(&mut self, i: usize, j: usize, k: usize, l: usize) {
        assert_eq!(self.get_taken_bitboard().is_set(i, j), true);

        let chess_piece = self.get_piece_on_position(i, j);

        assert_eq!(self.is_valid_move(chess_piece, i, j, k, l), true);

        let ind = num::ToPrimitive::to_usize(&chess_piece).unwrap();

        if self.get_player_on_move() == PlayerOnMove::White {
            assert_eq!(self.white.is_set(i, j), true);
            self.white.unset(i, j);
            self.white.set(k, l);
        } else {
            if self.white.is_set(k, l) {
                self.white.unset(k, l)
            }
            assert_eq!(
                (self.get_taken_bitboard() & (!self.white)).is_set(i, j),
                true
            );
        }

        let helper = &mut [
            &mut self.pawn,
            &mut self.rook,
            &mut self.knight,
            &mut self.bishop,
            &mut self.queen,
            &mut self.king,
        ];

        helper[ind].unset(i, j);
        helper[ind].set(k, l);

        self.change_player_on_move()
    }

    fn change_player_on_move(&mut self) {
        self.other ^= 1;
    }

    fn get_pieces_on_move(&self, piece: ChessPiece) -> Bitboard {
        let player_mask = match self.get_player_on_move() {
            PlayerOnMove::White => self.white,
            PlayerOnMove::Black => !self.white,
        };
        let piece_mask = match piece {
            ChessPiece::Pawn => self.pawn,
            ChessPiece::Rook => self.rook,
            ChessPiece::Knight => self.knight,
            ChessPiece::Bishop => self.bishop,
            ChessPiece::Queen => self.queen,
            ChessPiece::King => self.king,
        };

        piece_mask & player_mask
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            white: Bitboard::new(0xffff),
            pawn: Bitboard::new((0xff << 8) | (0xff << 48)),
            rook: Bitboard::new((0b1000_0001) | (0b1000_0001 << 56)),
            knight: Bitboard::new((0b0100_0010) | (0b0100_0010 << 56)),
            bishop: Bitboard::new((0b0010_0100) | (0b0010_0100 << 56)),
            queen: Bitboard::new((0b0000_1000) | (0b0000_1000 << 56)),
            king: Bitboard::new((0b0001_0000) | (0b0001_0000 << 56)),
            other: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let free = self.get_free_bitboard();

        match self.get_player_on_move() {
            PlayerOnMove::White => {
                write!(f, "White on move.\n").unwrap();
            }
            PlayerOnMove::Black => {
                write!(f, "Black on move.\n").unwrap();
            }
        }

        for i in (0..8).rev() {
            write!(f, "{} ", i).unwrap();
            //write!(f, "{} ", i + 1).unwrap();
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
        write!(f, "   0   1   2   3   4   5   6   7\n")
        //write!(f, "   a   b   c   d   e   f   g   h\n")
    }
}
