use strum::{EnumIter, Display};
use crate::chess::PieceColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum GameMode {
    TwoPlayer,
    VsStockfish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn to_piece_color(self) -> PieceColor {
        match self {
            PlayerColor::White => PieceColor::White,
            PlayerColor::Black => PieceColor::Black,
        }
    }
}
