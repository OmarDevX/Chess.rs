#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
}
