use std::fmt;
use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor) -> Self {
        Self {
            piece_type,
            color,
            has_moved: false,
        }
    }

    pub fn fen_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::King, PieceColor::White) => 'K',
            (PieceType::Queen, PieceColor::White) => 'Q',
            (PieceType::Rook, PieceColor::White) => 'R',
            (PieceType::Bishop, PieceColor::White) => 'B',
            (PieceType::Knight, PieceColor::White) => 'N',
            (PieceType::Pawn, PieceColor::White) => 'P',
            (PieceType::King, PieceColor::Black) => 'k',
            (PieceType::Queen, PieceColor::Black) => 'q',
            (PieceType::Rook, PieceColor::Black) => 'r',
            (PieceType::Bishop, PieceColor::Black) => 'b',
            (PieceType::Knight, PieceColor::Black) => 'n',
            (PieceType::Pawn, PieceColor::Black) => 'p',
        }
    }

    pub fn to_char(&self) -> char {
        let c = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };

        if self.color == PieceColor::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }

}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
