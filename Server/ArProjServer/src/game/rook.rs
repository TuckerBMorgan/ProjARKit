use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::Coord;

pub fn possible_rook_moves(game_state: &GameState, piece: Piece) -> HashSet<Coord> {
    let mut moves = HashSet::new();

    if piece.row > 0 {
        // up
        let mut row = piece.row - 1;
        let col = piece.col;

        while game_state.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !game_state.grid[row][col].is_none() {
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

    while game_state.valid_move(piece, row, col) {
        moves.insert(Coord { row, col });

        if !game_state.grid[row][col].is_none() {
            break;
        }

        row += 1;
    }

    if piece.col > 0 {
        // left
        row = piece.row;
        col = piece.col - 1;

        while game_state.valid_move(piece, row, col) {
            moves.insert(Coord { row, col });

            if !game_state.grid[row][col].is_none() {
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

    while game_state.valid_move(piece, row, col) {
        moves.insert(Coord { row, col });

        if !game_state.grid[row][col].is_none() {
            break;
        }

        col += 1;
    }

    return moves;
}
