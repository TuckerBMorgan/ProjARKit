use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::Coord;

pub fn possible_knight_moves(game_state: &GameState, piece: &Piece) -> HashSet<Coord> {
    let mut moves = HashSet::new();

    // top top left
    if piece.row >= 2 && piece.col >= 1{
        let row = piece.row - 2;
        let col = piece.col - 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // top top right
    if piece.row >= 2 {
        let row = piece.row - 2;
        let col = piece.col + 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // top right
    if piece.row >= 1 {
        let row = piece.row - 1;
        let col = piece.col + 2;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // top left
    if piece.row >= 1 && piece.col >= 2 {
        let row = piece.row - 1;
        let col = piece.col - 2;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // bot left
    if piece.col >= 2 {
        let row = piece.row + 1;
        let col = piece.col - 2;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // bot right
    let row = piece.row + 1;
    let col = piece.col + 2;
    game_state.insert_if_valid(piece, row, col, &mut moves);

    // bot bot right
    let row = piece.row + 2;
    let col = piece.col + 1;
    game_state.insert_if_valid(piece, row, col, &mut moves);

    // bot bot left
    if piece.col >= 1 {
        let row = piece.row + 2;
        let col = piece.col - 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    return moves;
}
