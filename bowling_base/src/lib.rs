// Internal Libraries
pub mod game;
pub mod util;

use game::frame::Frame;
use game::info::Info;
use game::Game;
use util::database::DBConn;

// External Libraries
use chrono::NaiveDate;

pub fn add_game(conn: &mut DBConn, game: Game) -> Result<(), String> {
    if !game.valid() {
        return Err("Unable to add game: Game is not valid".to_string());
    }

    match conn.add_game(&game) {
        Err(err) => Err(err),
        _ => Ok(()),
    }
}

pub fn modify_game(conn: &mut DBConn, info: &Info, frame: Frame) -> Result<(), String> {
    if !info.full() {
        return Err(
            "Unable to modify game: Info needs to have both date and game number".to_string(),
        );
    }

    if !frame.valid() {
        return Err("Unable to modify game: Input frame is not valid".to_string());
    }

    match conn.modify_game(info, &frame) {
        Err(err) => Err(err),
        _ => Ok(()),
    }
}

pub fn remove_game(conn: &mut DBConn, info: &Info) -> Result<(), String> {
    if !info.full() {
        return Err(
            "Unable to remove game: Info needs to have both date and game number".to_string(),
        );
    }

    match conn.remove_game(info) {
        Err(err) => Err(err),
        _ => Ok(()),
    }
}

pub fn get_game(conn: &mut DBConn, info: &Info) -> Result<Game, String> {
    if !info.full() {
        return Err("Unable to get game: Info needs to have both date and game number".to_string());
    }

    conn.get_game(info)
}

pub fn get_games(conn: &mut DBConn, info: &Info) -> Result<Vec<Game>, String> {
    if info.full() {
        return Err("Unable to get games: Info needs to only have date".to_string());
    }

    conn.get_games(info)
}

pub fn get_games_played(conn: &mut DBConn, info: &Info) -> u8 {
    conn.get_games_played(info).unwrap_or(0)
}

pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m-%d-%y") {
        return Some(date);
    }
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%y") {
        return Some(date);
    }
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m-%d-%Y") {
        return Some(date);
    }
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        return Some(date);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date() {
        let hypen = "1-2-2023";
        let slash = "1/2/2023";
        let hypen_long = "01-02-2023";
        let slash_long = "01/02/2023";
        let hypen_short = "1-2-23";
        let slash_short = "1/2/23";

        let actual = NaiveDate::from_ymd_opt(2023, 01, 02).unwrap();

        assert_eq!(super::parse_date(hypen).unwrap(), actual);
        assert_eq!(super::parse_date(slash).unwrap(), actual);
        assert_eq!(super::parse_date(hypen_long).unwrap(), actual);
        assert_eq!(super::parse_date(slash_long).unwrap(), actual);
        assert_eq!(super::parse_date(hypen_short).unwrap(), actual);
        assert_eq!(super::parse_date(slash_short).unwrap(), actual);
    }
}
