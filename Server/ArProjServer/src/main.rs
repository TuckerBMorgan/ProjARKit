#![allow(dead_code)]

use std::fmt;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    White,
    Black
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize
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
                    PieceType::Rook => "♖",
                    PieceType::Bishop => "♗",
                    PieceType::Knight => "♘",
                    _ => "♙",
                }
            } else {
                match self.piece_type {
                    PieceType::King => "♚",
                    PieceType::Queen => "♛",
                    PieceType::Rook => "♜",
                    PieceType::Bishop => "♝",
                    PieceType::Knight => "♞",
                    _ => "♟️",
            }
        };
        write!(f, "{}", piece_rep)
    }
}

struct Board {
    grid: [[Option<Piece>; 8]; 8],
    rows: usize,
    cols: usize,
    last_move: (Option<Piece>, Option<Coord>)
}

impl Board {
    fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        let piece = self.grid[from_row][from_col];
        if !piece.is_none() {
            let mut piece = piece.unwrap();
            let moves = self.possible_moves(&piece);
            let move_coord = Coord { row:to_row, col:to_col };

            // if the chosen move is in the generated possible moves
            if moves.contains(&move_coord) {
                piece.has_moved = true;
                piece.row = to_row;
                piece.col = to_col;

                if piece.color == Color::White && from_row == 3 || piece.color == Color::Black && from_row == 4 {
                    match piece.piece_type {
                        PieceType::Pawn => {
                            let last_piece = self.last_move.0;
                            last_piece.map(|last_piece|
                                if last_piece.piece_type == PieceType::Pawn {
                                    if last_piece.col != from_col {
                                        self.grid[last_piece.row][last_piece.col] = None;
                                    }
                                }
                            );
                        },
                        _ => ()
                    }
                }
                
                self.last_move = (Some(piece), Some(Coord { row: from_row, col: from_col }));
                self.grid[to_row][to_col] = Some(piece); 
                self.grid[from_row][from_col] = None
            } else {
                println!("[Error] Attempt to move piece to invalid square");
            }
        } else {
            println!("[Error] Attempt to move an empty piece");                
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

    fn print_moves(&self, moves: &HashSet<Coord>) {
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
   
    fn possible_moves(&self, piece: &Piece) -> HashSet<Coord> {
        match piece.piece_type {
            PieceType::Knight => self.possible_knight_moves(piece),
            PieceType::Bishop => self.possible_bishop_moves(piece),
            PieceType::Queen => self.possible_queen_moves(piece),
            PieceType::Pawn => self.possible_pawn_moves(piece),
            PieceType::King => self.possible_king_moves(piece),
            _ => HashSet::new()
        }
    }

    fn possible_pawn_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        let mut row = piece.row;
        let mut col = piece.col;

        if piece.color == Color::White {
            row -= 1;
        } else {
            row += 1;
        }

        if self.valid_move(piece, row, col) {
            if self.grid[row][col].is_none() {
                moves.insert(Coord { row, col });
            }
        }

        if !piece.has_moved {
            if piece.color == Color::White {
                row -= 1;
            } else {
                row += 1;
            }
            
            if self.grid[row][col].is_none() {
                moves.insert(Coord { row, col });
            }
        }

        row = piece.row;
        col = piece.col + 1;

        if piece.color == Color::White {
            row -= 1;
        } else {
            row += 1;
        }

        if self.valid_move(piece, row, col) {
            if !self.grid[row][col].is_none() {
                moves.insert(Coord { row, col} );
            } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
                let last_piece = self.last_move.0;

                last_piece.map(|last_piece| {
                    if last_piece.row == piece.row && last_piece.col == col {
                        moves.insert(Coord { row, col });
                    }
                });
            }
        }

        col = piece.col;

        if col > 0 {
            col = piece.col - 1;
        }
        
        if self.valid_move(piece, row, col) {
            if !self.grid[row][col].is_none() {
                moves.insert(Coord { row, col} );
            } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
                let last_piece = self.last_move.0;

                last_piece.map(|last_piece| {
                    if last_piece.row == piece.row && last_piece.col == col {
                        moves.insert(Coord { row, col });
                    }
                });
            }
        }

