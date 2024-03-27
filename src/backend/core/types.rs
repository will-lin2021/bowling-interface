use std::cmp::Ordering;

use chrono::{Datelike, Local, NaiveDate};
use itertools::{izip, Itertools};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    // Constructor
    pub fn build() -> Self {
        let today = Local::now().date_naive();

        Self {
            year: today.year() as u16,
            month: today.month() as u8,
            day: today.day() as u8,
        }
    }

    pub fn build_with(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
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
}

impl std::convert::From<(u16, u8, u8)> for Date {
    fn from(value: (u16, u8, u8)) -> Self {
        Date {
            year: value.0,
            month: value.1,
            day: value.2,
        }
    }
}

impl std::convert::From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Date {
            year: value.year() as u16,
            month: value.month() as u8,
            day: value.day() as u8,
        }
    }
}

impl std::cmp::PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl std::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(Ordering::Equal) => match self.month.partial_cmp(&other.month) {
                Some(Ordering::Equal) => self.day.partial_cmp(&other.day),
                other => other,
            },
            other => other,
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}/{:0>2}/{:0>2}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Frame {
    #[default]
    Uninit,
    TwoFrame(u8, u8),
    ThreeFrame(u8, u8, u8),
}

impl Frame {
    // Constructor
    pub fn build() -> Self {
        Self::default()
    }

    // Method
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Uninit => false,
            Self::TwoFrame(t1, t2) => self.score() <= 10 && *t1 <= 10 && *t2 <= 10,
            Self::ThreeFrame(t1, t2, t3) => {
                if *t1 == 10 || *t1 + t2 == 10 {
                    self.score() <= 30 && *t1 <= 10 && *t2 <= 10 && *t3 <= 10
                } else {
                    false
                }
            }
        }
    }

    pub fn is_valid_no(&self, no: u8) -> bool {
        match self {
            Self::Uninit => false,
            Self::TwoFrame(t1, t2) => {
                if no != 10 {
                    self.score() <= 10 && *t1 <= 10 && *t2 <= 10
                } else {
                    self.score() < 10 && *t1 <= 10 && *t2 <= 10
                }
            }
            Self::ThreeFrame(t1, t2, t3) => {
                if no != 10 {
                    false
                } else if *t1 == 10 || *t1 + t2 == 10 {
                    self.score() <= 30 && *t1 <= 10 && *t2 <= 10 && *t3 <= 10
                } else {
                    false
                }
            }
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Self::Uninit => 0,
            Self::TwoFrame(t1, t2) => t1 + t2,
            Self::ThreeFrame(t1, t2, t3) => t1 + t2 + t3,
        }
    }

    pub fn is_strike(&self) -> bool {
        matches!(self, Frame::TwoFrame(10, 0)) || matches!(self, Frame::ThreeFrame(10, _, _))
    }

    pub fn is_spare(&self) -> bool {
        matches!(self, Frame::TwoFrame(t1, t2) if (*t1 != 10) && t1 + t2 == 10)
            || matches!(self, Frame::ThreeFrame(t1, t2, _) if (*t1 != 10) && t1 + t2 == 10)
    }

    pub fn num_strikes(&self) -> u8 {
        match self {
            Self::TwoFrame(10, 0) => 1,
            Self::ThreeFrame(10, 10, 10) => 3,
            Self::ThreeFrame(10, 10, _) => 2,
            Self::ThreeFrame(10, _, _) => 1,
            Self::ThreeFrame(t1, t2, 10) if t1 + t2 == 10 => 1,
            _ => 0,
        }
    }

    pub fn num_spares(&self) -> u8 {
        match self {
            Self::TwoFrame(t1, t2) if *t1 != 10 && t1 + t2 == 10 => 1,
            Self::ThreeFrame(10, t2, t3) if *t2 != 10 && t2 + t3 == 10 => 1,
            Self::ThreeFrame(t1, t2, _) if *t1 != 10 && t1 + t2 == 10 => 1,
            _ => 0,
        }
    }

    pub fn strike_chances(&self) -> u8 {
        match self {
            Self::Uninit => 0,
            Self::TwoFrame(..) => 1,
            Self::ThreeFrame(10, 10, _) => 3,
            Self::ThreeFrame(10, _, _) => 2,
            Self::ThreeFrame(_, _, _) => 1,
        }
    }

    pub fn spare_chances(&self) -> u8 {
        match self {
            Self::Uninit => 0,
            Self::TwoFrame(t1, _) if *t1 != 10 => 1,
            Self::TwoFrame(_, _) => 0,
            Self::ThreeFrame(10, 10, _) => 0,
            Self::ThreeFrame(10, t2, _) if *t2 != 10 => 1,
            Self::ThreeFrame(t1, _, 10) if *t1 != 10 => 1,
            Self::ThreeFrame(_, _, _) => 1,
        }
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

impl std::convert::From<Vec<u8>> for Frame {
    fn from(value: Vec<u8>) -> Self {
        match value.len() {
            1 if value[0] == 10 => Self::TwoFrame(10, 0),
            2 => Self::TwoFrame(value[0], value[1]),
            3 => Self::ThreeFrame(value[0], value[1], value[2]),
            _ => Self::Uninit,
        }
    }
}

impl std::cmp::PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Uninit, Self::Uninit) => true,
            (Self::TwoFrame(s_t1, s_t2), Self::TwoFrame(o_t1, o_t2)) => {
                s_t1 == o_t1 && s_t2 == o_t2
            }
            (Self::ThreeFrame(s_t1, s_t2, s_t3), Self::ThreeFrame(o_t1, o_t2, o_t3)) => {
                s_t1 == o_t1 && s_t2 == o_t2 && s_t3 == o_t3
            }
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    game_num: u8,
    frames: Box<[Frame]>,
}

