#[derive(ToPrimitive, FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessPiece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}
