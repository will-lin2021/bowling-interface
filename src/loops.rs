use crate::error::Error;

use bt_base::{
    base::{Date, Frame, Game},
    util::helper::{get_user_input, parse_options, parse_scores},
};

use std::io::{stdout, Write};

const EMPTY_INPUT: &str = "No inputs entered";
const INVALID_ENTRIES: &str = "Invalid entry";
const INCORRECT_NUM_SCORES: &str = "Invalid number of scores";
const INVALID_SCORES: &str = "Invalid score";

pub fn new_game_loop(date: Date, game_num: u8) -> Option<Game> {
    println!("{}: Game {}", date, game_num);

    let mut new_game = Game::build();

    let mut curr_frame_num = 1;
    let mut done = false;

    'frame_loop: loop {
        println!("{}", new_game);

        if done {
            println!("Game Completed: Use `d` to save and exit game")
        } else {
            println!("Frame {:}", curr_frame_num);
        }

        print!("[>] ");
        stdout().flush().ok();

        // Get user input
        let user_input = get_user_input();

        // Process user input for any menu options
        let user_options = parse_options(&user_input);
        let option = match user_options.get_opt() {
            None => {
                eprintln!("{}\n", Error::Info(EMPTY_INPUT.to_string()));

                continue;
            }
            Some(opt) => opt,
        };

        // Check user option
        match option {
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
                let args = user_options.get_args();

                // Check the number of arguments
                if !(3..=4).contains(&args.len()) {
                    eprintln!(
                        "{}\n",
                        Error::Info("Usage: m <frame num> <t1> <t2> [t3 - opt]".to_string())
                    );

                    continue;
                }

                // Check frame number
                let mod_frame_num = match args[0].parse::<u8>() {
                    Ok(entered_frame_num) => {
                        if !(1..curr_frame_num).contains(&entered_frame_num) {
                            eprintln!(
                                "{}\n",
                                Error::Info(format!(
                                    "Frame number must be between 1-{}",
                                    curr_frame_num - 1
                                ))
                            );

                            continue;
                        }

                        entered_frame_num
                    }
                    Err(_) => {
                        eprintln!(
                            "{}\n",
                            Error::Info("Frame number must be a number".to_string())
                        );

                        continue;
                    }
                };

                // Parse scores
                let mod_scores = match parse_scores(&args[1..].join(" ")) {
                    None => {
                        eprintln!("{}\n", Error::Info(INVALID_ENTRIES.to_string()));

                        continue;
                    }
                    Some(entered_scores) => entered_scores,
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
                    eprintln!("{}\n", Error::Info(INVALID_SCORES.to_string()));

                    continue;
                }

                // Modify previous score
                new_game.frames_mut()[(mod_frame_num - 1) as usize] = mod_frame;

                println!();

                continue 'frame_loop;
            }
            _ => (),
        }

        if done {
            println!("Game is finished.");
            println!("Use `d` to save and exit game, or `q` to quit without saving game");
            println!("`m` can also be used to modify and existing frame");

            continue;
        }

        // Parse scores
        let frame_scores = match parse_scores(&user_input) {
            None => {
                eprintln!("{}\n", Error::Info(INVALID_ENTRIES.to_string()));

                continue;
            }
            Some(scores) => scores,
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
            eprintln!("{}\n", Error::Info(INVALID_SCORES.to_string()));

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

pub fn mod_game_loop() {
    // TODO: Make loop for modify game menu option
}
