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

struct GameState {
    grid: [[Option<Piece>; 8]; 8],
    rows: usize,
    cols: usize,
    last_move: (Option<Piece>, Option<Coord>)
}

impl GameState {
    fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
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

                    if piece.color == Color::White && from_row == 3 || piece.color == Color::Black && from_row == 4 {
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
                    } else if piece.piece_type == PieceType::Pawn && to_row == (if piece.color == Color::White {0} else {self.rows - 1}) {
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

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    fn valid_move(&self, piece: &Piece, row: usize, col: usize) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        }

        let target_piece = self.grid[row][col];
        match target_piece {
            Some(target_piece) => !(piece.color == target_piece.color || piece.row == row && piece.col == col),
            None => true
        }
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
            PieceType::Rook => self.possible_rook_moves(piece),
            PieceType::King => self.possible_king_moves(piece),
        }
    }

    fn insert_if_valid(&self, piece: &Piece, row: usize, col: usize, moves: &mut HashSet<Coord>) {
        if self.valid_move(piece, row, col) {
            if piece.piece_type != PieceType::King 
            || piece.piece_type == PieceType::King && self.in_check(Coord {row, col}, piece.color) {
                moves.insert(Coord { row, col });
            }
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

        if self.in_bounds(row, col) && self.grid[row][col].is_none() {
            self.insert_if_valid(piece, row, col, &mut moves);
        }
        
        if !piece.has_moved {
            if piece.color == Color::White {
                row -= 1;
            } else {
                row += 1;
            }
            
            if self.in_bounds(row, col) && self.grid[row][col].is_none() {
                self.insert_if_valid(piece, row, col, &mut moves);
            }
        }

        row = piece.row;
        col = piece.col + 1;

        if piece.color == Color::White {
            row -= 1;
        } else {
            row += 1;
        }

        if self.in_bounds(row, col) && !self.grid[row][col].is_none() {
            self.insert_if_valid(piece, row, col, &mut moves);
        } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
            let last_piece = self.last_move.0;

            last_piece.map(|last_piece| {
                if last_piece.row == piece.row && last_piece.col == col {
                    self.insert_if_valid(piece, row, col, &mut moves);
                }
            });
        }

        col = piece.col;

        if col > 0 {
            col = piece.col - 1;
        }
        
        if self.in_bounds(row, col) && !self.grid[row][col].is_none() {
            self.insert_if_valid(piece, row, col, &mut moves);
        } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
            let last_piece = self.last_move.0;

            last_piece.map(|last_piece| {
                if last_piece.row == piece.row && last_piece.col == col {
                    self.insert_if_valid(piece, row, col, &mut moves);
                }
            });
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

        if piece.row > 0 {
            // up
            let mut row = piece.row - 1;
            let col = piece.col;

            while self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });

                if !self.grid[row][col].is_none() {
                    break;
                }

                if row > 0 {
                    row -= 1;
                } else {
                    break;
                }
            }
        }

        // down
        let mut row = piece.row + 1;
        let mut col = piece.col;

        while self.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !self.grid[row][col].is_none() {
                break;
            }

            row += 1;
        }

        if piece.col > 0 {
            // left
            row = piece.row;
            col = piece.col - 1;

            while self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });

                if !self.grid[row][col].is_none() {
                    break;
                }
                if col > 0 {
                    col -= 1;
                } else {
                    break;
                }
            }
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

        if piece.col > 0 {
            //up positive diagonal
            row = piece.row + 1;
            col = piece.col - 1;

            while self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
                
                if !self.grid[row][col].is_none() {
                    break;
                }

                if col > 0 {
                    row += 1;
                    col -= 1;
                } else {
                    break;
                }
            }
        }

        if piece.row > 0 && piece.col > 0 { 
            //up negative diagonal
            row = piece.row - 1;
            col = piece.col - 1;

            while self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });
                
                if !self.grid[row][col].is_none() {
                    break;
                }
                
                if row > 0 && col > 0 {
                    row -= 1;
                    col -= 1;
                } else {
                    break;
                }
            }
        }

        if piece.row > 0 { 
            //down negative diagonal
            row = piece.row - 1;
            col = piece.col + 1;

            while self.valid_move(piece, row, col) {
                moves.insert(Coord { row, col });

                if !self.grid[row][col].is_none() {
                    break;
                }
                
                if row > 0 {
                    row -= 1;
                    col += 1;
                } else {
                    break;
                }
            }
        }

        return moves;
    }

    fn possible_knight_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        // top top left
        if piece.row >= 2 && piece.col >= 1{
            let row = piece.row - 2;
            let col = piece.col - 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // top top right
        if piece.row >= 2 {
            let row = piece.row - 2;
            let col = piece.col + 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // top right
        if piece.row >= 1 {
            let row = piece.row - 1;
            let col = piece.col + 2;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // top left
        if piece.row >= 1 && piece.col >= 2 {
            let row = piece.row - 1;
            let col = piece.col - 2;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // bot left
        if piece.col >= 2 {
            let row = piece.row + 1;
            let col = piece.col - 2;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // bot right
        let row = piece.row + 1;
        let col = piece.col + 2;
        self.insert_if_valid(piece, row, col, &mut moves);

        // bot bot right
        let row = piece.row + 2;
        let col = piece.col + 1;
        self.insert_if_valid(piece, row, col, &mut moves);

        // bot bot left
        if piece.col >= 1 {
            let row = piece.row + 2;
            let col = piece.col - 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        return moves;
    }

    fn possible_king_moves(&self, piece: &Piece) -> HashSet<Coord> {
        let mut moves = HashSet::new();

        if piece.row > 0 {
            // up
            let row = piece.row - 1;
            let col = piece.col;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // down
        let mut row = piece.row + 1;
        let mut col = piece.col;
        self.insert_if_valid(piece, row, col, &mut moves);

        if piece.col > 0 {
            // left
            row = piece.row;
            col = piece.col - 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        // right
        row = piece.row;
        col = piece.col + 1;
        self.insert_if_valid(piece, row, col, &mut moves);

        //down positive diagonal
        row = piece.row + 1;
        col = piece.col + 1;
        self.insert_if_valid(piece, row, col, &mut moves);

        if piece.col > 0 {
            //up positive diagonal
            row = piece.row + 1;
            col = piece.col - 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        if piece.row > 0 && piece.col > 0 { 
            //up negative diagonal
            row = piece.row - 1;
            col = piece.col - 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        if piece.row > 0 { 
            //down negative diagonal
            row = piece.row - 1;
            col = piece.col + 1;
            self.insert_if_valid(piece, row, col, &mut moves);
        }

        return moves;
    }

    fn in_check(&self, pos: Coord, color: Color) -> bool {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let spot = Coord { row, col };
                if spot != pos {
                    let curr_piece = self.grid[row][col];
                    let mut check = false;
                    curr_piece.map(|curr_piece|
                        if curr_piece.color != color {
                            if self.possible_moves(&curr_piece).contains(&pos) {
                                check = true;
                            }
                        }
                    );
                    if check == true {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

fn gen_piece(piece_type: PieceType, has_moved: bool, row: usize, col: usize, color: Color) -> Option<Piece> {
    Some(Piece {piece_type, has_moved, row, col, color})
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

fn main() {
    let mut b = GameState{..Default::default()};
    
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
                let mut x = String::new(); 
                let mut vals = input.split_whitespace().collect::<Vec<&str>>();

                if vals.len() != 5 {
                    println!("Enter values: (format => row1 col1 row2 col2)");
                    stdin().read_line(&mut x).expect("wtf");
                    vals = x.split_whitespace().collect();
                } else {
                    vals.remove(0); 
                }

                let row1 = vals[0].parse::<usize>().unwrap();
                let col1 = vals[1].parse::<usize>().unwrap();
                let row2 = vals[2].parse::<usize>().unwrap();
                let col2 = vals[3].parse::<usize>().unwrap();

                let piece = b.grid[row1][col1];

                if !piece.is_none() {
                    println!("Trying to move ({}, {}) to ({}, {}):", vals[0], vals[1], vals[2], vals[3]);
                    b.move_piece(row1, col1, row2, col2);
                }
            },
            's' => {
                let mut x = String::new(); 
                let mut vals = input.split_whitespace().collect::<Vec<&str>>();

                if vals.len() != 3 {
                    println!("Enter values: (format => row col)");
                    stdin().read_line(&mut x).expect("wtf");

                    vals = x.split_whitespace().collect();
                } else {
                    vals.remove(0);
                }

                let row = vals[0].parse::<usize>().unwrap();
                let col = vals[1].parse::<usize>().unwrap();

                b.grid[row][col].map(|piece| {
                   println!("Generating possible moves for ({}, {})", row, col);
                   b.print_moves(&b.possible_moves(&piece));
                });
            },
            'p' => { println!("{}", b); continue },
            'e' => break,
            _ => { println!("m -> Move\ns -> Show Moves\np -> Print board\ne -> exit"); continue }
        } 
    }
}