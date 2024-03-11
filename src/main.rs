use bt_base::prelude::*;

pub mod loops;

use std::{
    env,
    io::{stdout, Write},
};

use dotenv::dotenv;

// TODO: Check out superconsole crate for a better interface for games?

// TODO: Move this error prints to an enum type that corresponds with below TODO
// Print Guide
// [!] General Error: General Errors, such as incorrect input, etc. (used for user errors)
// [!!] Functional Error: Particular Function of Program Doesn't Work (used for internal errors)
// [!!!] Crucial Error: Program Termination (used for severe internal errors)

fn main() {
    dotenv().ok();

    println!("Bowling Score Tracker");

    // TODO: Move these errors into a enum type
    let user = match env::var("USER_NAME") {
        Ok(user) => user,
        Err(_) => {
            println!("[!!!] Database Username not set in ENV");

            return;
        }
    };
    let pass = match env::var("USER_PASS") {
        Ok(pass) => pass,
        Err(_) => {
            println!("[!!!] Database Password not set in ENV");

            return;
        }
    };
    let auth_db = match env::var("USER_AUTH") {
        Ok(auth_db) => auth_db,
        Err(_) => {
            println!("[!!!] Database Auth DB not set in ENV");

            return;
        }
    };

    let host = match env::var("DB_HOST") {
        Ok(host) => host,
        Err(_) => {
            println!("[!!!] Database Host not set in ENV");

            return;
        }
    };
    let port = match env::var("DB_PORT") {
        Ok(port) => port,
        Err(_) => {
            println!("[!!!] Database Port not set in ENV");

            return;
        }
    };

    let db_name = match env::var("DB_NAME") {
        Ok(db_name) => db_name,
        Err(_) => {
            println!("[!!!] Database Name not set in ENV");

            return;
        }
    };
    let coll_name = match env::var("COLL_NAME") {
        Ok(coll_name) => coll_name,
        Err(_) => {
            println!("[!!!] Collection Name not set in ENV");

            return;
        }
    };

    let db_conn =
        DatabaseConn::connect_full(&user, &pass, &host, &port, &auth_db, &db_name, &coll_name);

    let mut user_menu_option: MenuOption;
    loop {
        print!("[>] ");
        stdout().flush().ok();

        user_menu_option = parse_options(get_user_input().as_str());

        if user_menu_option.is_empty() {
            eprintln!("[!] No inputs entered\n");

            continue;
        }

        match user_menu_option.get_opt() {
            "quit" | "q" => {
                println!("Quitting Bowling Score Tracker...");

                break;
            }
            "new" | "n" => {
                println!();

                let args = user_menu_option.get_args();

                let date = match args.len() {
                    0 => Date::build(),
                    1 => match parse_date(args[0].as_str()) {
                        Some(date) => date,
                        None => {
                            eprintln!("[!] Invalid date entry...\n");

                            continue;
                        }
                    },
                    _ => {
                        eprintln!("[!] Invalid arguments for New Game...\n");

                        continue;
                    }
                };

                println!(
                    "{:?}",
                    loops::new_game_loop(date, db_conn.num_games(date).unwrap() + 1)
                );
            }
            "modify" | "m" => {
                println!();

                todo!("Modify option")
            }
            "get" | "g" => {
                println!();

                todo!("Print option")
            }
            "delete" | "d" => {
                println!();

                todo!("Delete option")
            }
            "option" | "o" => {
                println!();

                todo!("Options option")
            }
            "help" | "h" => {
                println!();

                todo!("Help option")
            }
            "test" | "t" => {
                unimplemented!()
            }
            cmd => {
                if cmd
                    .as_bytes()
                    .iter()
                    .all(|b| (65..=90).contains(b) || (97..=122).contains(b))
                {
                    eprintln!("[!] Invalid Command: '{}'\n", cmd);
                } else {
                    eprintln!("[!] Invalid Command\n");
                }

                continue;
            }
        }

        println!();
    }
}
