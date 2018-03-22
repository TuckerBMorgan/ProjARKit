#![allow(dead_code)]

use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    King,
    Queen,
    LRook,
    RRook,
    LBishop,
    RBishop,
    LKnight,
    RKnight
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    White,
    Black
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    has_moved: bool,
    row: usize,
    col: usize,
    color: Color
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_rep = if self.color == Color::White {
                match self.piece_type {
                    PieceType::King => "♔",
                    PieceType::Queen => "♕",
                    PieceType::LRook | PieceType::RRook => "♖",
                    PieceType::LBishop | PieceType::RBishop => "♗",
                    PieceType::LKnight | PieceType::RKnight => "♘",
                    _ => "♙",
                }
            } else {
                match self.piece_type {
                    PieceType::King => "♚",
                    PieceType::Queen => "♛",
                    PieceType::LRook | PieceType::RRook => "♜",
                    PieceType::LBishop | PieceType::RBishop => "♝",
                    PieceType::LKnight | PieceType::RKnight => "♞",
                    _ => "♟️",
            }
        };
        write!(f, "{}", piece_rep)
    }
}

struct Board {
    grid: [[Option<Piece>; 8]; 8],
    rows: usize,
    cols: usize
}

impl Board {
    fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        let piece = self.grid[from_row][from_col];
        if !piece.is_none() {
            if self.valid_move(&piece.unwrap(), to_row, to_col) {
                piece.unwrap().has_moved = true;
                self.grid[to_row][to_col] = piece; 
                self.grid[from_row][from_col] = None
            } else {
                println!("Invalid move");                
            }
        }
   } 

    fn valid_move(&self, piece: &Piece, row: usize, col: usize) -> bool {
        match piece.piece_type {
            PieceType::King => self.valid_move_king(piece, row, col),
            _ => true
        }
    }

    fn valid_move_king(&self, piece: &Piece, row: usize, col: usize) -> bool {
        if (piece.row as i32 - row as i32).abs() > 1 || (piece.col as i32 - col as i32).abs() > 1 {
            return false;
        }

        let target_piece = self.grid[row][col];
        if !target_piece.is_none() {
            if piece.color == target_piece.unwrap().color {
                return false;
            }
        }

        return true;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            grid: [
                [
                    Some(Piece { piece_type: PieceType::RRook, has_moved: false, row: 0, col: 0, color: Color::Black}), 
                    Some(Piece { piece_type: PieceType::RKnight, has_moved: false, row: 0, col: 1, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::RBishop, has_moved: false, row: 0, col: 2, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::Queen, has_moved: false, row: 0, col: 3, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::King, has_moved: false, row: 0, col: 4, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::LBishop, has_moved: false, row: 0, col: 5, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::LKnight, has_moved: false, row: 0, col: 6, color: Color::Black}),
                    Some(Piece { piece_type: PieceType::LRook, has_moved: false, row: 0, col: 7, color: Color::Black})
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
                    Some(Piece { piece_type: PieceType::LRook, has_moved: false, row: 7, col: 0, color: Color::White}), 
                    Some(Piece { piece_type: PieceType::LKnight, has_moved: false, row: 7, col: 1, color: Color::White}),
                    Some(Piece { piece_type: PieceType::LBishop, has_moved: false, row: 7, col: 2, color: Color::White}),
                    Some(Piece { piece_type: PieceType::Queen, has_moved: false, row: 7, col: 3, color: Color::White}),
                    Some(Piece { piece_type: PieceType::King, has_moved: false, row: 7, col: 4, color: Color::White}),
                    Some(Piece { piece_type: PieceType::RBishop, has_moved: false, row: 7, col: 5, color: Color::White}),
                    Some(Piece { piece_type: PieceType::RKnight, has_moved: false, row: 7, col: 6, color: Color::White}),
                    Some(Piece { piece_type: PieceType::RRook, has_moved: false, row: 7, col: 7, color: Color::White})
                ],
            ],
            rows: 8,
            cols: 8,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_rep = String::new();
        for row in 0..self.rows {
            for col in  0..self.cols {
                let curr_piece = &self.grid[row][col];
                if curr_piece.is_none() {
                    board_rep.push_str(" ");
                } else {
                    board_rep.push_str(&format!("{}", curr_piece.as_ref().unwrap()));
                }
            }
            board_rep.push('\n');
        } 
        board_rep.pop();
        write!(f, "{}", board_rep)
    }
}


fn main() {
    let mut b = Board{..Default::default()};
    println!("{}", b);
    println!("\nTrying to move pawn at (6,0) to (5,0)");
    b.move_piece(6, 0, 5, 0);
    println!("\n{}", b);
    println!("\nTrying to move king at (7,4) to (6,4)");
    b.move_piece(7, 4, 6, 4);
    println!("\n{}", b);
    println!("\nTrying to move pawn at (6,4) to (5,4)");
    b.move_piece(6, 4, 5, 4);
    println!("\n{}", b);
    println!("\nTrying to move king at (7,4) to (6,4)");
    b.move_piece(7, 4, 6, 4);
    println!("\n{}", b);
}