impl Game {
    // Constructor
    pub fn build(num: u8) -> Self {
        Self {
            game_num: num,
            frames: (1..=10)
                .map(|_| Frame::Uninit)
                .collect::<Vec<Frame>>()
                .into_boxed_slice(),
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

    pub fn game_num_mut(&mut self) -> &mut u8 {
        &mut self.game_num
    }

    pub fn frames_mut(&mut self) -> &mut [Frame] {
        &mut self.frames
    }

    // Method
    pub fn is_valid(&self) -> bool {
        (1..).zip(self.frames.iter()).all(|(n, f)| f.is_valid_no(n))
    }

    // Statistics
    pub fn score(&self) -> u16 {
        let mut score: u16 = 0;

        let iterator = izip!(
            self.frames.iter(),
            self.frames.iter().skip(1),
            self.frames.iter().skip(2)
        );

        for (f1, f2, f3) in iterator {
            if f1.is_strike() {
                score += f1.score() as u16;

                if f2.is_strike() {
                    score += f2.score() as u16;

                    score += match f3 {
                        Frame::Uninit => 0,
                        Frame::TwoFrame(t1, _) => *t1,
                        Frame::ThreeFrame(t1, _, _) => *t1,
                    } as u16;
                } else {
                    score += f2.score() as u16;
                }
            } else if f1.is_spare() {
                score += f1.score() as u16;

                if let Frame::TwoFrame(t1, _) = f2 {
                    score += *t1 as u16;
                }
            } else {
                score += f1.score() as u16;
            }
        }

        let frame_9 = &self.frames[8];
        let frame_10 = &self.frames[9];

        if frame_9.is_strike() {
            score += 10;

            score += match frame_10 {
                Frame::Uninit => 0,
                Frame::TwoFrame(t1, t2) => t1 + t2,
                Frame::ThreeFrame(t1, t2, _) => t1 + t2,
            } as u16;
        } else if frame_9.is_spare() {
            score += 10;

            score += match frame_10 {
                Frame::Uninit => 0,
                Frame::TwoFrame(t1, _) => *t1,
                Frame::ThreeFrame(t1, _, _) => *t1,
            } as u16;
        } else {
            score += frame_9.score() as u16;
        }

        score += frame_10.score() as u16;

        score
    }

    pub fn score_n(&self, frame_no: u8) -> Option<u16> {
        let iterator = izip!(
            (1..=10),
            self.frames.iter(),
            self.frames.iter().skip(1),
            self.frames.iter().skip(2)
        );

        let mut score: u16 = 0;

        for (n, f1, f2, f3) in iterator {
            if n > frame_no {
                return Some(score);
            }

            if !f1.is_valid() {
                return None;
            }

            if f1.is_strike() {
                if !f2.is_valid() {
                    return None;
                }

                if f2.is_strike() {
                    if !f3.is_valid() {
                        return None;
                    }

                    score += f1.score() as u16
                        + f2.score() as u16
                        + match f3 {
                            Frame::Uninit => unreachable!(),
                            Frame::TwoFrame(t1, _) => *t1,
                            Frame::ThreeFrame(t1, _, _) => *t1,
                        } as u16;
                } else {
                    score += f1.score() as u16 + f2.score() as u16;
                }
            } else if f1.is_spare() {
                if !f2.is_valid() {
                    return None;
                }

                if let Frame::TwoFrame(t1, _) = f2 {
                    score += f1.score() as u16 + *t1 as u16;
                }
            } else {
                score += f1.score() as u16;
            }
        }

        if frame_no == 8 {
            return Some(score);
        }

        let frame_9 = &self.frames[8];
        let frame_10 = &self.frames[9];

        if !frame_9.is_valid() {
            return None;
        }

        if frame_9.is_strike() {
            if !frame_10.is_valid() {
                return None;
            }

            score += match frame_10 {
                Frame::Uninit => return None,
                Frame::TwoFrame(t1, t2) => 10 + t1 + t2,
                Frame::ThreeFrame(t1, t2, _) => 10 + t1 + t2,
            } as u16;
        } else if frame_9.is_spare() {
            if !frame_10.is_valid() {
                return None;
            }

            score += match frame_10 {
                Frame::Uninit => 0,
                Frame::TwoFrame(t1, _) => 10 + t1,
                Frame::ThreeFrame(t1, _, _) => 10 + t1,
            } as u16;
        } else {
            score += frame_9.score() as u16;
        }

        if frame_no == 9 {
            return Some(score);
        }

        if !frame_10.is_valid() {
            return None;
        }

        Some(score + frame_10.score() as u16)
    }

    pub fn possible_score() -> u16 {
        todo!() // Get possible score
    }

    pub fn pin_count(&self) -> u8 {
        self.frames.iter().fold(0, |acc, f| match f {
            Frame::Uninit => acc,
            Frame::TwoFrame(t1, t2) => acc + t1 + t2,
            Frame::ThreeFrame(t1, t2, t3) => acc + t1 + t2 + t3,
        })
    }

    pub fn num_strikes(&self) -> u8 {
        self.frames.iter().fold(0, |acc, f| acc + f.num_strikes())
    }

    pub fn num_spares(&self) -> u8 {
        self.frames.iter().fold(0, |acc, f| acc + f.num_spares())
    }

    pub fn strike_chances(&self) -> u8 {
        self.frames
            .iter()
            .fold(0, |acc, f| acc + f.strike_chances())
    }

    pub fn spare_chances(&self) -> u8 {
        self.frames.iter().fold(0, |acc, f| acc + f.spare_chances())
    }

    pub fn open_frames(&self) -> u8 {
        self.frames
            .iter()
            .filter(|f| !f.is_strike() && !f.is_spare())
            .count() as u8
    }

    pub fn clean_frames(&self) -> u8 {
        10 - self.open_frames()
    }

    pub fn avg_first_ball_pinfall(&self) -> f32 {
        let total_first_ball_pins = self.frames.iter().fold(0, |acc, f| match f {
            Frame::Uninit => acc,
            Frame::TwoFrame(t1, _) => acc + t1,
            Frame::ThreeFrame(t1, _, _) => acc + t1,
        });

        total_first_ball_pins as f32 / 10.0
    }
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.game_num == other.game_num
            && self
                .frames
                .iter()
                .zip(other.frames.iter())
                .all(|(g1, g2)| g1 == g2)
    }
}

impl std::cmp::PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = match self.frames()[9] {
            Frame::ThreeFrame(..) => 83,
            _ => 81,
        };
        let top_border: String = (0..cols).map(|_| "=").collect();
        let mid_border: String = (1..=10)
            .map(|n| {
                if n == 10 && cols == 83 {
                    "      ---|"
                } else {
                    "    ---|"
                }
            })
            .collect();

