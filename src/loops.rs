use std::io::{stdout, Write};

// TODO: Refactor: Move this file to cli

use crate::backend::{
    core::types::{Date, Frame, Game},
    util::helper::*,
};

use crate::error::Error;

const EMPTY_INPUT: &str = "No inputs entered";
const INVALID_ENTRY: &str = "Invalid entry";
const FRAME_NUM_NAN: &str = "Invalid frame number";
const INCORRECT_NUM_SCORES: &str = "Invalid number of scores";
const INVALID_SCORE: &str = "Invalid score";

pub fn new_game_loop(date: Date, game_num: u8) -> Option<Game> {
    println!("Tracking Game {} on {}", game_num, date);

    let mut new_game = Game::build(game_num);

    let mut curr_frame_num = 1;
    let mut done = false;

    'frame_loop: loop {
        println!("{}", new_game);

        if done {
            println!("\nGame Completed: Use `d` to save and exit game")
        } else {
            println!("Frame {:}", curr_frame_num);
        }

        println!("\td: save and exit game");
        println!("\tq: quit without saving game");
        println!("\tm: modify existing frame");

        print!("[>] ");
        stdout().flush().ok();

        // Get user input
        let user_input = get_user_input();

        // Process user input for any menu options
        let user_options = parse_options(&user_input);
        let opt = match user_options.get_opt() {
            Some(str) => str,
            None => {
                eprintln!("{}\n", Error::Info(EMPTY_INPUT.to_string()));

                continue;
            }
        };

        // Check user option
        match opt {
            "quit" | "q" => {
                println!("Exiting...");

                return None;
            }
            "done" | "d" => {
                if !done {
                    eprintln!("Game is not complete\n");

                    continue;
                }

                println!("Saving and exiting...");

                return Some(new_game);
            }
            "modify" | "m" => {
                println!();

                let args = user_options.get_args();
                if args.len() != 1 {
                    eprintln!("{}\n", Error::Info("Usage: m <frame num>".to_string()));

                    continue;
                }

                // Check frame number
                let mod_frame_num = match args[0].parse::<u8>() {
                    Ok(num) => {
                        if !(1..curr_frame_num).contains(&num) {
                            if curr_frame_num == 1 {
                                eprintln!("{}\n", Error::Info("No frames played yet".to_string()));
                            } else {
                                eprintln!(
                                    "{}\n",
                                    Error::Info(format!(
                                        "Frame number must be between 1-{}",
                                        curr_frame_num - 1
                                    ))
                                );
                            }

                            continue;
                        }

                        num
                    }
                    Err(_) => {
                        eprintln!("{}\n", Error::Info(FRAME_NUM_NAN.to_string()));

                        continue;
                    }
                };

                loop {
                    print!("[>] ");
                    stdout().flush().ok();

                    let user_input = get_user_input();

                    let user_options = parse_options(&user_input);
                    let opt = match user_options.get_opt() {
                        Some(str) => str,
                        None => {
                            eprintln!("{}\n", Error::Info(EMPTY_INPUT.to_string()));

                            continue;
                        }
                    };

                    match opt {
                        "quit" | "q" => break,
                        _ => (),
                    }

                    // Parse scores
                    let mod_scores = match parse_scores(&user_input) {
                        Some(vec) => vec,
                        None => {
                            eprintln!("{}\n", Error::Info(INVALID_ENTRY.to_string()));

                            continue;
                        }
                    };

                    // Check number of scores entered
                    if (mod_frame_num < 10 && mod_scores.len() != 2)
                        || (mod_frame_num == 10 && !(2..=3).contains(&mod_scores.len()))
                    {
                        eprintln!("{}\n", Error::Info(INCORRECT_NUM_SCORES.to_string()));

                        continue;
                    }

                    // Check frame validity
                    let mod_frame = Frame::from(mod_scores);
                    if !mod_frame.is_valid_no(mod_frame_num) {
                        eprintln!("{}\n", Error::Info(INVALID_SCORE.to_string()));

                        continue;
                    }

                    // Modify previous score
                    new_game.frames_mut()[(mod_frame_num - 1) as usize] = mod_frame;
                    break;
                }

                println!();

                continue 'frame_loop;
            }
            _ => (),
        }

        if done {
            println!("Game is finished.");

            continue;
        }

        // Parse scores
        let frame_scores = match parse_scores(&user_input) {
            Some(vec) => vec,
            None => {
                eprintln!("{}\n", Error::Info(INVALID_ENTRY.to_string()));

                continue;
            }
        };

        // Check number of scores entered
        if (curr_frame_num < 10 && frame_scores.len() != 2)
            || (curr_frame_num == 10 && !(2..=3).contains(&frame_scores.len()))
        {
            eprintln!("{}\n", Error::Info(INCORRECT_NUM_SCORES.to_string()));

            continue;
        }

        // Check frame validity
        let new_frame = Frame::from(frame_scores);
        if !new_frame.is_valid_no(curr_frame_num) {
            eprintln!("{}\n", Error::Info(INVALID_SCORE.to_string()));

            continue;
        }

        // Add to game
        new_game.frames_mut()[(curr_frame_num - 1) as usize] = new_frame;

        if curr_frame_num == 10 {
            done = true;
        }

        curr_frame_num += 1;

        println!();
    }
}

pub fn mod_game_loop(date: Date, game_num: u8, game: &mut Game) -> Option<&Game> {
    // TODO: print options
    println!("Modifying Game {} on {}", game_num, date);

    let mut modified = false;

    loop {
        println!("{}", game);
        println!("d: save changes and exit");
        println!("q: exit without saving");

        print!("[>] ");
        stdout().flush().ok();

        // Get user input
        let user_input = get_user_input();

        // Process user input for any menu options
        let user_options = parse_options(&user_input);
        let opt = match user_options.get_opt() {
            Some(str) => str,
            None => {
                eprintln!("{}\n", Error::Info(EMPTY_INPUT.to_string()));

                continue;
            }
        };

        match opt {
            "quit" | "q" => {
                println!("Exiting...");

                return None;
            }
            "done" | "d" => {
                if modified {
                    println!("Saving changes, exiting...");
                } else {
                    println!("No changes made, exiting...")
                }

                return Some(game);
            }
            _ => (),
        }

        // Get frame number
        let frame_num = match opt.parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("{}\n", Error::Info(FRAME_NUM_NAN.to_string()));

                continue;
            }
        };

        // Parse scores
        let frame_scores = match parse_scores(&user_options.get_args().join(" ")) {
            Some(scores) => scores,
            None => {
                eprintln!("{}\n", Error::Info(INVALID_ENTRY.to_string()));

                continue;
            }
        };

        // Check number of scores entered
        if (frame_num < 10 && frame_scores.len() != 2)
            || (frame_num == 10 && !(2..=3).contains(&frame_scores.len()))
        {
            eprintln!("{}\n", Error::Info(INCORRECT_NUM_SCORES.to_string()));

            continue;
        }

        // Check frame validity
        let new_frame = Frame::from(frame_scores);
        if !new_frame.is_valid_no(frame_num) {
            eprintln!("{}\n", Error::Info(INVALID_SCORE.to_string()));

            continue;
        }

        // Add to game
        game.frames_mut()[(frame_num - 1) as usize] = new_frame;

        modified = true;

        println!();
    }
}
