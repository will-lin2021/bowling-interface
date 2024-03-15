pub mod error;
pub mod loops;

use bt_base::prelude::*;

use error::Error;

use std::{
    env,
    io::{stdout, Write},
};

use dotenvy::dotenv;

// TODO: Check out superconsole crate for a better interface for games?

// TODO: Move this error prints to an enum type that corresponds with below TODO
// Print Guide
// [!] General Error: General Errors, such as incorrect input, etc. (used for user errors)
// [!!] Functional Error: Particular Function of Program Doesn't Work (used for internal errors)
// [!!!] Crucial Error: Program Termination (used for severe internal errors)

fn main() {
    dotenv().expect(".env not found");

    println!("Bowling Score Tracker");

    let user = match env::var("USER_NAME") {
        Ok(user) => user,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Username not set in ENV".to_string())
            );

            return;
        }
    };
    let pass = match env::var("USER_PASS") {
        Ok(pass) => pass,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Password not set in ENV".to_string())
            );

            return;
        }
    };
    let auth_db = match env::var("USER_AUTH") {
        Ok(auth_db) => auth_db,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Auth DB not set in ENV".to_string())
            );

            return;
        }
    };

    let host = match env::var("DB_HOST") {
        Ok(host) => host,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Host not set in ENV".to_string())
            );

            return;
        }
    };
    let port = match env::var("DB_PORT") {
        Ok(port) => port,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Port not set in ENV".to_string())
            );

            return;
        }
    };

    let db_name = match env::var("DB_NAME") {
        Ok(db_name) => db_name,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Database Name not set in ENV".to_string())
            );

            return;
        }
    };
    let coll_name = match env::var("COLL_NAME") {
        Ok(coll_name) => coll_name,
        Err(_) => {
            eprintln!(
                "{}",
                Error::Severe("Collection Name not set in ENV".to_string())
            );

            return;
        }
    };

    let db_conn =
        DatabaseConn::connect_full(&user, &pass, &host, &port, &auth_db, &db_name, &coll_name);

    let mut user_menu_options: MenuOption;
    loop {
        print!("[>] ");
        stdout().flush().ok();

        user_menu_options = parse_options(get_user_input().as_str());
        let user_menu_opt = match user_menu_options.get_opt() {
            None => {
                eprintln!("{}\n", Error::Info("No inputs entered".to_string()));

                continue;
            }
            Some(opt) => opt,
        };

        match user_menu_opt {
            "quit" | "q" => {
                println!("Quitting Bowling Score Tracker...");

                break;
            }
            "new" | "n" => {
                println!();

                let args = user_menu_options.get_args();

                let date = match args.len() {
                    0 => Date::build(),
                    1 => match parse_date(args[0].as_str()) {
                        Some(date) => date,
                        None => {
                            eprintln!("{}\n", Error::Info("Invalid date entry".to_string()));

                            continue;
                        }
                    },
                    _ => {
                        eprintln!(
                            "{}\n",
                            Error::Info("Invalid arguments for New Game".to_string())
                        );

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
            "print" | "p" => {
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
                    eprintln!("{}", Error::Info(format!("Invalid Command: '{}'\n", cmd)));
                } else {
                    eprintln!("{}", Error::Info("Invalid Command\n".to_string()));
                }

                continue;
            }
        }

        println!();
    }
}
