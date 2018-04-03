use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::Coord;

pub fn possible_bishop_moves(game_state: &GameState, piece: &Piece) -> HashSet<Coord> {
    let mut moves = HashSet::new();

    //down positive diagonal
    let mut row = piece.row + 1;
    let mut col = piece.col + 1;

    while game_state.valid_move(piece, row, col) {
        moves.insert(Coord { row, col });

        if !game_state.grid[row][col].is_none() {
            break;
        }

        row += 1;
        col += 1;
    }

    if piece.col > 0 {
        //up positive diagonal
        row = piece.row + 1;
        col = piece.col - 1;

        while game_state.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
            
            if !game_state.grid[row][col].is_none() {
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

        while game_state.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });
            
            if !game_state.grid[row][col].is_none() {
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

        while game_state.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !game_state.grid[row][col].is_none() {
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
