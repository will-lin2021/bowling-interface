use bt_base::{
    base::{Date, Frame, Game},
    util::helper::{get_user_input, parse_options, parse_scores},
};

use std::io::{stdout, Write};

pub fn new_game_loop(date: Date, game_num: u8) -> Option<Game> {
    println!("{}: Game {}", date, game_num);

    let mut game = Game::build();

    let mut frame_no = 1;
    let mut bonus = false;
    while frame_no <= 10 {
        println!("{}", game);
        println!("Frame {:}", frame_no);

        let mut throws_left = 2;

        let mut frame_scores: Vec<u8> = Vec::new();
        while throws_left > 0 {
            print!("[>] ");
            stdout().flush().ok();

            let input = get_user_input();

            let options = parse_options(&input);
            if options.is_empty() {
                eprintln!("[!] No inputs entered\n");

                continue;
            }

            match options.get_opt() {
                "quit" | "q" => {
                    println!("Exiting New Game...");

                    return None;
                }
                "modify" | "m" => {
                    let args = options.get_args();

                    if args.len() != 1 {
                        eprintln!("[!] Invalid number of arguments\n");

                        continue;
                    }

                    let frame_num = match args[0].parse::<u8>() {
                        Ok(num) => {
                            if !(1..frame_no).contains(&num) {
                                eprintln!(
                                    "Invalid frame number: must be between 1 and {}\n",
                                    frame_no - 1
                                );

                                continue;
                            }

                            num
                        }
                        Err(_) => {
                            eprintln!("[!] Not a number\n");

                            continue;
                        }
                    };

                    continue;
                }
                _ => (),
            }

            let input_scores: Vec<u8> = parse_scores(&input);

            if input_scores.is_empty() {
                eprintln!("[!] Invalid scores entered\n");

                continue;
            }

            let mut scores =
                input_scores[0..std::cmp::min(input_scores.len(), throws_left)].to_vec();

            if (frame_no != 10 && frame_scores.iter().sum::<u8>() + scores.iter().sum::<u8>() > 10)
                || (frame_scores.iter().sum::<u8>() + scores.iter().sum::<u8>() > 30)
            {
                eprintln!("[!] Invalid scores entered\n");

                continue;
            }

            throws_left -= scores.len();

            frame_scores.append(&mut scores);

            if frame_no != 10 && throws_left == 1 && frame_scores.iter().sum::<u8>() == 10 {
                frame_scores.push(0);
                throws_left = 0;
            } else if frame_no == 10 && !bonus && frame_scores.iter().sum::<u8>() >= 10 {
                throws_left += 1;
                bonus = true
            }

            println!();
        }

        game.frames_mut()[(frame_no - 1) as usize] = Frame::from(frame_scores);

        frame_no += 1;
    }

    println!("{}", game);

    Some(game)
}

pub fn mod_game_loop() {
    // TODO: Make loop for modify game menu option
}
