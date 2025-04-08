use eframe::{egui, Frame};
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2, FontId, Align2};
use strum::IntoEnumIterator;
use crate::chess::{Board, ChessMove, GameState, Piece, PieceColor, PieceType, Position};
use crate::game_mode::{Difficulty, GameMode, PlayerColor};
use crate::stockfish::Stockfish;

pub struct ChessApp {
    board: Board,
    selected_position: Option<Position>,
    possible_moves: Vec<ChessMove>,
    board_flipped: bool,
    game_state: GameState,
    game_mode: Option<GameMode>,
    difficulty: Difficulty,
    player_color: PlayerColor,
    stockfish: Option<Stockfish>,
    is_thinking: bool,
}

impl ChessApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: Board::new(),
            selected_position: None,
            possible_moves: Vec::new(),
            board_flipped: true,
            game_state: GameState::InProgress,
            game_mode: None,
            difficulty: Difficulty::Medium,
            player_color: PlayerColor::White,
            stockfish: None,
            is_thinking: false,
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
                let visual_rank = if !self.board_flipped { 7 - rank } else { rank };
                let visual_file = if !self.board_flipped { 7 - file } else { file };

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

                // Highlight king in check
                if matches!(self.game_state, GameState::Check | GameState::Checkmate) {
                    if let Some(king_pos) = self.board.get_king_position(self.board.current_turn()) {
                        if king_pos.rank == rank && king_pos.file == file {
                            painter.rect_filled(rect, 0.0, Color32::from_rgba_premultiplied(255, 0, 0, 60));
                        }
                    }
                }

                // Highlight selected square
                if let Some(selected) = self.selected_position {
                    if selected.rank == rank && selected.file == file {
                        painter.rect_stroke(rect, 0.0, Stroke::new(2.0, Color32::YELLOW));
                    }
                }

                // Highlight possible moves
                for mv in &self.possible_moves {
                    if mv.to.rank == rank && mv.to.file == file {
                        painter.circle_filled(rect.center(), square_size / 6.0, Color32::from_rgba_premultiplied(100, 100, 100, 100));
                    }
                }

                // Draw pieces
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

                let actual_rank = if !self.board_flipped { 7 - rank } else { rank };
                let actual_file = if !self.board_flipped { 7 - file } else { file };

                self.handle_square_click(Position::new(actual_rank, actual_file));
            }
        }
    }

    fn draw_piece(&self, painter: &egui::Painter, rect: Rect, piece: Piece) {
        let text = match piece.piece_type {
            PieceType::King => "♚",
            PieceType::Queen => "♛",
            PieceType::Rook => "♜",
            PieceType::Bishop => "♝",
            PieceType::Knight => "♞",
            PieceType::Pawn => "♟",
        };

        let font_size = rect.height() * 0.8;
        let font = FontId::monospace(font_size);
        
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            text,
            font,
            if piece.color == PieceColor::White {
                Color32::WHITE
            } else {
                Color32::BLACK
            },
        );
    }

    fn handle_square_click(&mut self, pos: Position) {
        if self.game_state == GameState::Checkmate {
            return;
        }

        if let Some(selected_pos) = self.selected_position {
            if let Some(mv) = self.possible_moves.iter()
                .find(|m| m.from == selected_pos && m.to == pos) 
            {
                self.board.make_move(*mv);
                self.game_state = self.board.check_game_state();
            }
            self.selected_position = None;
            self.possible_moves.clear();
        } else if let Some(piece) = self.board.get_piece(pos) {
            if piece.color == self.board.current_turn() {
                self.selected_position = Some(pos);
                self.possible_moves = self.board.get_valid_moves(pos);
            }
        }
    }

    fn draw_game_status(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Current turn: {}", self.board.current_turn()));
                
                match self.game_state {
                    GameState::Check => { ui.label("Check!"); }
                    GameState::Checkmate => { ui.label("Checkmate! Click to restart"); }
                    GameState::Stalemate => { ui.label("Stalemate!"); }
                    GameState::InProgress => {}
                }
            });
            
            ui.label(format!("FEN: {}", self.board.to_fen()));
        });
    }

    fn reset_game(&mut self) {
        self.board = Board::new();
        self.board_flipped = true;
        self.game_state = GameState::InProgress;
        self.selected_position = None;
        self.possible_moves.clear();
        self.is_thinking = false;
    }
    
    fn show_difficulty_selection(&mut self, ui: &mut egui::Ui) {
        ui.label("Select Difficulty:");
        for difficulty in Difficulty::iter() {
            if ui.radio(self.difficulty == difficulty, format!("{:?}", difficulty)).clicked() {
                self.difficulty = difficulty;
            }
        }
    }
    
    fn show_color_selection(&mut self, ui: &mut egui::Ui) {
        ui.label("Select Your Color:");
        for color in PlayerColor::iter() {
            if ui.radio(self.player_color == color, format!("{:?}", color)).clicked() {
                self.player_color = color;
            }
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(game_mode) = self.game_mode {
                // Show configuration if in Stockfish mode but not initialized
                if game_mode == GameMode::VsStockfish && self.stockfish.is_none() {
                    ui.vertical_centered(|ui| {
                        ui.heading("Configure Computer Opponent");
                        ui.separator();
                        self.show_difficulty_selection(ui);
                        self.show_color_selection(ui);
                        
                        if ui.button("Start Game").clicked() {
                            self.board_flipped = self.player_color == PlayerColor::Black;
                            self.stockfish = Some(Stockfish::new("./src/chess/stockfish/stockfish-ubuntu-x86-64-avx2"));
                        }
                    });
                } else {
                    // Game in progress
                    self.draw_game_status(ui);
                    ui.separator();
                    self.draw_board(ui);
                
                    if self.game_state == GameState::Checkmate && ui.button("New Game").clicked() {
                        self.reset_game();
                    }
                }
                
                // Handle AI move
                if let Some(stockfish) = &mut self.stockfish {
                    if game_mode == GameMode::VsStockfish 
                        && self.board.current_turn() != self.player_color.to_piece_color()
                        && self.game_state == GameState::InProgress
                        && !self.is_thinking {
                        self.is_thinking = true;
                        let board = self.board.clone();
                        stockfish.set_skill_level(match self.difficulty {
                            Difficulty::Easy => 5,
                            Difficulty::Medium => 15,
                            Difficulty::Hard => 20,
                        });
                        stockfish.set_position(&board.to_fen());
                        
                        if let Some(mv) = stockfish.get_best_move(1000) {
                            self.board.make_move(ChessMove::from_uci(&mv).unwrap());
                            self.game_state = self.board.check_game_state();
                        }
                        self.is_thinking = false;
                    }
                }
            } else {
                // Game mode selection
                ui.vertical_centered(|ui| {
                    ui.heading("Select Game Mode");
                    ui.add_space(20.0);
                    
                    egui::Grid::new("game_mode_grid")
                        .spacing([40.0, 20.0])
                        .show(ui, |ui| {
                            if ui.button("Two Players (Local)").on_hover_text("Play against another person on this device").clicked() {
                                self.game_mode = Some(GameMode::TwoPlayer);
                                self.board_flipped=false;
                            }
                            
                            if ui.button("Play vs Stockfish").on_hover_text("Challenge the computer opponent").clicked() {
                                self.game_mode = Some(GameMode::VsStockfish);
                                // Reset stockfish instance when selecting mode
                                self.stockfish = None;
                            }
                            ui.end_row();
                        });
                });
            }
        });
    }
}

async fn get_stockfish_move(board: Board, difficulty: Difficulty) -> String {
    let mut stockfish = Stockfish::new("./src/chess/stockfish/stockfish-ubuntu-x86-64-avx2");
    stockfish.set_skill_level(match difficulty {
        Difficulty::Easy => 5,
        Difficulty::Medium => 15,
        Difficulty::Hard => 20,
    });
    stockfish.set_position(&board.to_fen());
    stockfish.get_best_move(1000).unwrap_or_default()
}
