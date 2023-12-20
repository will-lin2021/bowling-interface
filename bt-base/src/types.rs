use std::{cmp::Ordering, u8};

use chrono::{Local, NaiveDate};
use mysql::{params, prelude::FromValue, FromRowError, Params, Row};

//
// GameInfo Type
//
#[derive(Debug)]
pub enum GameInfo {
    None,
    Partial(NaiveDate),
    Full(NaiveDate, u8),
}

impl GameInfo {
    // Constructor
    pub fn build() -> Self {
        Self::None
    }

    pub fn build_date(date: NaiveDate) -> Self {
        Self::Partial(date)
    }

    pub fn build_with(date: NaiveDate, game: u8) -> Self {
        Self::Full(date, game)
    }

    // Getter
    pub fn date(&self) -> &NaiveDate {
        match self {
            Self::Partial(date) | Self::Full(date, _) => date,
            Self::None => panic!(),
        }
    }

    pub fn game(&self) -> u8 {
        match self {
            Self::Full(_, game) => *game,
            _ => panic!(),
        }
    }

    pub fn date_mut(&mut self) -> &mut NaiveDate {
        match self {
            Self::Partial(date) | Self::Full(date, _) => date,
            Self::None => panic!(),
        }
    }

    pub fn game_mut(&mut self) -> &mut u8 {
        match self {
            Self::Full(_, game) => game,
            _ => panic!(),
        }
    }
}

