use std::cmp::Ordering;

use chrono::{Local, Datelike};
use serde::{Serialize, Deserialize};

//
// Date
//
#[derive(Clone, Serialize, Deserialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

//
// Frame
//
#[derive(Default, Serialize, Deserialize)]
pub enum Frame {
    #[default]
    Uninit,
    TwoFrame(u8, u8),
    ThreeFrame(u8, u8, u8),
}

//
// Game
//
#[derive(Serialize, Deserialize)]
pub struct Game {
    game_num: u8,
    frames: Box<[Frame]>,
}

//
// Games
//
#[derive(Serialize, Deserialize)]
pub struct Games {
    #[serde(rename="_id")]
    date: Date,
    games: Vec<Game>,
}

//
// Date impl
//
impl Date {
    // Constructor
    pub fn build() -> Self {
        let today = Local::now().date_naive();

        Self {
            year: today.year().try_into().unwrap_or_default(),
            month: today.month().try_into().unwrap_or_default(),
            day: today.day().try_into().unwrap_or_default(),
        }
    }

    pub fn build_with(year: u16, month: u8, day: u8) -> Self {
        Self {
            year,
            month,
            day,
        }
    }

    // Getter
    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    // Method

}

impl std::convert::From<(u16, u8, u8)> for Date {
    fn from(value: (u16, u8, u8)) -> Self {
        Date {
            year: value.0,
            month: value.1,
            day: value.2
        }
    }
}

impl std::cmp::PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day
    }
}

impl std::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(Ordering::Equal) => {
                match self.month.partial_cmp(&other.month) {
                    Some(Ordering::Equal) => self.day.partial_cmp(&other.day),
                    other => other
                }
            }
            other => other
        }
    }
}

//
// Frame impl
//
impl Frame {
    // Constructor
    pub fn build() -> Self {
        Self::Uninit
    }

    // Getter
    pub fn frame(&self) -> &Self {
        self
    }

    pub fn frame_mut(&mut self) -> &mut Self {
        self
    }

    // Method
    pub fn is_valid(&self) -> bool {
        match self {
            Self::TwoFrame(t1, t2) => self.score() <= 10 && t1 <= &10 && t2 <= &10,
            Self::ThreeFrame(t1, t2, t3) => {
                if *t1 == 10 || *t1 + t2 == 10 {
                    *t1 <= 10 && *t2 <= 10 && *t3 <= 10 && *t1 + t2 + t3 <= 30
                } else {
                    false
                }
            }
            Self::Uninit => false,
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Self::TwoFrame(t1, t2) => t1 + t2,
            Self::ThreeFrame(t1, t2, t3) => t1 + t2 + t3,
            Self::Uninit => 0,
        }
    }

    pub fn is_strike(&self) -> bool {
        matches!(self, Frame::TwoFrame(10, 0)) || matches!(self, Frame::ThreeFrame(10, ..))
    }

    pub fn is_spare(&self) -> bool {
        matches!(self, Frame::TwoFrame(t1, t2) if (*t1 != 10) && t1 + t2 == 10)
            || matches!(self, Frame::ThreeFrame(t1, t2, ..) if (*t1 != 10) && t1 + t2 == 10)
    }
}

impl std::convert::From<(u8, u8)> for Frame {
    fn from(value: (u8, u8)) -> Self {
        Self::TwoFrame(value.0, value.1)
    }
}

impl std::convert::From<(u8, u8, u8)> for Frame {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::ThreeFrame(value.0, value.1, value.2)
    }
}

impl std::cmp::PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TwoFrame(s_t1, s_t2), Self::TwoFrame(o_t1, o_t2)) => {
                s_t1 == o_t1 && s_t2 == o_t2
            }
            (Self::ThreeFrame(s_t1, s_t2, s_t3), Self::ThreeFrame(o_t1, o_t2, o_t3)) => {
                s_t1 == o_t1 && s_t2 == o_t2 && s_t3 == o_t3
            }
            (Self::Uninit, Self::Uninit) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

//
// Game impl
//
impl Game {
    // Constructor
    pub fn build() -> Self {
        Self {
            game_num: 1,
            frames: Vec::with_capacity(10).into_boxed_slice(),
        }
    }

    pub fn build_with(num: u8, frames: Vec<Frame>) -> Self {
        if frames.len() != 10 {
            panic!()
        }

        Self {
            game_num: num,
            frames: frames.into_boxed_slice(),
        }
    }

    // Getter
    pub fn game_num(&self) -> u8 {
        self.game_num
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    // Method
    pub fn is_valid(&self) -> bool {
        self.frames.iter().all(|x| x.is_valid())
    }

    pub fn score(&self) -> u16 {
        self.frames.iter().fold(0, |acc, x| acc + x.score()).into()
    }

    pub fn strikes(&self) -> u8 {
        // TODO: Count number of strikes in the game
        0
    }

    pub fn spares(&self) -> u8 {
        // TODO: Count number of spares in the game
        0
    }
}

//
// Games impl
//
impl Games {
    // Constructor
    pub fn build_with(date: Date, games: Vec<Game>) -> Self {
        Games {
            date,
            games,
        }
    }

    // Getter
    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn games(&self) -> &Vec<Game> {
        &self.games
    }

    // Method
    pub fn average(&self) -> i16 {
        let total: i16 = self.games.iter().fold(0, |acc, x| acc + x.score()).try_into().unwrap();
        let num: i16 = self.games.len().try_into().unwrap();

        total / num
    }
}