        let frame_scores = self
            .frames()
            .iter()
            .map(|fr| match fr {
                Frame::TwoFrame(10, 0) => "  | X".to_string(),
                Frame::TwoFrame(t1, t2) if t1 + t2 == 10 => {
                    if *t1 == 0 {
                        "- | /".to_string()
                    } else {
                        format!("{} | /", t1)
                    }
                }
                Frame::TwoFrame(t1, t2) if t1 + t2 <= 10 => {
                    if *t1 == 0 && *t2 == 0 {
                        "- | -".to_string()
                    } else if *t1 == 0 {
                        format!("- | {}", t2)
                    } else if *t2 == 0 {
                        format!("{} | -", t1)
                    } else {
                        format!("{} | {}", t1, t2)
                    }
                }
                Frame::ThreeFrame(10, 10, 10) => "X X | X".to_string(),
                Frame::ThreeFrame(10, 10, t3) => {
                    if *t3 == 0 {
                        "X X | -".to_string()
                    } else {
                        format!("X X | {}", t3)
                    }
                }
                Frame::ThreeFrame(10, t2, t3) if t2 + t3 == 10 => {
                    if *t2 == 0 {
                        "X - | /".to_string()
                    } else {
                        format!("X {} | /", t2)
                    }
                }
                Frame::ThreeFrame(t1, t2, 10) if t1 + t2 == 10 => {
                    if *t1 == 0 {
                        "- / | X".to_string()
                    } else {
                        format!("{} / | X", t1)
                    }
                }
                Frame::ThreeFrame(t1, t2, t3) if t1 + t2 == 10 => {
                    if *t1 == 0 && *t3 == 0 {
                        "- / | -".to_string()
                    } else if *t1 == 0 {
                        format!("- / | {}", t3)
                    } else if *t3 == 0 {
                        format!("{} / | -", t1)
                    } else {
                        format!("{} / | {}", t1, t3)
                    }
                }
                Frame::Uninit => "  |  ".to_string(),
                _ => unreachable!(),
            })
            .join(" | ");

