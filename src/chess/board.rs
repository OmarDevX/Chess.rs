use super::{ChessMove, GameState, Piece, PieceColor, PieceType, Position};

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    turn: PieceColor,
    move_count: usize,
    captured_pieces: Vec<Piece>,
    en_passant_target: Option<Position>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            en_passant_target: None,
            squares: [[None; 8]; 8],
            turn: PieceColor::White,
            move_count: 0,
            captured_pieces: Vec::new(),
        };

        // Initialize the board with the standard chess setup
        board.setup_standard_position();
        board
    }

    pub fn setup_standard_position(&mut self) {
        // Place pawns
        for file in 0..8 {
            self.squares[1][file] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
            self.squares[6][file] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        }

        // Place rooks
        self.squares[0][0] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        self.squares[0][7] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        self.squares[7][0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        self.squares[7][7] = Some(Piece::new(PieceType::Rook, PieceColor::White));

        // Place knights
        self.squares[0][1] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        self.squares[0][6] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        self.squares[7][1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        self.squares[7][6] = Some(Piece::new(PieceType::Knight, PieceColor::White));

        // Place bishops
        self.squares[0][2] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        self.squares[0][5] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        self.squares[7][2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        self.squares[7][5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));

        // Place queens
        self.squares[0][3] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        self.squares[7][3] = Some(Piece::new(PieceType::Queen, PieceColor::White));

        // Place kings
        self.squares[0][4] = Some(Piece::new(PieceType::King, PieceColor::Black));
        self.squares[7][4] = Some(Piece::new(PieceType::King, PieceColor::White));
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        if position.is_valid() {
            self.squares[position.rank][position.file]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, position: Position, piece: Option<Piece>) {
        if position.is_valid() {
            self.squares[position.rank][position.file] = piece;
        }
    }

    pub fn current_turn(&self) -> PieceColor {
        self.turn
    }

    pub fn make_move(&mut self, chess_move: ChessMove) {
        let from = chess_move.from;
        let to = chess_move.to;

        if let Some(mut piece) = self.get_piece(from) {
            // Check if there's a piece to capture
            if let Some(captured_piece) = self.get_piece(to) {
                self.captured_pieces.push(captured_piece);
            }

            // Handle pawn promotion
            if let Some(promotion_type) = chess_move.promotion {
                piece.piece_type = promotion_type;
            }

            // Handle en passant capture
            if piece.piece_type == PieceType::Pawn && Some(to) == self.en_passant_target {
                let direction = if piece.color == PieceColor::White { 1 } else { -1 };
                let captured_pos = Position::new((to.rank as i32 + direction) as usize, to.file);
                if let Some(captured_piece) = self.get_piece(captured_pos) {
                    self.captured_pieces.push(captured_piece);
                    self.set_piece(captured_pos, None);
                }
            }

            // Set en passant target if pawn moved two squares
            self.en_passant_target = None;
            if piece.piece_type == PieceType::Pawn && (from.rank as i32 - to.rank as i32).abs() == 2 {
                let direction = if piece.color == PieceColor::White { -1 } else { 1 };
                self.en_passant_target = Some(Position::new((from.rank as i32 + direction) as usize, from.file));
            }

            // Mark the piece as moved and update position
            piece.has_moved = true;
            self.set_piece(from, None);
            self.set_piece(to, Some(piece));

            // Switch turns
            self.turn = self.turn.opposite();
            self.move_count += 1;
        }
    }

    pub fn get_valid_moves(&self, position: Position) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        if let Some(piece) = self.get_piece(position) {
            // Only allow moves for pieces of the current turn's color
            if piece.color != self.turn {
                return moves;
            }

            match piece.piece_type {
                PieceType::Pawn => self.get_pawn_moves(position, piece, &mut moves),
                PieceType::Knight => self.get_knight_moves(position, piece, &mut moves),
                PieceType::Bishop => self.get_bishop_moves(position, piece, &mut moves),
                PieceType::Rook => self.get_rook_moves(position, piece, &mut moves),
                PieceType::Queen => {
                    self.get_bishop_moves(position, piece, &mut moves);
                    self.get_rook_moves(position, piece, &mut moves);
                }
                PieceType::King => self.get_king_moves(position, piece, &mut moves),
            }

            // Filter out moves that would leave the king in check
            moves.retain(|m| !self.would_be_in_check_after_move(*m, piece.color));
        }

        moves
    }

    fn get_pawn_moves(&self, position: Position, piece: Piece, moves: &mut Vec<ChessMove>) {
        let direction = if piece.color == PieceColor::White { -1 } else { 1 };
        
        // Forward move
        if let Some(forward) = position.offset(direction, 0) {
            if self.get_piece(forward).is_none() {
                // Regular move
                moves.push(ChessMove::new(position, forward));
                
                // Double move from starting position
                let starting_rank = if piece.color == PieceColor::White { 6 } else { 1 };
                if position.rank == starting_rank {
                    if let Some(double_forward) = position.offset(direction * 2, 0) {
                        if self.get_piece(double_forward).is_none() {
                            moves.push(ChessMove::new(position, double_forward));
                        }
                    }
                }
                
                // Promotion
                let promotion_rank = if piece.color == PieceColor::White { 0 } else { 7 };
                if forward.rank == promotion_rank {
                    // Replace the regular move with promotion moves
                    moves.pop(); // Remove the regular move
                    
                    // Add promotion options
                    moves.push(ChessMove::with_promotion(position, forward, PieceType::Queen));
                    moves.push(ChessMove::with_promotion(position, forward, PieceType::Rook));
                    moves.push(ChessMove::with_promotion(position, forward, PieceType::Bishop));
                    moves.push(ChessMove::with_promotion(position, forward, PieceType::Knight));
                }
            }
        }
        
        // Capture moves
        for file_offset in [-1, 1].iter() {
            if let Some(capture_pos) = position.offset(direction, *file_offset) {
                if let Some(target) = self.get_piece(capture_pos) {
                    if target.color != piece.color {
                        // Regular capture
                        let capture_move = ChessMove::new(position, capture_pos);
                        
                        // Check for promotion on capture
                        let promotion_rank = if piece.color == PieceColor::White { 0 } else { 7 };
                        if capture_pos.rank == promotion_rank {
                            // Add promotion options for capture
                            moves.push(ChessMove::with_promotion(position, capture_pos, PieceType::Queen));
                            moves.push(ChessMove::with_promotion(position, capture_pos, PieceType::Rook));
                            moves.push(ChessMove::with_promotion(position, capture_pos, PieceType::Bishop));
                            moves.push(ChessMove::with_promotion(position, capture_pos, PieceType::Knight));
                        } else {
                            moves.push(capture_move);
                        }
                    }
                }
                
                // En passant capture
                if Some(capture_pos) == self.en_passant_target {
                    moves.push(ChessMove::new(position, capture_pos));
                }
            }
        }
    }

    fn get_knight_moves(&self, position: Position, piece: Piece, moves: &mut Vec<ChessMove>) {
        let offsets = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];
        
        for (rank_offset, file_offset) in offsets.iter() {
            if let Some(target_pos) = position.offset(*rank_offset, *file_offset) {
                match self.get_piece(target_pos) {
                    None => {
                        // Empty square, can move there
                        moves.push(ChessMove::new(position, target_pos));
                    }
                    Some(target) if target.color != piece.color => {
                        // Opponent's piece, can capture
                        moves.push(ChessMove::new(position, target_pos));
                    }
                    _ => {} // Own piece, can't move there
                }
            }
        }
    }

    fn get_bishop_moves(&self, position: Position, piece: Piece, moves: &mut Vec<ChessMove>) {
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        
        for (rank_dir, file_dir) in directions.iter() {
            let mut current_pos = position;
            
            loop {
                if let Some(next_pos) = current_pos.offset(*rank_dir, *file_dir) {
                    current_pos = next_pos;
                    
                    match self.get_piece(current_pos) {
                        None => {
                            // Empty square, can move there
                            moves.push(ChessMove::new(position, current_pos));
                        }
                        Some(target) if target.color != piece.color => {
                            // Opponent's piece, can capture and then stop
                            moves.push(ChessMove::new(position, current_pos));
                            break;
                        }
                        _ => {
                            // Own piece, can't move there and stop looking in this direction
                            break;
                        }
                    }
                } else {
                    // Off the board
                    break;
                }
            }
        }
    }

    fn get_rook_moves(&self, position: Position, piece: Piece, moves: &mut Vec<ChessMove>) {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        for (rank_dir, file_dir) in directions.iter() {
            let mut current_pos = position;
            
            loop {
                if let Some(next_pos) = current_pos.offset(*rank_dir, *file_dir) {
                    current_pos = next_pos;
                    
                    match self.get_piece(current_pos) {
                        None => {
                            // Empty square, can move there
                            moves.push(ChessMove::new(position, current_pos));
                        }
                        Some(target) if target.color != piece.color => {
                            // Opponent's piece, can capture and then stop
                            moves.push(ChessMove::new(position, current_pos));
                            break;
                        }
                        _ => {
                            // Own piece, can't move there and stop looking in this direction
                            break;
                        }
                    }
                } else {
                    // Off the board
                    break;
                }
            }
        }
    }

    fn get_king_moves(&self, position: Position, piece: Piece, moves: &mut Vec<ChessMove>) {
        let offsets = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        
        for (rank_offset, file_offset) in offsets.iter() {
            if let Some(target_pos) = position.offset(*rank_offset, *file_offset) {
                match self.get_piece(target_pos) {
                    None => {
                        // Empty square, can move there
                        moves.push(ChessMove::new(position, target_pos));
                    }
                    Some(target) if target.color != piece.color => {
                        // Opponent's piece, can capture
                        moves.push(ChessMove::new(position, target_pos));
                    }
                    _ => {} // Own piece, can't move there
                }
            }
        }
        
        // TODO: Castling (not implemented for simplicity)
    }

    pub fn is_in_check(&self, color: PieceColor) -> bool {
        // Find the king's position
        let mut king_pos = None;
        for rank in 0..8 {
            for file in 0..8 {
                let pos = Position::new(rank, file);
                if let Some(piece) = self.get_piece(pos) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        king_pos = Some(pos);
                        break;
                    }
                }
            }
            if king_pos.is_some() {
                break;
            }
        }
        
        if let Some(king_position) = king_pos {
            // Check if any opponent's piece can attack the king
            for rank in 0..8 {
                for file in 0..8 {
                    let pos = Position::new(rank, file);
                    if let Some(piece) = self.get_piece(pos) {
                        if piece.color != color {
                            // Get all moves for this opponent piece
                            let mut opponent_moves = Vec::new();
                            match piece.piece_type {
                                PieceType::Pawn => self.get_pawn_moves(pos, piece, &mut opponent_moves),
                                PieceType::Knight => self.get_knight_moves(pos, piece, &mut opponent_moves),
                                PieceType::Bishop => self.get_bishop_moves(pos, piece, &mut opponent_moves),
                                PieceType::Rook => self.get_rook_moves(pos, piece, &mut opponent_moves),
                                PieceType::Queen => {
                                    self.get_bishop_moves(pos, piece, &mut opponent_moves);
                                    self.get_rook_moves(pos, piece, &mut opponent_moves);
                                }
                                PieceType::King => self.get_king_moves(pos, piece, &mut opponent_moves),
                            }
                            
                            // Check if any move can capture the king
                            for m in opponent_moves {
                                if m.to == king_position {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        false
    }

    fn would_be_in_check_after_move(&self, chess_move: ChessMove, color: PieceColor) -> bool {
        // Create a temporary board to simulate the move
        let mut temp_board = self.clone();
        temp_board.make_move(chess_move);
        
        // Check if the king is in check after the move
        temp_board.is_in_check(color)
    }

    pub fn get_king_position(&self, color: PieceColor) -> Option<Position> {
        for rank in 0..8 {
            for file in 0..8 {
                let pos = Position::new(rank, file);
                if let Some(piece) = self.get_piece(pos) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some(pos);
                    }
                }
            }
        }
        None
    }

    pub fn check_game_state(&self) -> GameState {
        let current_color = self.turn;
        
        // Check if the current player is in check
        let in_check = self.is_in_check(current_color);
        
        // Check if the current player has any valid moves
        let mut has_valid_moves = false;
        for rank in 0..8 {
            for file in 0..8 {
                let pos = Position::new(rank, file);
                if let Some(piece) = self.get_piece(pos) {
                    if piece.color == current_color {
                        let moves = self.get_valid_moves(pos);
                        if !moves.is_empty() {
                            has_valid_moves = true;
                            break;
                        }
                    }
                }
            }
            if has_valid_moves {
                break;
            }
        }
        
        if !has_valid_moves {
            if in_check {
                GameState::Checkmate
            } else {
                GameState::Stalemate
            }
        } else if in_check {
            GameState::Check
        } else {
            GameState::InProgress
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            squares: self.squares.clone(),
            turn: self.turn,
            move_count: self.move_count,
            captured_pieces: self.captured_pieces.clone(),
            en_passant_target: self.en_passant_target,
        }
    }
}
