mod board;
mod piece;
mod position;
mod game_state;

pub use board::Board;
pub use piece::{Piece, PieceColor, PieceType};
pub use position::Position;
pub use game_state::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<PieceType>,
}

impl ChessMove {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    pub fn with_promotion(from: Position, to: Position, promotion: PieceType) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }

    pub fn from_uci(uci: &str) -> Option<Self> {
        if uci.len() < 4 || uci.len() > 5 {
            return None;
        }
        
        let from = Position::from_uci(&uci[0..2])?;
        let to = Position::from_uci(&uci[2..4])?;
        let promotion = uci.chars().nth(4).and_then(|c| match c {
            'q' => Some(PieceType::Queen),
            'r' => Some(PieceType::Rook),
            'b' => Some(PieceType::Bishop),
            'n' => Some(PieceType::Knight),
            _ => None,
        });

        Some(match promotion {
            Some(p) => Self::with_promotion(from, to, p),
            None => Self::new(from, to),
        })
    }
}
