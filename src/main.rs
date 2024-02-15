use bt_base::prelude::*;

use std::env;
use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;

use dotenv::dotenv;

fn parse_input() -> VecDeque<String> {
    let mut temp_input = String::new();

    stdin().read_line(&mut temp_input).ok();

    let temp_input = String::from(temp_input.trim());

    temp_input
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn main() {
    println!("Bowling Score Tracker");

    dotenv().ok();

    let user = env::var("USER_NAME").expect("Username not set in .env");
    let pass = env::var("USER_PASS").expect("Password not set in .env");
    let auth_db = env::var("USER_AUTH").expect("Authentication DB not set in .env");

    let host = env::var("DB_HOST").expect("Host not set in .env");
    let port = env::var("DB_PORT").expect("Port not set in .env");

    let db_name = env::var("DB_NAME").expect("DB name not set in .env");
    let coll_name = env::var("COLL_NAME").expect("Collection name not set in .env");

    let db_conn = DatabaseConn::connect_full(
        &user,
        &pass,
        &host,
        &port,
        &auth_db,
        &db_name,
        &coll_name
    );

    let mut user_inputs: VecDeque<String>;

    loop {
        print!("[>] ");
        stdout().flush().ok();

        user_inputs = parse_input();

        if user_inputs.is_empty() {
            eprintln!("[!] No inputs entered\n");

            user_inputs.clear();
            continue;
        }

        match user_inputs.pop_front().unwrap().as_str() {
            "quit" | "q" => {
                println!("Quitting Bowling Score Tracker...");

                break;
            }
            "new" | "n" => {
                game_loop(Date::build(), 1);
            }
            "modify" | "m" => {
                todo!("Modify option")
            }
            "print" | "p" => {
                todo!("Print option")
            }
            "delete" | "d" => {
                todo!("Delete option")
            }
            "help" | "h" => {
                todo!("Help option")
            }
            "test" | "t" => {
                unimplemented!()
            }
            cmd => {
                eprintln!("[!] Invalid Command: '{cmd}'");
            }
        }

        println!();
        user_inputs.clear();
    }
}

pub fn game_loop(date: Date, game: u8) {
    // TODO: Make loop for new game menu option
    let mut frame_no = 1;

    while frame_no <= 10 {
        print!("Frame {:}: ", frame_no);

        let mut frame = Frame::Uninit;

        while !frame.is_valid() {
            let mut temp = String::new();

            stdin().read_line(&mut temp).ok();

            let output = scan!(&temp, char::is_whitespace, u8, u8);

            println!("{:?}", output);
        }

        frame_no += 1;
    }
}

pub fn modify_loop() {
    // TODO: Make loop for modify game menu option
}
