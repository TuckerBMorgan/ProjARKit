use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Coord {
    pub row: usize,
    pub col: usize
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub has_moved: bool,
    pub row: usize,
    pub col: usize,
    pub color: Color
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_rep = if self.color == Color::White {
                match self.piece_type {
                    PieceType::King => "♔",
                    PieceType::Queen => "♕",
                    PieceType::Rook => "♖",
                    PieceType::Bishop => "♗",
                    PieceType::Knight => "♘",
                    _ => "♙",
                }
            } else {
                match self.piece_type {
                    PieceType::King => "♚",
                    PieceType::Queen => "♛",
                    PieceType::Rook => "♜",
                    PieceType::Bishop => "♝",
                    PieceType::Knight => "♞",
                    _ => "♟️",
            }
        };
        write!(f, "{}", piece_rep)
    }
}