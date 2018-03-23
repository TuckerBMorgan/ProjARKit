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
                let mut piece = piece.unwrap();
                piece.has_moved = true;
                piece.row = to_row;
                piece.col = to_col;
                self.grid[to_row][to_col] = Some(piece); 
                self.grid[from_row][from_col] = None
            } else {
                println!("Invalid move");                
            }
        }
   } 

    fn valid_move(&self, piece: &Piece, row: usize, col: usize) -> bool {
        if row > self.rows - 1 || col > self.cols - 1 {
            return false;
        }

        let target_piece = self.grid[row][col];
        if !target_piece.is_none() {
            if piece.color == target_piece.unwrap().color {
                return false;
            }

            if piece.row == row && piece.col == col {
                return false;
            }
        }

        return true;
   }

   fn print_moves(&self, moves: &[[u8; 8]; 8]) {
       let mut output = String::new();

       for row in 0..8 {
           for col in 0..8 {
               output.push_str(&format!("{} ", moves[row][col]));
           }
           output.push('\n');
       }
       output.pop();

       println!("{}", output);
   }

   fn possible_bishop_moves(&self, piece: &Piece) -> [[u8; 8]; 8] {
        let mut moves = [[0; 8]; 8];

        //down positive diagonal
        let mut row = piece.row + 1;
        let mut col = piece.col + 1;

        while self.valid_move(piece, row, col) {
            moves[row][col] = 1;

            if !self.grid[row][col].is_none() {
                break;
            }

            row += 1;
            col += 1;
        }

        //up positive diagonal
        row = piece.row + 1;
        col = piece.col - 1;

        while self.valid_move(piece, row, col) {
            moves[row][col] = 1;
            
            if !self.grid[row][col].is_none() {
                break;
            }

            if col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        
        //up negative diagonal
        row = piece.row - 1;
        col = piece.col - 1;

        while self.valid_move(piece, row, col) {
            moves[row][col] = 1;
            
            if !self.grid[row][col].is_none() {
                break;
            }

            if row == 0 || col == 0 {
                break;
            }

            row -= 1;
            col -= 1;
        }
        
        //down negative diagonal
        row = piece.row - 1;
        col = piece.col + 1;

        while self.valid_move(piece, row, col) {
            moves[row][col] = 1;

            if !self.grid[row][col].is_none() {
                break;
            }

            if row == 0 {
                break;
            }

            row -= 1;
            col += 1;
        }

        return moves;
   }

   fn possible_knight_moves(&self, piece: &Piece) -> [[u8; 8]; 8] {
        let mut moves = [[0; 8]; 8];

        // top top left
        if piece.row >= 2 && piece.col >= 1{
            let row = piece.row - 2;
            let col = piece.col - 1;

            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        // top top right
        if piece.row >= 2 {
            let row = piece.row - 2;
            let col = piece.col + 1;
            
            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        // top right
        if piece.row >= 1 {
            let row = piece.row - 1;
            let col = piece.col + 2;
            
            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        // top left
        if piece.row >= 1 && piece.col >= 2 {
            let row = piece.row - 1;
            let col = piece.col - 2;

            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        // bot left
        if piece.col >= 2 {
            let row = piece.row + 1;
            let col = piece.col - 2;

            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        // bot right
        let row = piece.row + 1;
        let col = piece.col + 2;

        if self.valid_move(piece, row, col) {
            moves[row][col] = 1
        }

        // bot bot right
        let row = piece.row + 2;
        let col = piece.col + 1;

        if self.valid_move(piece, row, col) {
            moves[row][col] = 1
        }

        // bot bot left
        if piece.col >= 1 {
            let row = piece.row + 2;
            let col = piece.col - 1;

            if self.valid_move(piece, row, col) {
                moves[row][col] = 1
            }
        }

        return moves;
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
    println!("\nTrying to move king at (7,4) to (8,4)");
    b.move_piece(7, 4, 8, 4);
    println!("\n{}", b);
    println!("\nTrying to move pawn at (6,4) to (5,4)");
    b.move_piece(6, 4, 5, 4);
    println!("\n{}", b);
    println!("\nTrying to move king at (7,4) to (6,4)");
    b.move_piece(7, 4, 6, 4);
    println!("\n{}", b);
    println!("\nTrying to move king at (6,4) to (7,4)");
    b.move_piece(6, 4, 7, 4);
    println!("\n{}", b);
    println!("\nTrying to move pawn at (6,6) to (5,6)");
    b.move_piece(6, 6, 5, 6);
    println!("\n{}", b);
    println!("\nGenerating all possible moves for bishop at (7,5):");
    b.print_moves(&b.possible_bishop_moves(&b.grid[7][5].unwrap()));
    println!("\nTrying to move bishop at (7,5) to (4,2)");
    b.move_piece(7, 5, 4, 2);
    println!("\n{}", b);
    println!("\nGenerating all possible moves for bishop at (4,2):");
    b.print_moves(&b.possible_bishop_moves(&b.grid[4][2].unwrap()));
    println!("\n{}", b);
    println!("\nGenerating all possible moves for knight at (7,6)");
    b.print_moves(&b.possible_knight_moves(&b.grid[7][6].unwrap()));
    println!("\nTrying to move knight at (7,6) to (5,5)");
    b.move_piece(7, 6, 5, 5);
    println!("\n{}", b);
    println!("\nGenerating all possible moves for knight at (5,5)");
    b.print_moves(&b.possible_knight_moves(&b.grid[5][5].unwrap()));
}
