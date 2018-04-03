use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::Coord;
use game::bishop::possible_bishop_moves;
use game::rook::possible_rook_moves;

pub fn possible_queen_moves(game_state: &GameState, piece: &Piece) -> HashSet<Coord> {
    let mut moves = possible_bishop_moves(game_state, piece);

    moves.extend(possible_rook_moves(game_state, piece));

    return moves;
}
