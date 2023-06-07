// Internal Libraries
use crate::game::frame::Frame;
use crate::game::info::Info;
use crate::game::Game;

// External Libraries
use mysql::params;
use mysql::prelude::*;
use mysql::OptsBuilder;
use mysql::Pool;

pub struct DBConn {
    conn: mysql::PooledConn,
}

impl DBConn {
    pub fn connect(
        user: String,
        pass: String,
        ip: String,
        port: u16,
        db_name: String,
    ) -> Result<Self, String> {
        let opts = OptsBuilder::new()
            .user(Some(user))
            .pass(Some(pass))
            .ip_or_hostname(Some(ip))
            .tcp_port(port)
            .db_name(Some(db_name));

        let pool = Pool::new(opts);
        if let Err(err) = pool {
            return Err(err.to_string());
        }
        let pool = pool.unwrap();

        Ok(Self {
            conn: pool.get_conn().expect("Unable to connect to database"),
        })
    }

    pub fn add_game(&mut self, game: &Game) -> Result<(), String> {
        let prep = self.conn.prep(
            "INSERT INTO data
            (date, game, f1t1, f1t2, f2t1, f2t2, f3t1, f3t2, f4t1, f4t2, f5t1, f5t2, f6t1, f6t2, f7t1, f7t2, f8t1, f8t2, f9t1, f9t2, f10t1, f10t2, f10t3)
            VALUES
            (:date, :game, :f1t1, :f1t2, :f2t1, :f2t2, :f3t1, :f3t2, :f4t1, :f4t2, :f5t1, :f5t2, :f6t1, :f6t2, :f7t1, :f7t2, :f8t1, :f8t2, :f9t1, :f9t2, :f10t1, :f10t2, :f10t3)"
        );
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        let param = params! {
            "date" => game.info().date(), "game" => game.info().game_num(),
            "f1t1" => game.frame(1).unwrap().throw(1), "f1t2" => game.frame(1).unwrap().throw(2),
            "f2t1" => game.frame(2).unwrap().throw(1), "f2t2" => game.frame(2).unwrap().throw(2),
            "f3t1" => game.frame(3).unwrap().throw(1), "f3t2" => game.frame(3).unwrap().throw(2),
            "f4t1" => game.frame(4).unwrap().throw(1), "f4t2" => game.frame(4).unwrap().throw(2),
            "f5t1" => game.frame(5).unwrap().throw(1), "f5t2" => game.frame(5).unwrap().throw(2),
            "f6t1" => game.frame(6).unwrap().throw(1), "f6t2" => game.frame(6).unwrap().throw(2),
            "f7t1" => game.frame(7).unwrap().throw(1), "f7t2" => game.frame(7).unwrap().throw(2),
            "f8t1" => game.frame(8).unwrap().throw(1), "f8t2" => game.frame(8).unwrap().throw(2),
            "f9t1" => game.frame(9).unwrap().throw(1), "f9t2" => game.frame(9).unwrap().throw(2),
            "f10t1" => game.frame(10).unwrap().throw(1), "f10t2" => game.frame(10).unwrap().throw(2), "f10t3" => game.frame(10).unwrap().throw(3),
        };

        let result = self.conn.exec_drop(&prep, param);
        if let Err(err) = result {
            return Err(err.to_string());
        }

        Ok(())
    }

    pub fn modify_game(&mut self, info: &Info, new_frame: &Frame) -> Result<(), String> {
        let frame_num = new_frame.frame_num();

        let command = match frame_num {
            1..=9 => {
                format!("UPDATE data SET f{frame_num}t1=:throw1, f{frame_num}t2=:throw2 WHERE date=:date")
            }
            10 => {
                format!("UPDATE data SET f{frame_num}t1=:throw1, f{frame_num}t2=:throw2, f{frame_num}t3=:throw3 WHERE date=:date")
            }
            _ => {
                return Err("Invalid Frame Number".to_string());
            }
        };

        let prep = self.conn.prep(command.as_str());
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        let param = match frame_num {
            1..=9 => {
                params! {
                    "throw1" => new_frame.throw(1),
                    "throw2" => new_frame.throw(2),
                    "throw3" => new_frame.throw(3),
                    "date" => info.date(),
                }
            }
            10 => {
                params! {
                    "throw1" => new_frame.throw(1),
                    "throw2" => new_frame.throw(2),
                    "throw3" => new_frame.throw(3),
                    "date" => info.date(),
                }
            }
            _ => {
                return Err("Invalid Frame Number".to_string());
            }
        };

        let result = self.conn.exec_drop(&prep, param);
        if let Err(err) = result {
            return Err(err.to_string());
        }

        Ok(())
    }

    pub fn remove_game(&mut self, info: &Info) -> Result<(), String> {
        let prep = self
            .conn
            .prep("DELETE FROM data WHERE (date, game) = (:date, :game)");
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        if info.game_num().is_none() {
            return Err("No game number to delete game".to_string());
        }

        let param = params! {
            "date" => info.date(),
            "game" => info.game_num(),
        };

        let result = self.conn.exec_drop(&prep, param);
        if let Err(err) = result {
            return Err(err.to_string());
        }

        Ok(())
    }

    pub fn get_game(&mut self, info: &Info) -> Result<Game, String> {
        let prep = self
            .conn
            .prep("SELECT * FROM data WHERE (date, game) = (:date, :game)");
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        let param = params! {
            "date" => info.date(),
            "game" => info.game_num(),
        };

        let game = self.conn.exec_first(&prep, param);
        if let Err(err) = game {
            return Err(err.to_string());
        }
        let game = game.unwrap();

        println!("{:#?}", game);

        Ok(game.unwrap())
    }