        let cumul_scores = (1..=10)
            .zip(self.frames().iter())
            .map(|(n, fr)| match fr {
                Frame::ThreeFrame(..) => format!(
                    "   {}   ",
                    self.score_n(n)
                        .map_or("   ".to_string(), |val| format!("{: >3}", val))
                ),
                _ => format!(
                    "  {}  ",
                    self.score_n(n)
                        .map_or("   ".to_string(), |val| format!("{: >3}", val))
                ),
            })
            .join("|");

        write!(
            f,
            "{}\n| {} |\n|{}\n|{}|\n{}",
            top_border, frame_scores, mid_border, cumul_scores, top_border
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Games {
    #[serde(rename = "_id")]
    date: Date,
    games: Vec<Game>,
}

impl Games {
    // Constructor
    pub fn build() -> Self {
        Games {
            date: Date::build(),
            games: Vec::new(),
        }
    }

    pub fn build_with(date: Date, game: Game) -> Self {
        Games {
            date,
            games: vec![game],
        }
    }

    pub fn build_from_vec(date: Date, games: Vec<Game>) -> Self {
        Games { date, games }
    }

    // Getter
    pub fn date(&self) -> Date {
        self.date
    }

    pub fn games(&self) -> &[Game] {
        &self.games
    }

    pub fn games_mut(&mut self) -> &mut [Game] {
        &mut self.games
    }

    // Method
    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn is_valid(&self) -> bool {
        self.games().iter().zip(1..).all(|(g, n)| g.game_num() == n)
            && self.games().iter().all(|f| f.is_valid())
    }

    pub fn average(&self) -> f32 {
        let total_score = self.games.iter().fold(0, |acc, x| acc + x.score());
        let num_games = self.games.len();

        total_score as f32 / num_games as f32
    }

    pub fn strike_rate(&self) -> f32 {
        let total_strikes = self.games.iter().fold(0, |acc, g| acc + g.num_strikes());
        let num_strike_chances = self.games.iter().fold(0, |acc, f| acc + f.strike_chances());

        total_strikes as f32 / num_strike_chances as f32
    }

    pub fn spare_rate(&self) -> f32 {
        let total_spares = self.games.iter().fold(0, |acc, g| acc + g.num_spares());
        let num_spare_chances = self.games.iter().fold(0, |acc, f| acc + f.spare_chances());

        total_spares as f32 / num_spare_chances as f32
    }

    pub fn open_frame_rate(&self) -> f32 {
        let total_open_frames = self.games.iter().fold(0, |acc, g| acc + g.open_frames());
        let total_frames = self.games.len() * 10;

        total_open_frames as f32 / total_frames as f32
    }

    pub fn clean_frame_rate(&self) -> f32 {
        let total_clean_frames = self.games.iter().fold(0, |acc, g| acc + g.clean_frames());
        let total_frames = self.games.len() * 10;

        total_clean_frames as f32 / total_frames as f32
    }

    pub fn avg_first_ball_pinfall(&self) -> f32 {
        let total_average = self
            .games
            .iter()
            .fold(0.0, |acc: f32, g: &Game| acc + g.avg_first_ball_pinfall());
        let total_games = self.games.len();

        total_average / total_games as f32
    }
}
