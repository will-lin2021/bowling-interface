// Internal Libraries
use bowling_base::game::frame::Frame;
use bowling_base::game::info::Info;
use bowling_base::game::Game;
use bowling_base::util::database::DBConn;

use bowling_base::parse_date;

// External Libraries

use std::collections::VecDeque;
use std::env;
use std::io::{stdin, stdout, Write};

use chrono::{Local, NaiveDate};
use dotenv::dotenv;

fn get_user_input() -> String {
    let mut temp_input = String::new();

    stdin().read_line(&mut temp_input).ok();

    String::from(temp_input.trim())
}

fn split_input(input: &str) -> VecDeque<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
}

fn game_loop(date: NaiveDate, game_num: u8) -> Option<Game> {
    println!(
        "Date: {}, Game Number: {}",
        date.format("%m/%d/%y"),
        game_num
    );

    let game_info = Info::build_with(date, game_num);
    let mut game = Game::build_with(game_info);

    for frame_num in 1..=10 {
        let mut frame = Frame::build_with(frame_num);
        let mut throw_num = 1;
        let mut current_score = 0;

        loop {
            println!("Frame {} Throw {}", frame_num, throw_num);
            print!("[>] ");
            stdout().flush().ok();
            let user_input = get_user_input();

            match user_input.as_str() {
                "exit" | "e" => {
                    println!("Exiting New Game Entry...");

                    return None;
                }
                "help" | "h" => {
                    todo!("Make help menu")
                }
                _ => (),
            }

            let val = match user_input.parse::<u8>() {
                Err(_) => {
                    eprintln!("[!] Invalid value input");

                    println!();
                    continue;
                }
                Ok(11..) => {
                    eprintln!("[!] Too large value input");

                    println!();
                    continue;
                }
                Ok(val) => val,
            };

            current_score += val;

            match (frame_num, throw_num, current_score) {
                (1..=9, 1, 10) => {
                    // Frame 1-9: Strike
                    *frame.throw_mut(1).unwrap() = 10;
                    *frame.throw_mut(2).unwrap() = 0;

                    break;
                }
                (1..=9, 1, _) => {
                    // Frame 1-9: Not Strike
                    *frame.throw_mut(1).unwrap() = val;

                    throw_num += 1;
                }
                (1..=9, 2, 10) => {
                    // Frame 1-9: Spare
                    *frame.throw_mut(2).unwrap() = val;

                    break;
                }
                (1..=9, 2, _) => {
                    // Frame 1-9: Not Spare
                    *frame.throw_mut(2).unwrap() = val;

                    throw_num += 1;
                }
                (10, 1, 10) => {
                    // Frame 10: Strike
                    *frame.throw_mut(1).unwrap() = 10;

                    throw_num += 1;
                }
                (10, 1, _) => {
                    // Frame 10: Not Strike
                    *frame.throw_mut(1).unwrap() = val;

                    throw_num += 1;
                }
                (10, 2, 20) => {
                    // Frame 10: Double Strike
                    *frame.throw_mut(2).unwrap() = 10;

                    throw_num += 1;
                }
                (10, 2, 10) => {
                    // Frame 10: Spare
                    *frame.throw_mut(2).unwrap() = val;

                    throw_num += 1;
                }
                (10, 2, _) => {
                    // Frame 10: No Strike nor Spare; End Game
                    *frame.throw_mut(2).unwrap() = val;

                    break;
                }
                (10, 3, _) => {
                    // Frame 10: Had Strike or Spare; End Game
                    *frame.throw_mut(3).unwrap() = val;

                    break;
                }
                _ => unreachable!(),
            }

            if (frame_num < 10 && throw_num == 3) || (throw_num == 4) {
                break;
            }
        }

        *(game.frame_mut(frame_num).unwrap()) = frame;
    }

    Some(game)
}

fn modify_loop(game: &Game) {
    println!(
        "Date: {}, Game Number: {}",
        game.info().date().format("%m/%d/%y"),
        game.info().game_num().unwrap()
    );
}