    pub fn get_games(&mut self, info: &Info) -> Result<Vec<Game>, String> {
        let prep = self.conn.prep("SELECT * FROM data WHERE date=:date");
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        let param = params! {
            "date" => info.date(),
        };

        let game = self.conn.exec(&prep, param);
        if let Err(err) = game {
            return Err(err.to_string());
        }
        let game = game.unwrap();

        Ok(game)
    }

    pub fn get_games_played(&mut self, info: &Info) -> Result<u8, String> {
        let prep = self.conn.prep("SELECT COUNT(*) FROM data WHERE date=:date");
        if let Err(err) = prep {
            return Err(err.to_string());
        }
        let prep = prep.unwrap();

        let param = params! {
            "date" => info.date(),
        };

        let num_games = self.conn.exec_first(&prep, param);
        if let Err(err) = num_games {
            return Err(err.to_string());
        }
        let num_games = num_games.unwrap();

        Ok(num_games.unwrap())
    }
}

// Testing
#[cfg(test)]
mod tests {
    use super::DBConn;

    use crate::game::frame::Frame;
    use crate::game::info::Info;
    use crate::game::Game;

    use std::env;

    use chrono::NaiveDate;
    use dotenv::dotenv;

    fn setup() -> DBConn {
        dotenv().ok();

        DBConn::connect(
            env::var("MARIADB_USER").expect("MARIADB_USER not set"),
            env::var("MARIADB_PASS").expect("MARIADB_PASS not set"),
            env::var("MARIADB_IP").expect("MARIADB_IP not set"),
            env::var("MARIADB_PORT")
                .expect("MARIADB_PORT not set")
                .parse()
                .expect("MARIADB_PORT is not a number"),
            env::var("MARIADB_DB").expect("MARAIDB_DB"),
        )
        .expect("Unable to connect to local database")
    }

    fn verify_cleanup() -> bool {
        let mut local = setup();

        let info = Info::build_with(NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(), 1);

        local.get_games_played(&info).unwrap() == 0
    }

    #[test]
    fn seq_tests() {
        add_game();
        assert!(verify_cleanup());
        modify_game();
        assert!(verify_cleanup());
        remove_game();
        assert!(verify_cleanup());
        get_game();
        assert!(verify_cleanup());
        get_games();
        assert!(verify_cleanup());
        get_games_played();
        assert!(verify_cleanup());
    }

    fn add_game() {
        let mut local = setup();

        // Game info
        let info = Info::build_with(NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(), 1);

        // Game instance init
        let test_game = Game::build_with(info.clone());

        // FUNCTION TO TEST
        local.add_game(&test_game).unwrap();

        // Verify funtion worked
        assert_eq!(test_game, local.get_game(&info).unwrap());

        // Cleanup
        local.remove_game(&info).unwrap();
    }

    fn modify_game() {
        let mut local = setup();

        // Game info
        let info = Info::build_with(NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(), 1);

        // Game instance init
        let mut test_game = Game::build_with(info.clone());

        // Add game to database
        local.add_game(&test_game).unwrap();

        // Frame to modify
        let test_frame = Frame::from((1, 5, 5));

        // Modify frame in Game instance
        *test_game.frame_mut(1).unwrap() = test_frame.clone();

        // FUNCTION TO TEST
        local.modify_game(&info, &test_frame).unwrap();

        // Verify function worked
        assert_eq!(test_game.frame(1), local.get_game(&info).unwrap().frame(1));

        // Cleanup
        local.remove_game(&info).unwrap();
    }

    fn remove_game() {
        let mut local = setup();

        // Game info
        let info = Info::build_with(NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(), 1);

        // Game instance init
        let test_game = Game::build_with(info.clone());

        // Add game to database
        local.add_game(&test_game).unwrap();

        // FUNCTION TO TEST
        local.remove_game(&info).unwrap();

        // Verify function worked
        assert_eq!(local.get_games_played(&info), Ok(0));
    }

    fn get_game() {
        let mut local = setup();

        // Game info
        let info = Info::build_with(NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(), 1);

        // Game instance init
        let test_game = Game::build_with(info.clone());

        // Add game to database
        local.add_game(&test_game).unwrap();

        // FUNCTION TO TEST
        assert_eq!(local.get_game(&info).unwrap(), test_game);

        // Cleanup
        local.remove_game(&info).unwrap();
    }

    fn get_games() {
        let mut local = setup();

        // Game infos
        let info = Info::build_with(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(), 0);
        let infos = vec![
            Info::build_with(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(), 1),
            Info::build_with(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(), 2),
            Info::build_with(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(), 3),
        ];

        // Game instances init
        let test_games: Vec<Game> = infos.iter().map(|x| Game::build_with(x.clone())).collect();

        // Add games to database
        for game in &test_games {
            local.add_game(game).unwrap();
        }

        // FUNCTION TO TEST
        assert_eq!(local.get_games(&info).unwrap(), test_games);

        // Cleanup
        for info in &infos {
            local.remove_game(&info).unwrap();
        }
    }

    fn get_games_played() {
        let mut local = setup();

        // Game infos
        let info = Info::build_with(NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(), 0);
        let mut infos: Vec<Info> = Vec::new();
        for i in 1..=10 {
            infos.push(Info::build_with(
                NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(),
                i,
            ));
        }

        // FUNCTION TO TEST
        for i in 1..=10 {
            local
                .add_game(&Game::build_with(infos[i - 1].clone()))
                .unwrap();
            assert_eq!(local.get_games_played(&info).unwrap() as usize, i);
        }

        // Cleanup
        for i in 1..=10 {
            local.remove_game(&infos[i - 1]).unwrap();
        }
    }
}
