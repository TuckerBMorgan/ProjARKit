use std::collections::HashSet;

use game::gamestate::GameState;
use game::piece::Piece;
use game::piece::PieceType;
use game::piece::Coord;

pub fn possible_king_moves(game_state: &GameState, piece: &Piece) -> HashSet<Coord> {
    let mut moves = HashSet::new();

    if !piece.has_moved{
        let rook1 = game_state.grid[piece.row][0];
        let rook2 = game_state.grid[piece.row][7];

        rook1.map(|rook1|{
            if !rook1.has_moved{
                if game_state.grid[piece.row][1].is_none() && game_state.grid[piece.row][2].is_none() && game_state.grid[piece.row][3].is_none(){
                    game_state.insert_if_valid(piece, piece.row, 2, &mut moves);
                    }
            }
        });

        rook2.map(|rook2| {
            if !rook2.has_moved{
                if game_state.grid[piece.row][5].is_none() && game_state.grid[piece.row][6].is_none(){
                    game_state.insert_if_valid(piece, piece.row, 6, &mut moves);
                }
            }
        });
    }
    if piece.row > 0 {
        // up
        let row = piece.row - 1;
        let col = piece.col;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // down
    let mut row = piece.row + 1;
    let mut col = piece.col;
    game_state.insert_if_valid(piece, row, col, &mut moves);

    if piece.col > 0 {
        // left
        row = piece.row;
        col = piece.col - 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    // right
    row = piece.row;
    col = piece.col + 1;
    game_state.insert_if_valid(piece, row, col, &mut moves);

    //down positive diagonal
    row = piece.row + 1;
    col = piece.col + 1;
    game_state.insert_if_valid(piece, row, col, &mut moves);

    if piece.col > 0 {
        //up positive diagonal
        row = piece.row + 1;
        col = piece.col - 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    if piece.row > 0 && piece.col > 0 { 
        //up negative diagonal
        row = piece.row - 1;
        col = piece.col - 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    if piece.row > 0 { 
        //down negative diagonal
        row = piece.row - 1;
        col = piece.col + 1;
        game_state.insert_if_valid(piece, row, col, &mut moves);
    }

    return moves;
}

pub fn in_check(game_state: &GameState, pos: Coord, piece: &Piece) -> bool {
    for row in 0..game_state.rows {
        for col in 0..game_state.cols {
            let curr_piece = game_state.grid[row][col];
            let check = match curr_piece {
                Some(curr_piece) => {
                    if curr_piece.color != piece.color && curr_piece.piece_type != PieceType::King {
                        game_state.possible_moves(&curr_piece).contains(&pos)
                    } else {
                        false
                    }
                },
                None => false
            };

            if check {
                return true;
            }
        }
    }
    return false;
}