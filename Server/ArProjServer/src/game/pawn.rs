use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::Color;
use game::piece::Coord;

pub fn possible_pawn_moves(game_state: &GameState, piece: &Piece) -> HashSet<Coord> {
    let mut moves = HashSet::new();

    let mut row = piece.row;
    let mut col = piece.col;

    if piece.color == Color::White {
        row -= 1;
    } else {
        row += 1;
    }

    if game_state.in_bounds(row, col) && game_state.grid[row][col].is_none() {
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }
    
    if !piece.has_moved {
        if piece.color == Color::White {
            row -= 1;
        } else {
            row += 1;
        }
        
        if game_state.in_bounds(row, col) && game_state.grid[row][col].is_none() {
            game_state.insert_if_valid(piece, row, col, &mut moves);
        }
    }

    row = piece.row;
    col = piece.col + 1;

    if piece.color == Color::White {
        row -= 1;
    } else {
        row += 1;
    }

    if game_state.in_bounds(row, col) && !game_state.grid[row][col].is_none() {
        game_state.insert_if_valid(piece, row, col, &mut moves);
    } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
        let last_piece = game_state.last_move.0;

        last_piece.map(|last_piece| {
            if last_piece.row == piece.row && last_piece.col == col {
                game_state.insert_if_valid(piece, row, col, &mut moves);
            }
        });
    }

    col = piece.col;

    if col > 0 {
        col = piece.col - 1;
    }
    
    if game_state.in_bounds(row, col) && !game_state.grid[row][col].is_none() {
        game_state.insert_if_valid(piece, row, col, &mut moves);
    } else if piece.color == Color::White && piece.row == 3 || piece.color == Color::Black && piece.row == 4 {
        let last_piece = game_state.last_move.0;

        last_piece.map(|last_piece| {
            if last_piece.row == piece.row && last_piece.col == col {
                game_state.insert_if_valid(piece, row, col, &mut moves);
            }
        });
    }

    return moves
}

