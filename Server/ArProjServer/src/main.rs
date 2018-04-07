mod game;
use game::gamestate::GameState;

fn main() {
    let mut game_state = GameState::new();
   
    // loop for input
    use std::io::{stdin};
    let mut s = String::new();

    println!("<<{:?}'s Turn>>", game_state.turn);
    println!("{}", game_state);

    loop {
        s.clear();
        println!("(m)ove; (s)how moves; (p)rint board; (e)xit");
        match stdin().read_line(&mut s) {
            Ok(_) => {
                let input = s.trim_right();
                match input.chars().next().unwrap() {
                    'm' => {
                        let mut x = String::new(); 
                        let mut vals = input.split_whitespace().collect::<Vec<&str>>();

                        if vals.len() != 5 {
                            println!("Enter values: (format => row1 col1 row2 col2)");
                            match stdin().read_line(&mut x) {
                                Ok(_) => {
                                    vals = x.split_whitespace().collect();
                                },
                                Err(error) => println!("[Error] {}", error)
                            }
                        } else {
                            vals.remove(0); 
                        }

                        if vals.len() != 4 {
                            println!("[Error] Not enough arguments");
                            continue;
                        }

                        match vals[0].parse::<usize>() {
                            Ok(row1) => {
                                match vals[1].parse::<usize>() {
                                    Ok(col1) => {
                                        match vals[2].parse::<usize>() {
                                            Ok(row2) => {
                                                match vals[3].parse::<usize>() {
                                                    Ok(col2) => {
                                                        match game_state.grid[row1][col1] {
                                                            Some(piece) => {
                                                                if piece.color == game_state.turn {
                                                                    println!("Trying to move ({}, {}) to ({}, {}):", vals[0], vals[1], vals[2], vals[3]);
                                                                    game_state.move_piece(row1, col1, row2, col2);
                                                                    println!("<<{:?}'s Turn>>", game_state.turn);
                                                                    println!("{}", game_state);
                                                                } else {
                                                                    println!("[Error] Trying to move {:?} piece on {:?}'s turn.", piece.color, game_state.turn);
                                                                }
                                                            },
                                                            None => println!("[Error] No piece at ({}, {})", row1, col1)
                                                        }
                                                    },
                                                    Err(e) => println!("[Error] {}: {}", e, vals[3])
                                                }
                                            },
                                            Err(e) => println!("<Error>> {}: {}", e, vals[2])
                                        }
                                    },
                                    Err(e) => println!("[Error] {}: {}", e, vals[1])
                                }
                            },
                            Err(e) => println!("[Error] {}: {}", e, vals[0])
                        }
                    },
                    's' => {
                        let mut x = String::new(); 
                        let mut vals = input.split_whitespace().collect::<Vec<&str>>();

                        if vals.len() != 3 {
                            println!("Enter values: (format => row col)");
                            match stdin().read_line(&mut x) {
                                Ok(_) => {
                                    vals = x.split_whitespace().collect();
                                },
                                Err(error) => println!("[Error] {}", error)
                            }
                        } else {
                            vals.remove(0);
                        }

                        if vals.len() != 2 {
                            println!("[Error] Not enough arguments");
                            continue;
                        }

                        match vals[0].parse::<usize>() {
                            Ok(row) => {
                                match vals[1].parse::<usize>() {
                                    Ok(col) => {
                                        game_state.grid[row][col].map(|piece| {
                                            println!("Generating possible moves for ({}, {})", row, col);
                                            game_state.print_moves(game_state.possible_moves(piece));
                                        });
                                    },
                                    Err(e) => println!("{}: {}", e, vals[1])
                                }
                            },
                            Err(e) => println!("{}: {}", e, vals[0])
                        }
                    },
                    'g' => {
                        if game_state.no_available_moves(game_state.turn) {
                            println!("{:?} has been checkmated! gg", game_state.turn);
                            break
                        }
                        continue 
                    },
                    'p' => { 
                        println!("<<{:?}'s Turn>>", game_state.turn);
                        println!("{}", game_state); 
                        continue 
                    },
                    'e' => break,
                    _ => { println!("m -> Move\ns -> Show Moves\np -> Print board\ne -> exit"); continue }
                }
            },
            Err(error) => println!("[Error] {}", error)
        }
    }
}