fn main() {
    println!("Bowling Score Tracker");

    dotenv().ok();

    let mut conn = DBConn::connect(
        env::var("MARIADB_USER").expect("MARIADB_USER not set"),
        env::var("MARIADB_PASS").expect("MARIADB_PASS not set"),
        env::var("MARIADB_IP").expect("MARIADB_IP not set"),
        env::var("MARIADB_PORT")
            .expect("MARIADB_PORT not set")
            .parse()
            .expect("MARIADB_PORT is not a number"),
        env::var("MARIADB_DB").expect("MARAIDB_DB"),
    )
    .expect("Database connection is necessary for function");

    let mut user_input: String;
    let mut user_inputs: VecDeque<String>;

    loop {
        print!("[>] ");
        stdout().flush().ok();
        user_input = get_user_input();
        user_inputs = split_input(&user_input);

        if user_inputs.is_empty() {
            eprintln!("[!] No inputs given");

            println!();
            user_input.clear();
            user_inputs.clear();
            continue;
        }

        match user_inputs.pop_front().unwrap().as_str() {
            "quit" | "q" => {
                println!("Quitting Bowling Score Tracker...");

                break;
            }
            "new" | "n" => {
                if let 2.. = user_inputs.len() {
                    eprintln!("[!] Too many inputs");

                    println!();
                    user_input.clear();
                    user_inputs.clear();
                    continue;
                }

                let date = match user_inputs.pop_front() {
                    Some(date_string) => match parse_date(date_string.as_str()) {
                        Some(date) => date,
                        None => {
                            eprintln!("[!] Date input is invalid");

                            println!();
                            user_input.clear();
                            user_inputs.clear();
                            continue;
                        }
                    },
                    None => Local::now().date_naive(),
                };

                let info = Info::build_with(date, 0);

                let num_games = bowling_base::get_games_played(&mut conn, &info);

                if let Some(game) = game_loop(date, num_games + 1) {
                    conn.add_game(&game)
                        .unwrap_or_else(|err| eprintln!("[!] {}", err));
                }
            }
            "modify" | "m" => {
                if let 0 = user_inputs.len() {
                    eprintln!("[!] Not enough inputs");

                    println!();
                    user_input.clear();
                    user_inputs.clear();
                    continue;
                }
                if let 3.. = user_inputs.len() {
                    eprintln!("[!] Too many inputs");

                    println!();
                    user_input.clear();
                    user_inputs.clear();
                    continue;
                }

                let date = match user_inputs.pop_front() {
                    Some(date_string) => match parse_date(date_string.as_str()) {
                        Some(date) => date,
                        None => {
                            eprintln!("[!] Date input is invalid");

                            println!();
                            user_input.clear();
                            user_inputs.clear();
                            continue;
                        }
                    },
                    None => Local::now().date_naive(),
                };

                let game_num = match user_inputs.pop_front() {
                    Some(game_num_str) => match game_num_str.parse::<u8>() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("[!] Game number input is invalid");

                            println!();
                            user_input.clear();
                            user_inputs.clear();
                            continue;
                        }
                    },
                    None => match conn.get_games_played(&Info::from(date)) {
                        Ok(0) => {
                            eprintln!("[!] No games played on date");

                            println!();
                            user_input.clear();
                            user_inputs.clear();
                            continue;
                        }
                        Ok(num) => num,
                        _ => {
                            panic!("Database access failed");
                        }
                    },
                };

                let game_info = Info::build_with(date, game_num);
            }
            "print" | "p" => {
                todo!("Make print menu option")
            }
            "remove" | "r" => {
                todo!("Make remove menu option")
            }
            "help" | "h" => {
                todo!("Make help menu option")
            }
            "test" | "t" => {
                unimplemented!("Nothing being tested");
            }
            cmd => {
                eprintln!("[!] Invalid Command: '{cmd}'")
            }
        }

        println!();
        user_input.clear();
        user_inputs.clear();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