// `PartialEq` impl
impl std::cmp::PartialEq for GameInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Partial(s_date), Self::Partial(o_date)) => s_date == o_date,
            (Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => {
                s_date == o_date && s_game == o_game
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

// `PartialOrd` impl
impl std::cmp::PartialOrd for GameInfo {
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Partial(s_date), Self::Partial(o_date)) => s_date >= o_date,
            (Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => {
                if s_date > o_date {
                    true
                } else if s_date == o_date {
                    s_game >= o_game
                } else {
                    false
                }
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Partial(s_date), Self::Partial(o_date)) => s_date > o_date,
            (Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => {
                if s_date > o_date {
                    true
                } else if s_date == o_date {
                    s_game > o_game
                } else {
                    false
                }
            }
            (Self::None, Self::None) => false,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Partial(s_date), Self::Partial(o_date)) => s_date <= o_date,
            (Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => {
                if s_date < o_date {
                    true
                } else if s_date == o_date {
                    s_game <= o_game
                } else {
                    false
                }
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Partial(s_date), Self::Partial(o_date)) => s_date < o_date,
            (Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => {
                if s_date < o_date {
                    true
                } else if s_date == o_date {
                    s_game < o_game
                } else {
                    false
                }
            }
            (Self::None, Self::None) => false,
            _ => false,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.date() < other.date() {
            // Date is before
            Some(Ordering::Less)
        } else if self.date() > other.date() {
            // Date is after
            Some(Ordering::Greater)
        } else if self.game() < other.game() {
            // Date is same; Game No. is before
            Some(Ordering::Less)
        } else if self.game() > other.game() {
            // Date is same; Game No. is after
            Some(Ordering::Greater)
        } else {
            // Date is same; Game No. is same
            Some(Ordering::Equal)
        }
    }
}

// `From` impl
impl std::convert::From<NaiveDate> for GameInfo {
    fn from(value: NaiveDate) -> Self {
        Self::Partial(value)
    }
}

impl std::convert::From<(NaiveDate, u8)> for GameInfo {
    fn from(value: (NaiveDate, u8)) -> Self {
        Self::Full(value.0, value.1)
    }
}

// `DBQuery` impl
impl bt_util::database::DBQuery for GameInfo {
    fn query_statement(&self, table: &str, sort: bool) -> String {
        match self {
            GameInfo::None => {
                if sort {
                    format!("SELECT * FROM {table} ORDER BY date, num")
                } else {
                    format!("SELECT * FROM {table}")
                }
            }
            GameInfo::Partial(..) => {
                if sort {
                    format!("SELECT * FROM {table} WHERE (date) = (:date) ORDER BY num")
                } else {
                    format!("SELECT * FROM {table} WHERE (date) = (:date)")
                }
            }
            GameInfo::Full(..) => {
                if sort {
                    format!(
                        "SELECT * FROM {table} WHERE (date, game) = (:date, :game) ORDER BY date"
                    )
                } else {
                    format!("SELECT * FROM {table} WHERE (date, game) = (:date, :game)")
                }
            }
        }
    }

    fn query_parameter(&self) -> Params {
        match self {
            GameInfo::Partial(date) => {
                params! {
                    "date" => date
                }
            }
            GameInfo::Full(date, game) => {
                params! {
                    "date" => date,
                    "game" => game,
                }
            }
            GameInfo::None => Params::Empty,
        }
    }
}

// `DBDelete` impl
impl bt_util::database::DBDelete for GameInfo {
    fn del_statement(&self, table: &str) -> String {
        format!("DELETE FROM {table} WHERE (date, game) = (:date, :game)")
    }

    fn del_parameter(&self) -> Params {
        params! {
            "date" => self.date(),
            "game" => self.game(),
        }
    }
}

//
// Frame Type
//
#[derive(Debug, Default, Clone)]
pub enum Frame {
    #[default]
    Uninit,
    TwoFrame(u8, u8, u8),
    ThreeFrame(u8, u8, u8, u8),
}

impl Frame {
    // Constructor
    pub fn build() -> Self {
        Self::Uninit
    }

    // Getter
    pub fn frame(&self) -> &Self {
        self
    }

    pub fn throw(&self, throw_num: usize) -> u8 {
        match self {
            Frame::TwoFrame(_, t1, t2) => match throw_num {
                1 => *t1,
                2 => *t2,
                _ => panic!(),
            },
            Frame::ThreeFrame(_, t1, t2, t3) => match throw_num {
                1 => *t1,
                2 => *t2,
                3 => *t3,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn throw_opt(&self, throw_num: usize) -> Option<u8> {
        match self {
            Frame::TwoFrame(_, t1, t2) => match throw_num {
                1 => Some(*t1),
                2 => Some(*t2),
                _ => None,
            },
            Frame::ThreeFrame(_, t1, t2, t3) => match throw_num {
                1 => Some(*t1),
                2 => Some(*t2),
                3 => Some(*t3),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn frame_mut(&mut self) -> &mut Self {
        self
    }

    // Type-specific
    pub fn is_valid(&self) -> bool {
        match self {
            Self::TwoFrame(fr, ..) => (1..=10).contains(fr) && self.score() <= 10,
            Self::ThreeFrame(fr, t1, t2, t3) => {
                if self.score() > 30 || (*t1 > 10 || *t2 > 10 || *t3 > 10) {
                    false
                } else {
                    (1..=10).contains(fr) && t1 + t2 >= 10
                }
            }
            Self::Uninit => false,
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Self::TwoFrame(_, t1, t2) => t1 + t2,
            Self::ThreeFrame(_, t1, t2, t3) => t1 + t2 + t3,
            Self::Uninit => 0,
        }
    }

    pub fn is_strike(&self) -> bool {
        matches!(self, Frame::TwoFrame(_, 10, 0)) || matches!(self, Frame::ThreeFrame(_, 10, ..))
    }

    pub fn is_spare(&self) -> bool {
        matches!(self, Frame::TwoFrame(_, t1, t2) if (*t1 != 10) && t1 + t2 == 10)
            || matches!(self, Frame::ThreeFrame(_, t1, t2, ..) if (*t1 != 10) && t1 + t2 == 10)
    }
}

// `From` impl
impl std::convert::From<(u8, u8, u8)> for Frame {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::TwoFrame(value.0, value.1, value.2)
    }
}

impl std::convert::From<(u8, u8, u8, u8)> for Frame {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::ThreeFrame(value.0, value.1, value.2, value.3)
    }
}

// `PartialEq` impl
impl std::cmp::PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TwoFrame(s_fr, s_t1, s_t2), Self::TwoFrame(o_fr, o_t1, o_t2)) => {
                s_fr == o_fr && s_t1 == o_t1 && s_t2 == o_t2
            }
            (
                Self::ThreeFrame(s_fr, s_t1, s_t2, s_t3),
                Self::ThreeFrame(o_fr, o_t1, o_t2, o_t3),
            ) => s_fr == o_fr && s_t1 == o_t1 && s_t2 == o_t2 && s_t3 == o_t3,
            (Self::Uninit, Self::Uninit) => true,
            _ => false,
        }
    }
}

//
// Game Type
//
pub struct Game {
    date: NaiveDate,
    game: u8,
    frames: Box<[Frame]>,
}

// Constructor
impl Game {
    // Constructor
    pub fn build() -> Game {
        Game {
            date: Local::now().date_naive(),
            game: 1,
            frames: (0..10)
                .map(|_| Frame::Uninit)
                .collect::<Vec<Frame>>()
                .into_boxed_slice(),
        }
    }

    pub fn build_date(date: NaiveDate) -> Game {
        Game {
            date,
            game: 1,
            frames: (0..10)
                .map(|_| Frame::Uninit)
                .collect::<Vec<Frame>>()
                .into_boxed_slice(),
        }
    }

    pub fn build_with(info: GameInfo) -> Game {
        match info {
            GameInfo::None => Game {
                date: Local::now().date_naive(),
                game: 1,
                frames: (0..10)
                    .map(|_| Frame::Uninit)
                    .collect::<Vec<Frame>>()
                    .into_boxed_slice(),
            },
            GameInfo::Partial(date) => Game {
                date,
                game: 1,
                frames: (0..10)
                    .map(|_| Frame::Uninit)
                    .collect::<Vec<Frame>>()
                    .into_boxed_slice(),
            },
            GameInfo::Full(date, game) => Game {
                date,
                game,
                frames: (0..10)
                    .map(|_| Frame::Uninit)
                    .collect::<Vec<Frame>>()
                    .into_boxed_slice(),
            },
        }
    }

    // Getter
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn game(&self) -> u8 {
        self.game
    }

    fn frame(&self, frame_num: usize) -> &Frame {
        &self.frames[frame_num - 1]
    }

    pub fn frame_opt(&self, frame_num: usize) -> Option<&Frame> {
        if !(1..=10).contains(&frame_num) {
            return None;
        }

        Some(&self.frames[frame_num - 1])
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn date_mut(&mut self) -> &mut NaiveDate {
        &mut self.date
    }

    pub fn game_mut(&mut self) -> &mut u8 {
        &mut self.game
    }

    fn frame_mut(&mut self, frame_num: usize) -> &mut Frame {
        &mut self.frames[frame_num - 1]
    }

    pub fn frame_mut_opt(&mut self, frame_num: usize) -> Option<&mut Frame> {
        if !(1..=10).contains(&frame_num) {
            return None;
        }

        Some(&mut self.frames[frame_num - 1])
    }

    pub fn frames_mut(&mut self) -> &[Frame] {
        &mut self.frames
    }

    // Type-specific
    pub fn is_valid(&self) -> bool {
        if self.date() > &Local::now().date_naive() {
            return false;
        }

        for (frame_num, frame) in (1..=10).zip(self.frames.iter()) {
            if !frame.is_valid() {
                return false;
            }

            if (1..=9).contains(&frame_num) {
                if !matches!(frame, Frame::TwoFrame(..)) {
                    return false;
                }
            } else if frame_num == 10 {
                if (matches!(frame, Frame::TwoFrame(..)) && (frame.is_strike() || frame.is_spare()))
                    || (matches!(frame, Frame::ThreeFrame(..))
                        && (!frame.is_strike() && !frame.is_spare()))
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn score(&self) -> u16 {
        let mut sum: u16 = 0;

        for (frame_num, frame) in (1..=10).zip(self.frames.iter()) {
            sum += frame.score() as u16;

            match frame_num {
                1..=8 => {
                    if frame.is_strike() {
                        if self.frame(frame_num + 1).is_strike() {
                            sum += 10
                                + match self.frame(frame_num + 2) {
                                    Frame::TwoFrame(_, t1, ..) | Frame::ThreeFrame(_, t1, ..) => {
                                        *t1 as u16
                                    }
                                    _ => 0,
                                }
                        } else {
                            sum += self.frame(frame_num + 1).score() as u16
                        }
                    } else if frame.is_spare() {
                        sum += match self.frame(frame_num + 1) {
                            Frame::TwoFrame(_, t1, ..) => *t1 as u16,
                            _ => 0,
                        }
                    }
                }
                9 => {
                    if frame.is_strike() {
                        sum += match self.frame(10) {
                            Frame::TwoFrame(_, t1, t2) | Frame::ThreeFrame(_, t1, t2, ..) => {
                                (t1 + t2) as u16
                            }
                            _ => 0,
                        }
                    } else if frame.is_spare() {
                        sum += match self.frame(10) {
                            Frame::TwoFrame(_, t1, ..) | Frame::ThreeFrame(_, t1, ..) => *t1 as u16,
                            _ => 0,
                        }
                    }
                }
                10 => {}
                _ => unreachable!(),
            }
        }

        sum
    }
}

// `From` impl
#[rustfmt::skip]
impl
    std::convert::From<(
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
    )> for Game
{
    fn from(
        value: (
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
        ),
    ) -> Self {
        let mut game = Game::build();

        *game.frame_mut(1) = Frame::from((1, value.0, value.1));
        *game.frame_mut(2) = Frame::from((2, value.2, value.3));
        *game.frame_mut(3) = Frame::from((3, value.4, value.5));
        *game.frame_mut(4) = Frame::from((4, value.6, value.7));
        *game.frame_mut(5) = Frame::from((5, value.8, value.9));
        *game.frame_mut(6) = Frame::from((6, value.10, value.11));
        *game.frame_mut(7) = Frame::from((7, value.12, value.13));
        *game.frame_mut(8) = Frame::from((8, value.14, value.15));
        *game.frame_mut(9) = Frame::from((9, value.16, value.17));
        *game.frame_mut(10) = Frame::from((10, value.18, value.19));

        game
    }
}

#[rustfmt::skip]
impl
    std::convert::From<(
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8,
        u8, u8, u8,
    )> for Game
{
    fn from(
        value: (
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8,
            u8, u8, u8,
        ),
    ) -> Self {
        let mut game = Game::build();

        *game.frame_mut(1) = Frame::from((1, value.0, value.1));
        *game.frame_mut(2) = Frame::from((2, value.2, value.3));
        *game.frame_mut(3) = Frame::from((3, value.4, value.5));
        *game.frame_mut(4) = Frame::from((4, value.6, value.7));
        *game.frame_mut(5) = Frame::from((5, value.8, value.9));
        *game.frame_mut(6) = Frame::from((6, value.10, value.11));
        *game.frame_mut(7) = Frame::from((7, value.12, value.13));
        *game.frame_mut(8) = Frame::from((8, value.14, value.15));
        *game.frame_mut(9) = Frame::from((9, value.16, value.17));
        *game.frame_mut(10) = Frame::from((10, value.18, value.19, value.20));

        game
    }
}

// `PartialEq` impl
impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.game == other.game && self.frames == other.frames
    }
}

// `FromRow` (mysql) impl
impl mysql::prelude::FromRow for Game {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let info = match (row.get(0), row.get(1)) {
            (Some(date), Some(game)) => (NaiveDate::from_value(date), u8::from_value(game)),
            _ => panic!(),
        };

        let game_info = GameInfo::from(info);
        let mut game = Game::build_with(game_info);

        for fr in 1..=9 {
            match (game.frame_mut(fr), row.get(2 * fr), row.get(2 * fr)) {
                (frame, Some(throw1), Some(throw2)) => {
                    *frame = Frame::TwoFrame(
                        fr.try_into().unwrap(),
                        u8::from_value(throw1),
                        u8::from_value(throw2),
                    );
                }
                _ => panic!(),
            }
        }

        match (game.frame_mut(10), row.get(20), row.get(21), row.get(22)) {
            (frame, Some(throw1), Some(throw2), None) => {
                *frame = Frame::TwoFrame(10, u8::from_value(throw1), u8::from_value(throw2));
            }
            (frame, Some(throw1), Some(throw2), Some(throw3)) => {
                *frame = Frame::ThreeFrame(
                    10,
                    u8::from_value(throw1),
                    u8::from_value(throw2),
                    u8::from_value(throw3),
                )
            }
            _ => panic!(),
        }

        game
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        let info = match (row.get(0), row.get(1)) {
            (Some(date), Some(game)) => (NaiveDate::from_value(date), u8::from_value(game)),
            _ => return Err(FromRowError(row)),
        };

        let game_info = GameInfo::from(info);
        let mut game = Game::build_with(game_info);

        for fr in 1..=9 {
            match (game.frame_mut(fr), row.get(2 * fr), row.get(2 * fr)) {
                (frame, Some(throw1), Some(throw2)) => {
                    *frame = Frame::TwoFrame(
                        fr.try_into().unwrap(),
                        u8::from_value(throw1),
                        u8::from_value(throw2),
                    );
                }
                _ => return Err(FromRowError(row)),
            }
        }

        match (game.frame_mut(10), row.get(20), row.get(21), row.get(22)) {
            (frame, Some(throw1), Some(throw2), None) => {
                *frame = Frame::TwoFrame(10, u8::from_value(throw1), u8::from_value(throw2));
            }
            (frame, Some(throw1), Some(throw2), Some(throw3)) => {
                *frame = Frame::ThreeFrame(
                    10,
                    u8::from_value(throw1),
                    u8::from_value(throw2),
                    u8::from_value(throw3),
                )
            }
            _ => return Err(FromRowError(row)),
        }

        Ok(game)
    }
}

// `DBInsert` (bt_util) impl
impl bt_util::database::DBInsert for Game {
    fn ins_statement(&self, table: &str) -> String {
        format!(
            "INSERT INTO {table}
            (date, game,
            f1t1, f1t2,
            f2t1, f2t2,
            f3t1, f3t2,
            f4t1, f4t2,
            f5t1, f5t2,
            f6t1, f6t2,
            f7t1, f7t2,
            f8t1, f8t2,
            f9t1, f9t2,
            f10t1, f10t2, f10t3)
            VALUES
            (:date, :game,
            :f1t1, :f1t2,
            :f2t1, :f2t2,
            :f3t1, :f3t2,
            :f4t1, :f4t2,
            :f5t1, :f5t2,
            :f6t1, :f6t2,
            :f7t1, :f7t2,
            :f8t1, :f8t2,
            :f9t1, :f9t2,
            :f10t1, :f10t2, :f10t3)"
        )
    }

    fn ins_parameter(&self) -> Params {
        params! {
            "date" => self.date(),
            "game" => self.game(),
            "f1t1" => self.frame(1).throw(1),
            "f1t2" => self.frame(1).throw(2),
            "f2t1" => self.frame(2).throw(1),
            "f2t2" => self.frame(2).throw(2),
            "f3t1" => self.frame(3).throw(1),
            "f3t2" => self.frame(3).throw(2),
            "f4t1" => self.frame(4).throw(1),
            "f4t2" => self.frame(4).throw(2),
            "f5t1" => self.frame(5).throw(1),
            "f5t2" => self.frame(5).throw(2),
            "f6t1" => self.frame(6).throw(1),
            "f6t2" => self.frame(6).throw(2),
            "f7t1" => self.frame(7).throw(1),
            "f7t2" => self.frame(7).throw(2),
            "f8t1" => self.frame(8).throw(1),
            "f8t2" => self.frame(8).throw(2),
            "f9t1" => self.frame(9).throw(1),
            "f9t2" => self.frame(9).throw(2),
            "f10t1" => self.frame(10).throw(1),
            "f10t2" => self.frame(10).throw(2),
            "f10t3" => self.frame(10).throw(3),
        }
    }
}

// `DBModify` (bt_util) impl
impl bt_util::database::DBModify for Game {
    fn mod_statement(&self, table: &str) -> String {
        format!(
            "UPDATE {table}
        SET
        f1t1=:f1t1, f1t2=:f1t2,
        f2t1=:f2t1, f2t2=:f2t2,
        f3t1=:f3t1, f3t2=:f3t2,
        f4t1=:f4t1, f4t2=:f4t2,
        f5t1=:f5t1, f5t2=:f5t2,
        f6t1=:f6t1, f6t2=:f6t2,
        f7t1=:f7t1, f7t2=:f7t2,
        f8t1=:f8t1, f8t2=:f8t2,
        f9t1=:f9t1, f9t2=:f9t2,
        f10t1=:f10t1, f10t2=:f10t2, f10t3=:f10t3
        WHERE
        (date, game)=(:date, :game)"
        )
    }

    fn mod_parameter(&self) -> Params {
        params! {
            "date" => self.date(),
            "game" => self.game(),
            "f1t1" => self.frame(1).throw(1),
            "f1t2" => self.frame(1).throw(2),
            "f2t1" => self.frame(2).throw(1),
            "f2t2" => self.frame(2).throw(2),
            "f3t1" => self.frame(3).throw(1),
            "f3t2" => self.frame(3).throw(2),
            "f4t1" => self.frame(4).throw(1),
            "f4t2" => self.frame(4).throw(2),
            "f5t1" => self.frame(5).throw(1),
            "f5t2" => self.frame(5).throw(2),
            "f6t1" => self.frame(6).throw(1),
            "f6t2" => self.frame(6).throw(2),
            "f7t1" => self.frame(7).throw(1),
            "f7t2" => self.frame(7).throw(2),
            "f8t1" => self.frame(8).throw(1),
            "f8t2" => self.frame(8).throw(2),
            "f9t1" => self.frame(9).throw(1),
            "f9t2" => self.frame(9).throw(2),
            "f10t1" => self.frame(10).throw(1),
            "f10t2" => self.frame(10).throw(2),
            "f10t3" => self.frame(10).throw(3),
        }
    }
}
