use eframe::{egui, Frame};
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

use crate::chess::{Board, ChessMove, GameState, Piece, PieceColor, PieceType, Position};

pub struct ChessApp {
    board: Board,
    selected_position: Option<Position>,
    possible_moves: Vec<ChessMove>,
    board_flipped: bool,
    game_state: GameState,
}

impl ChessApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: Board::new(),
            selected_position: None,
            possible_moves: Vec::new(),
            board_flipped: false,
            game_state: GameState::InProgress,
        }
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let board_size = available_size.x.min(available_size.y);
        let square_size = board_size / 8.0;

        let board_rect = Rect::from_min_size(
            Pos2::new(
                (available_size.x - board_size) / 2.0,
                (available_size.y - board_size) / 2.0,
            ),
            Vec2::new(board_size, board_size),
        );

        let painter = ui.painter();

        // Draw board squares
        for rank in 0..8 {
            for file in 0..8 {
                let visual_rank = if self.board_flipped { 7 - rank } else { rank };
                let visual_file = if self.board_flipped { 7 - file } else { file };

                let color = if (rank + file) % 2 == 0 {
                    Color32::from_rgb(240, 217, 181)
                } else {
                    Color32::from_rgb(181, 136, 99)
                };

                let pos = Pos2::new(
                    board_rect.left() + visual_file as f32 * square_size,
                    board_rect.top() + visual_rank as f32 * square_size,
                );
                
                let rect = Rect::from_min_size(pos, Vec2::new(square_size, square_size));
                painter.rect_filled(rect, 0.0, color);

                // Highlight selected square
                if let Some(selected) = self.selected_position {
                    if selected.rank == rank && selected.file == file {
                        painter.rect_stroke(rect, 0.0, Stroke::new(2.0, Color32::YELLOW));
                    }
                }

                // Highlight possible moves
                if self.selected_position.is_some() {
                    for chess_move in &self.possible_moves {
                        let target_rank = if self.board_flipped { 7 - chess_move.to.rank } else { chess_move.to.rank };
                        let target_file = if self.board_flipped { 7 - chess_move.to.file } else { chess_move.to.file };
                        
                        if visual_rank == target_rank && visual_file == target_file {
                            painter.circle_filled(
                                rect.center(),
                                square_size / 6.0,
                                Color32::from_rgba_premultiplied(100, 100, 100, 100),
                            );
                        }
                    }
                }

                // Draw piece
                if let Some(piece) = self.board.get_piece(Position::new(rank, file)) {
                    self.draw_piece(painter, rect, piece);
                }
            }
        }

        // Handle clicks
        let response = ui.allocate_rect(board_rect, Sense::click());
        if response.clicked() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let file = ((mouse_pos.x - board_rect.left()) / square_size) as usize;
                let rank = ((mouse_pos.y - board_rect.top()) / square_size) as usize;
                
                let actual_rank = if self.board_flipped { 7 - rank } else { rank };
                let actual_file = if self.board_flipped { 7 - file } else { file };
                
                self.handle_square_click(Position::new(actual_rank, actual_file));
            }
        }
    }

    fn draw_piece(&self, painter: &egui::Painter, rect: Rect, piece: Piece) {
        let color = match piece.color {
            PieceColor::White => Color32::WHITE,
            PieceColor::Black => Color32::from_rgb(50, 50, 50),
        };
        let size = rect.width() * 0.6;
        let center = rect.center();

        match piece.piece_type {
            PieceType::Pawn => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♟", egui::FontId::monospace(size), color);
            }
            PieceType::Rook => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♜", egui::FontId::monospace(size), color);
            }
            PieceType::Knight => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♞", egui::FontId::monospace(size), color);
            }
            PieceType::Bishop => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♝", egui::FontId::monospace(size), color);
            }
            PieceType::Queen => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♛", egui::FontId::monospace(size), color);
            }
            PieceType::King => {
                painter.text(center, egui::Align2::CENTER_CENTER, "♚", egui::FontId::monospace(size), color);
            }
        }
    }

    fn handle_square_click(&mut self, pos: Position) {
        if let Some(selected) = self.selected_position {
            if let Some(mv) = self.possible_moves.iter().find(|m| m.to == pos) {
                self.board.make_move(*mv);
                self.board_flipped = !self.board_flipped;
                self.selected_position = None;
                self.possible_moves.clear();
                self.game_state = self.board.check_game_state();
            } else {
                self.selected_position = None;
                self.possible_moves.clear();
            }
        } else if let Some(piece) = self.board.get_piece(pos) {
            if piece.color == self.board.current_turn() {
                self.selected_position = Some(pos);
                self.possible_moves = self.board.get_valid_moves(pos);
            }
        }
    }

    fn draw_game_status(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Current turn: {}", self.board.current_turn()));
            match self.game_state {
                GameState::Check => { ui.label("Check!"); },
                GameState::Checkmate => { ui.label("Checkmate!"); },
                GameState::Stalemate => { ui.label("Stalemate!"); },
                _ => {}
            }
        });
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_game_status(ui);
            ui.separator();
            self.draw_board(ui);
        });
    }
}
