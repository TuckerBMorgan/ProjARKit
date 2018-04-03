mod game;
use game::gamestate::GameState;

fn main() {
    let mut game_state = GameState{..Default::default()};
    
    println!("{}", game_state);
    /*game_state.move_piece(6, 4, 4, 4);
    println!("{}", game_state);
    let piece = game_state.grid[7][4].unwrap();
    game_state.print_moves(&game_state.possible_moves(&piece));
    */

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

                let piece = game_state.grid[row1][col1];

                if !piece.is_none() {
                    println!("Trying to move ({}, {}) to ({}, {}):", vals[0], vals[1], vals[2], vals[3]);
                    game_state.move_piece(row1, col1, row2, col2);
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

                game_state.grid[row][col].map(|piece| {
                   println!("Generating possible moves for ({}, {})", row, col);
                   game_state.print_moves(&game_state.possible_moves(&piece));
                });
            },
            'p' => { println!("{}", game_state); continue },
            'e' => break,
            _ => { println!("m -> Move\ns -> Show Moves\np -> Print board\ne -> exit"); continue }
        }
    }
}