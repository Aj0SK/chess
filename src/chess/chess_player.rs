#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessPlayer {
    White = 0,
    Black = 1,
}

impl ChessPlayer {
    pub fn get_opponent(&self) -> ChessPlayer {
        match self {
            ChessPlayer::White => ChessPlayer::Black,
            ChessPlayer::Black => ChessPlayer::White,
        }
    }
}
