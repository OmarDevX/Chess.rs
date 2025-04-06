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
}
