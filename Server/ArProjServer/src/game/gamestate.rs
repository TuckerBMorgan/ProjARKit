use std::fmt;
use std::collections::HashSet;

use game::piece::Piece;
use game::piece::Color;
use game::piece::Coord;
use game::piece::PieceType;
use game::king::possible_king_moves;
use game::king::in_check;
use game::pawn::possible_pawn_moves;
use game::rook::possible_rook_moves;
use game::bishop::possible_bishop_moves;
use game::knight::possible_knight_moves;
use game::queen::possible_queen_moves;

pub struct GameState {
    pub grid: [[Option<Piece>; 8]; 8],
    pub rows: usize,
    pub cols: usize,
    pub last_move: (Option<Piece>, Option<Coord>)
}

impl GameState {
    pub fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        let piece = self.grid[from_row][from_col];
        match piece {
            Some(mut piece) => {
                let moves = self.possible_moves(&piece);
                let move_coord = Coord { row:to_row, col:to_col };

                // if the chosen move is in the generated possible moves
                if moves.contains(&move_coord) {
                    piece.has_moved = true;
                    piece.row = to_row;
                    piece.col = to_col;

                    if piece.color == Color::White && from_row == 3 
                    || piece.color == Color::Black && from_row == 4 {
                        match piece.piece_type {
                            PieceType::Pawn => {
                                let last_piece = self.last_move.0;
                                last_piece.map(|last_piece|
                                    if last_piece.piece_type == PieceType::Pawn && last_piece.col != from_col {
                                        self.grid[last_piece.row][last_piece.col] = None;
                                    }
                                );
                            },
                            _ => ()
                        }
                    } else if piece.piece_type == PieceType::Pawn 
                    && to_row == (if piece.color == Color::White {0} else {self.rows - 1}) {
                        // TODO: handle promotion properly
                        // for the moment, just promote to queen
                        piece.piece_type = PieceType::Queen;
                    }
                    
                    self.last_move = (Some(piece), Some(Coord { row: from_row, col: from_col }));
                    self.grid[to_row][to_col] = Some(piece); 
                    self.grid[from_row][from_col] = None
                } else {
                    println!("[Error] Attempt to move piece to invalid square")
                }
            },
            None => println!("[Error] Attempt to move an invalid piece")
        }
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn valid_move(&self, piece: &Piece, row: usize, col: usize) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        } else if piece.piece_type == PieceType::King && in_check(&self, Coord {row, col}, piece) {
            return false;
        }

        let target_piece = self.grid[row][col];
        match target_piece {
            Some(target_piece) => !(piece.color == target_piece.color || piece.row == row && piece.col == col),
            None => true
        }
    }

    pub fn print_moves(&self, moves: &HashSet<Coord>) {
       let mut output = String::new();

       for row in 0..8 {
           for col in 0..8 {
               let coord = Coord { row, col };
               output.push_str(&format!("{} ", 
                    if moves.contains(&coord) {
                        "1"
                    } else {
                        "0"
                    }));
           }
           output.push('\n');
       }
       for coord in moves.iter() {
           output.push_str(&format!("({}, {}), ", coord.row, coord.col));
       }
       output.pop();
       output.pop();

       println!("{}", output);
    }
   
    pub fn possible_moves(&self, piece: &Piece) -> HashSet<Coord> {
        match piece.piece_type {
            PieceType::Knight =>    possible_knight_moves(&self, piece),
            PieceType::Bishop =>    possible_bishop_moves(&self, piece),
            PieceType::Queen =>     possible_queen_moves(&self, piece),
            PieceType::Pawn =>      possible_pawn_moves(&self, piece),
            PieceType::Rook =>      possible_rook_moves(&self, piece),
            PieceType::King =>      possible_king_moves(&self, piece),
        }
    }

    pub fn insert_if_valid(&self, piece: &Piece, row: usize, col: usize, moves: &mut HashSet<Coord>) {
        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
        }
    }
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            grid: [
                [
                    Some(Piece { piece_type: PieceType::Rook, has_moved: false, row: 0, col: 0, color: Color::Black}), 
                    Some(Piece { piece_type: PieceType::Knight, has_moved: false, row: 0, col: 1, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Bishop, has_moved: false, row: 0, col: 2, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Queen, has_moved: false, row: 0, col: 3, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::King, has_moved: false, row: 0, col: 4, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Bishop, has_moved: false, row: 0, col: 5, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Knight, has_moved: false, row: 0, col: 6, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Rook, has_moved: false, row: 0, col: 7, color: Color::Black})
                ],
                [
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 0, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 1, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 2, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 3, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 4, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 5, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 6, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 1, col: 7, color: Color::Black}),
                ],
                [
                    None, None, None, None, None, None, None, None
                ],
                [
                    None, None, None, None, None, None, None, None
                ],
                [
                    None, None, None, None, None, None, None, None
                ],
                [
                    None, None, None, None, None, None, None, None
                ],
                [
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 0, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 1, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 2, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 3, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 4, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 5, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 6, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Pawn, has_moved: false, row: 6, col: 7, color: Color::White}),
                ],
                [
                    Some(Piece { piece_type: PieceType::Rook, has_moved: false, row: 7, col: 0, color: Color::White}), 
                    Some(Piece { piece_type: PieceType::Knight, has_moved: false, row: 7, col: 1, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Bishop, has_moved: false, row: 7, col: 2, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Queen, has_moved: false, row: 7, col: 3, color: Color::White}),
                    Some(Piece { piece_type: PieceType::King, has_moved: false, row: 7, col: 4, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Bishop, has_moved: false, row: 7, col: 5, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Knight, has_moved: false, row: 7, col: 6, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Rook, has_moved: false, row: 7, col: 7, color: Color::White})
                ],
            ],
            rows: 8,
            cols: 8,
            last_move: (None, None)
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_rep = String::new();
        for row in 0..self.rows {
            for col in  0..self.cols {
                let curr_piece = &self.grid[row][col];
                match curr_piece {
                    Some(piece) => board_rep.push_str(&format!("{}", piece)),
                    None => board_rep.push_str(" ")
                }
            }
            board_rep.push('\n');
        } 
        board_rep.pop();
        write!(f, "{}", board_rep)
    }
}