use bt_base::types::{Frame, Game, GameInfo};
use bt_util::database::DBConn;

use std::collections::VecDeque;
use std::env;
use std::io::{stdin, stdout, Write};

use chrono::{Local, NaiveDate};
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

    let mut conn: DBConn<Game> = match DBConn::connect(
        env::var("DB_USER").ok(),
        env::var("DB_PASS").ok(),
        env::var("DB_IP").ok(),
        env::var("DB_PORT").unwrap().parse().unwrap(),
        env::var("DB_DB").ok(),
        env::var("DB_TABLE").ok().unwrap(),
    ) {
        Ok(conn) => conn,
        Err(err) => panic!("{err}"),
    };

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
                todo!("New option")
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
            cmd => {
                eprintln!("[!] Invalid Command: '{cmd}'");
            }
        }

        println!();
        user_inputs.clear();
    }
}
