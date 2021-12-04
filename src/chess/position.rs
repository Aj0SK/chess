extern crate rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use super::bitboard::Bitboard;
use super::chess_piece::ChessPiece;
use super::chess_player::ChessPlayer;
use super::magic_bitboards::{
    BISHOP_BLOCKER_MASKS, BISHOP_MAP, KING_POSSIBLE_MOVES, ROOK_BLOCKER_MASKS, ROOK_MAP,
};
use std::fmt;

const WHITE_PIECES: [&str; 6] = ["♙", "♖", "♘", "♗", "♕", "♔"];
const BLACK_PIECES: [&str; 6] = ["♟︎", "♜", "♞", "♝", "♛", "♚"];

#[derive(Debug, Copy, Clone, Hash)]
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

    fn get_white_pieces(&self) -> Bitboard {
        self.white
    }

    fn get_black_pieces(&self) -> Bitboard {
        self.get_taken_bitboard() - self.white
    }

    fn get_pieces_of_player(&self, player: ChessPlayer) -> Bitboard {
        match player {
            ChessPlayer::White => self.get_white_pieces(),
            ChessPlayer::Black => self.get_black_pieces(),
        }
    }

    fn change_player_on_move(&mut self) {
        self.other ^= 1;
    }

    fn get_player_on_move(&self) -> ChessPlayer {
        match self.other & 1 {
            0 => ChessPlayer::White,
            _ => ChessPlayer::Black,
        }
    }

    fn get_pieces_on_move(&self) -> Bitboard {
        match self.get_player_on_move() {
            ChessPlayer::White => self.get_white_pieces(),
            ChessPlayer::Black => self.get_black_pieces(),
        }
    }

    fn get_piece_mask(&self, piece: ChessPiece) -> Bitboard {
        match piece {
            ChessPiece::Pawn => self.pawn,
            ChessPiece::Rook => self.rook,
            ChessPiece::Knight => self.knight,
            ChessPiece::Bishop => self.bishop,
            ChessPiece::Queen => self.queen,
            ChessPiece::King => self.king,
        }
    }

    fn get_piece_type_by_player(&self, piece: ChessPiece, player: ChessPlayer) -> Bitboard {
        let player_mask = match player {
            ChessPlayer::White => self.get_white_pieces(),
            ChessPlayer::Black => self.get_black_pieces(),
        };
        player_mask & self.get_piece_mask(piece)
    }

    fn get_on_move_piece_type(&self, piece: ChessPiece) -> Bitboard {
        self.get_pieces_on_move() & self.get_piece_mask(piece)
    }

    // Returns type of chess piece standing on [i, j]
    // Function assumes that some piece is standing on it
    fn get_piece_on_position(&self, i: usize, j: usize) -> Option<ChessPiece> {
        let index = [
            &self.pawn,
            &self.rook,
            &self.knight,
            &self.bishop,
            &self.queen,
            &self.king,
        ]
        .iter()
        .position(|&piece_bitfield| piece_bitfield.is_set(i, j));

        match index {
            Some(position) => Some(num::FromPrimitive::from_usize(position).unwrap()),
            None => None,
        }
    }

    fn get_valid_pawn_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        assert!(self
            .get_piece_type_by_player(ChessPiece::Pawn, player)
            .is_set(i, j));
        let my_pawn = self.get_piece_type_by_player(ChessPiece::Pawn, player) & (1 << (i * 8 + j));
        assert_eq!(my_pawn.is_set(i, j), true);

        let classic_move = match player {
            ChessPlayer::White => (my_pawn << 8) & self.get_free_bitboard(),
            ChessPlayer::Black => (my_pawn >> 8) & self.get_free_bitboard(),
        };

        let diagonal_move = match player {
            ChessPlayer::White => ((my_pawn << 7) | (my_pawn << 9)) & self.get_black_pieces(),
            ChessPlayer::Black => ((my_pawn >> 7) | (my_pawn >> 9)) & self.get_white_pieces(),
        };

        let starting_move = match player {
            ChessPlayer::White => ((my_pawn << 8) & self.get_free_bitboard()) << 8,
            ChessPlayer::Black => ((my_pawn >> 8) & self.get_free_bitboard()) >> 8,
        };

        let all_moves = classic_move | diagonal_move | starting_move;
        all_moves & (!self.get_pieces_of_player(player))
    }

    fn get_valid_knight_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        assert!(self
            .get_piece_type_by_player(ChessPiece::Knight, player)
            .is_set(i, j));
        let my_knight =
            self.get_piece_type_by_player(ChessPiece::Knight, player) & (1 << (i * 8 + j));

        assert_eq!(my_knight.is_set(i, j), true);

        let all_moves =
            (my_knight << 15) | (my_knight << 17) | (my_knight >> 15) | (my_knight >> 17);
        all_moves & (!self.get_pieces_of_player(player))
    }

    fn get_valid_rook_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        let all_moves = ROOK_MAP
            .get(&(
                i * 8 + j,
                self.get_taken_bitboard() & ROOK_BLOCKER_MASKS[i * 8 + j],
            ))
            .unwrap();
        *all_moves & (!self.get_pieces_of_player(player))
    }

    fn get_valid_bishop_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        let all_moves = BISHOP_MAP
            .get(&(
                i * 8 + j,
                self.get_taken_bitboard() & BISHOP_BLOCKER_MASKS[i * 8 + j],
            ))
            .unwrap();

        *all_moves & (!self.get_pieces_of_player(player))
    }

    fn get_valid_queen_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        assert!(self
            .get_piece_type_by_player(ChessPiece::Queen, player)
            .is_set(i, j));
        self.get_valid_rook_moves(i, j, player) | self.get_valid_bishop_moves(i, j, player)
    }

    fn get_valid_king_moves(&self, i: usize, j: usize, player: ChessPlayer) -> Bitboard {
        assert!(self
            .get_piece_type_by_player(ChessPiece::King, player)
            .is_set(i, j));
        KING_POSSIBLE_MOVES[i * 8 + j] & (!self.get_pieces_of_player(player))
    }

    fn is_valid_move(
        &self,
        chess_piece: ChessPiece,
        i: usize,
        j: usize,
        k: usize,
        l: usize,
    ) -> bool {
        let player_on_move = self.get_player_on_move();
        match chess_piece {
            ChessPiece::Pawn => self.get_valid_pawn_moves(i, j, player_on_move).is_set(k, l),
            ChessPiece::Rook => self.get_valid_rook_moves(i, j, player_on_move).is_set(k, l),
            ChessPiece::Knight => self
                .get_valid_knight_moves(i, j, player_on_move)
                .is_set(k, l),
            ChessPiece::Bishop => self
                .get_valid_bishop_moves(i, j, player_on_move)
                .is_set(k, l),
            ChessPiece::Queen => self
                .get_valid_queen_moves(i, j, player_on_move)
                .is_set(k, l),
            ChessPiece::King => self.get_valid_king_moves(i, j, player_on_move).is_set(k, l),
        }
    }

    fn is_position_valid(&self) -> bool {
        let piece_map = [
            &self.pawn,
            &self.rook,
            &self.knight,
            &self.bishop,
            &self.queen,
            &self.king,
        ];
        for i in 0..64 {
            let total_on_pos: i32 = piece_map
                .iter()
                .map(|piece_bitfield| {
                    if piece_bitfield.is_set(i / 8, i % 8) {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            if total_on_pos >= 2 {
                return false;
            }
        }
        true
    }

    pub fn make_move(&mut self, i: usize, j: usize, k: usize, l: usize) -> bool {
        assert!(i < 8 && j < 8 && (i * 8 + j < 64));
        assert_ne!((i, j), (k, l));
        assert_eq!(self.get_taken_bitboard().is_set(i, j), true);
        assert!(self.is_position_valid());

        let chess_piece = self.get_piece_on_position(i, j).unwrap();
        let player_on_move = self.get_player_on_move();
        let eliminated = self.get_piece_on_position(k, l);

        assert_eq!(self.is_valid_move(chess_piece, i, j, k, l), true);
        assert_eq!(
            self.get_piece_type_by_player(ChessPiece::King, player_on_move.get_opponent())
                .is_set(k, l),
            false
        );
        if player_on_move == ChessPlayer::White {
            assert_eq!(self.get_white_pieces().is_set(i, j), true);
            self.white.clear(i, j);
            self.white.set(k, l);
        } else {
            if self.white.is_set(k, l) {
                self.white.clear(k, l)
            }
            assert!(self.get_black_pieces().is_set(i, j));
        }

        let helper = &mut [
            &mut self.pawn,
            &mut self.rook,
            &mut self.knight,
            &mut self.bishop,
            &mut self.queen,
            &mut self.king,
        ];

        let ind = num::ToPrimitive::to_usize(&chess_piece).unwrap();
        assert!(helper[ind].is_set(i, j));

        if eliminated.is_some() {
            let ind_of_end = num::ToPrimitive::to_usize(&eliminated.unwrap()).unwrap();
            helper[ind_of_end].clear(k, l);
        }

        helper[ind].clear(i, j);
        helper[ind].set(k, l);

        if chess_piece == ChessPiece::Pawn {
            match player_on_move {
                ChessPlayer::White => {
                    if k == 7 {
                        helper[ind].clear(k, l);
                        helper[4].set(k, l);
                    }
                }
                ChessPlayer::Black => {
                    if k == 0 {
                        helper[ind].clear(k, l);
                        helper[4].set(k, l);
                    }
                }
            };
        }

        assert!(self.get_players_king_pos(player_on_move) < 64);
        assert!(self.get_players_king_pos(player_on_move.get_opponent()) < 64);
        assert!(self.is_position_valid());

        if self.is_in_check(player_on_move) {
            return false;
        }

        self.change_player_on_move();
        true
    }

    fn get_attacked_by_pawns(&self, by_player: ChessPlayer) -> Bitboard {
        let my_pawns = self.get_piece_type_by_player(ChessPiece::Pawn, by_player);
        let classic_move = match by_player {
            ChessPlayer::White => (my_pawns << 8) & self.get_free_bitboard(),
            ChessPlayer::Black => (my_pawns >> 8) & self.get_free_bitboard(),
        };
        let diagonal_move = match by_player {
            ChessPlayer::White => ((my_pawns << 7) | (my_pawns << 9)) & self.get_black_pieces(),
            ChessPlayer::Black => ((my_pawns >> 7) | (my_pawns >> 9)) & self.get_white_pieces(),
        };
        let starting_move = match by_player {
            ChessPlayer::White => ((my_pawns << 8) & self.get_free_bitboard()) << 8,
            ChessPlayer::Black => ((my_pawns >> 8) & self.get_free_bitboard()) >> 8,
        };
        let all_moves = classic_move | diagonal_move | starting_move;
        all_moves & !self.get_pieces_of_player(by_player)
    }

    fn get_attacked_by_knight(&self, by_player: ChessPlayer) -> Bitboard {
        let my_knights = self.get_piece_type_by_player(ChessPiece::Knight, by_player);

        let all_moves =
            (my_knights << 15) | (my_knights << 17) | (my_knights >> 15) | (my_knights >> 17);
        all_moves & !self.get_pieces_of_player(by_player)
    }

    fn get_attacked_by_other(&self, by_player: ChessPlayer, piece: ChessPiece) -> Bitboard {
        let pos = self.get_piece_type_by_player(piece, by_player).get_ones();
        let mut attacked = Bitboard::default();
        for p in pos.iter() {
            match piece {
                ChessPiece::Queen => {
                    attacked = attacked | self.get_valid_queen_moves(p / 8, p % 8, by_player);
                }
                ChessPiece::Rook => {
                    attacked = attacked | self.get_valid_rook_moves(p / 8, p % 8, by_player);
                }
                ChessPiece::Bishop => {
                    attacked = attacked | self.get_valid_bishop_moves(p / 8, p % 8, by_player);
                }
                _ => {
                    panic!("Problem.");
                }
            };
        }
        attacked & !self.get_pieces_of_player(by_player)
    }

    fn get_players_king_pos(&self, player: ChessPlayer) -> usize {
        self.get_piece_type_by_player(ChessPiece::King, player)
            .trailing_zeros()
    }

    fn get_attacked_by_king(&self, by_player: ChessPlayer) -> Bitboard {
        let (i, j) = {
            let pos = self.get_players_king_pos(by_player);
            assert!(pos < 64);
            (pos / 8, pos % 8)
        };
        self.get_valid_king_moves(i, j, by_player) & !self.get_pieces_of_player(by_player)
    }

    pub fn get_attacked_positions(&self, by_player: ChessPlayer) -> Bitboard {
        self.get_attacked_by_pawns(by_player)
            | self.get_attacked_by_knight(by_player)
            | self.get_attacked_by_king(by_player)
            | self.get_attacked_by_other(by_player, ChessPiece::Rook)
            | self.get_attacked_by_other(by_player, ChessPiece::Bishop)
            | self.get_attacked_by_other(by_player, ChessPiece::Queen)
    }

    pub fn is_in_check(&self, player: ChessPlayer) -> bool {
        let attacked_by_opponent = self.get_attacked_positions(player.get_opponent());
        let resulting =
            attacked_by_opponent & self.get_piece_type_by_player(ChessPiece::King, player);
        resulting != Bitboard::new(0)
    }

    pub fn is_in_check_mate(&self, player: ChessPlayer) -> bool {
        (self.get_player_on_move() == player)
            && self.is_in_check(player)
            && (self.get_valid_moves().len() == 0)
    }

    pub fn get_valid_moves(&self) -> Vec<(usize, usize, usize, usize)> {
        let mut valid_moves = Vec::new();
        let player_on_move = self.get_player_on_move();
        let curr_player_pieces = self.get_pieces_of_player(player_on_move);
        let pieces_index = curr_player_pieces.get_ones();

        for piece_index in pieces_index.iter() {
            let (i, j) = (piece_index / 8, piece_index % 8);
            let piece_type = self.get_piece_on_position(i, j).unwrap();
            let valid_moves_mask = match piece_type {
                ChessPiece::Pawn => self.get_valid_pawn_moves(i, j, player_on_move),
                ChessPiece::Rook => self.get_valid_rook_moves(i, j, player_on_move),
                ChessPiece::Knight => self.get_valid_knight_moves(i, j, player_on_move),
                ChessPiece::Bishop => self.get_valid_bishop_moves(i, j, player_on_move),
                ChessPiece::Queen => self.get_valid_queen_moves(i, j, player_on_move),
                ChessPiece::King => self.get_valid_king_moves(i, j, player_on_move),
            };

            if valid_moves_mask == Bitboard::new(0) {
                continue;
            }

            for semi_valid_move in valid_moves_mask.get_ones().iter() {
                let (k, l) = (semi_valid_move / 8, semi_valid_move % 8);
                if self.is_valid_move(piece_type, i, j, k, l) && self.clone().make_move(i, j, k, l)
                {
                    valid_moves.push((i, j, k, l));
                }
            }
        }
        valid_moves
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
            ChessPlayer::White => {
                write!(f, "White on move.\n").unwrap();
            }
            ChessPlayer::Black => {
                write!(f, "Black on move.\n").unwrap();
            }
        }

        if self.is_in_check_mate(ChessPlayer::White) {
            write!(f, "White is in check-mate.\n").unwrap();
        } else if self.is_in_check(ChessPlayer::White) {
            write!(f, "White is in check.\n").unwrap();
        }

        if self.is_in_check_mate(ChessPlayer::Black) {
            write!(f, "Black is in check-mate.\n").unwrap();
        } else if self.is_in_check(ChessPlayer::Black) {
            write!(f, "Black is in check.\n").unwrap();
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

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        let mut pos = Position::default();
        for _ in 0..rng.gen_range(0..50) {
            let valid_moves = pos.get_valid_moves();
            if valid_moves.len() == 0 {
                break;
            }
            let random_move = valid_moves[rng.gen_range(0..valid_moves.len())];
            let successful_move =
                pos.make_move(random_move.0, random_move.1, random_move.2, random_move.3);
            assert!(successful_move);
        }
        pos
    }
}