        return moves
    }

    fn possible_queen_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = self.possible_bishop_moves(piece);

        moves.extend(&self.possible_rook_moves(piece));

        return moves;
    }

    fn possible_rook_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        // up
        let mut row = piece.row - 1;
        let mut col = piece.col;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !self.grid[row][col].is_none() {
                break;
            }

            row -= 1;
        }

        // down
        row = piece.row + 1;
        col = piece.col;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !self.grid[row][col].is_none() {
                break;
            }

            row += 1;
        }

        // left
        row = piece.row;
        col = piece.col - 1;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !self.grid[row][col].is_none() {
                break;
            }

            col -= 1;
        }

        // right
        row = piece.row;
        col = piece.col + 1;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !self.grid[row][col].is_none() {
                break;
            }

            col += 1;
        }

        return moves;
    }

    fn possible_bishop_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        //down positive diagonal
        let mut row = piece.row + 1;
        let mut col = piece.col + 1;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

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
            moves.insert(Coord { row, col });
            
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
            moves.insert(Coord { row, col });
            
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
            moves.insert(Coord { row, col });

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

    fn possible_knight_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        // top top left
        if piece.row >= 2 && piece.col >= 1{
            let row = piece.row - 2;
            let col = piece.col - 1;

            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        // top top right
        if piece.row >= 2 {
            let row = piece.row - 2;
            let col = piece.col + 1;
            
            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        // top right
        if piece.row >= 1 {
            let row = piece.row - 1;
            let col = piece.col + 2;
            
            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        // top left
        if piece.row >= 1 && piece.col >= 2 {
            let row = piece.row - 1;
            let col = piece.col - 2;

            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        // bot left
        if piece.col >= 2 {
            let row = piece.row + 1;
            let col = piece.col - 2;

            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        // bot right
        let row = piece.row + 1;
        let col = piece.col + 2;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
        }

        // bot bot right
        let row = piece.row + 2;
        let col = piece.col + 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
        }

        // bot bot left
        if piece.col >= 1 {
            let row = piece.row + 2;
            let col = piece.col - 1;

            if self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
            }
        }

        return moves;
    }

    fn possible_king_moves(&self, piece: &Piece) -> HashSet<Coord> {
         let mut moves = HashSet::new();

        // up
        let mut row = piece.row - 1;
        let mut col = piece.col;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

        

        }

        // down
        row = piece.row + 1;
        col = piece.col;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

        }

        // left
        row = piece.row;
        col = piece.col - 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
        }

        // right
        row = piece.row;
        col = piece.col + 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

        }

        //down positive diagonal
        row = piece.row + 1;
        col = piece.col + 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });


        }

        //up positive diagonal
        row = piece.row + 1;
        col = piece.col - 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
           
        }
        
        //up negative diagonal
        row = piece.row - 1;
        col = piece.col - 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
            
        }
        
        //down negative diagonal
        row = piece.row - 1;
        col = piece.col + 1;

        if self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

        }

        return moves;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
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

    // loop for input
    use std::io::{stdin};
    let mut s = String::new();
    println!("m -> Move\ns -> Show Moves\np -> Print board\ne -> exit");
    loop {
        s.clear();
        stdin().read_line(&mut s).expect("wtf");
        let input = s.trim_right();
        match input.chars().next().unwrap() {
            'm' => {
                println!("Enter values: (format => row1 col1 row2 col2)");
                let mut x = String::new();

                stdin().read_line(&mut x).expect("wtf");

                let new_input = x.trim_right();
                let vals: Vec<&str> = new_input.split(" ").collect();

                let row1 = vals[0].parse::<usize>().unwrap();
                let col1 = vals[1].parse::<usize>().unwrap();
                let row2 = vals[2].parse::<usize>().unwrap();
                let col2 = vals[3].parse::<usize>().unwrap();

                println!("Trying to move ({}, {}) to ({}, {}):", vals[0], vals[1], vals[2], vals[3]);
                b.move_piece(row1, col1, row2, col2);
            },
            's' => { 
                println!("Enter values: (format => row col)");
                let mut x = String::new();
                stdin().read_line(&mut x).expect("wtf");

                let new_input = x.trim_right();
                let vals: Vec<&str> = new_input.split(" ").collect();

                let row = vals[0].parse::<usize>().unwrap();
                let col = vals[1].parse::<usize>().unwrap();

                println!("Generating possible moves for ({}, {})", row, col);

                b.print_moves(&b.possible_moves(&b.grid[row][col].unwrap()));
            },
            'p' => { println!("{}", b); continue },
            'e' => break,
            _ => { println!("m -> Move\ns -> Show Moves\np -> Print board\ne -> exit"); continue }
        } 
    }
}
