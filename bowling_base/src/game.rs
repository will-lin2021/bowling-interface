// Internal Libraries
pub mod frame;
pub mod info;

use frame::Frame;
use info::Info;

// External Libraries
use chrono::NaiveDate;
use mysql::prelude::FromRow;

#[derive(Debug, Clone)]
pub struct Game {
    info: Info,
    frames: Vec<Frame>,
}

// Constructors
impl Game {
    pub fn build() -> Self {
        Game {
            info: Info::build(),
            frames: Vec::with_capacity(10),
        }
    }

    pub fn build_with(info: Info) -> Self {
        let mut this: Game = Game {
            info,
            frames: Vec::with_capacity(10),
        };

        for frame_num in 1..=10 {
            this.frames.push(Frame::build_with(frame_num));
        }

        this
    }
}

// Getters
impl Game {
    pub fn info(&self) -> &Info {
        &self.info
    }

    pub fn frame(&self, frame_num: usize) -> Option<&Frame> {
        if !(1..=10).contains(&frame_num) {
            return None;
        }

        Some(&self.frames[frame_num - 1])
    }

    pub fn info_mut(&mut self) -> &mut Info {
        &mut self.info
    }

    pub fn frame_mut(&mut self, frame_num: usize) -> Option<&mut Frame> {
        if !(1..=10).contains(&frame_num) {
            return None;
        }

        Some(&mut self.frames[frame_num - 1])
    }
}

// Type-related Implements
impl Game {
    pub fn valid(&self) -> bool {
        // This will check if the game is valid, meaning Info field is full,
        if !self.info.full() {
            return false;
        }

        for frame in &self.frames {
            if !frame.valid() {
                return false;
            }
        }

        true
    }

    pub fn complete(&self) -> bool {
        if !self.info.full() {
            return false;
        }

        for frame in &self.frames {
            if !frame.valid() {
                return false;
            }
        }

        true
    }

    pub fn calc_score(&self) -> u16 {
        let mut sum: u16 = 0;

        for frame in &self.frames {
            sum += frame.score() as u16;

            let frame_num = frame.frame_num();

            match frame_num {
                1..=8 => {
                    if frame.strike() {
                        let next_frame = self
                            .frame(frame_num + 1)
                            .expect("Frame 2 through 8 should always exist");

                        if next_frame.strike() {
                            let next_next_frame = self
                                .frame(frame_num + 2)
                                .expect("Frame 3 through 9 should always exist");

                            sum += 10 + next_next_frame.throw(1).unwrap() as u16;
                        } else {
                            sum += next_frame.score() as u16;
                        }
                    } else if frame.spare() {
                        let next_frame = self
                            .frame(frame_num + 1)
                            .expect("Frame 2 through 8 should always exist");

                        sum += next_frame.throw(1).unwrap() as u16;
                    }
                }
                9 => {
                    if frame.strike() {
                        let next_frame = self.frame(10).expect("Frame 10 should always exist");

                        sum += next_frame.throw(1).unwrap() as u16
                            + next_frame.throw(2).unwrap() as u16;
                    } else if frame.spare() {
                        let next_frame = self.frame(10).expect("Frame 10 should always exist");

                        sum += next_frame.throw(1).unwrap() as u16;
                    }
                }
                10 => (),
                _ => unreachable!(),
            }
        }

        sum
    }
}

// PartialEq Implements
impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.info == other.info && self.frames == other.frames
    }
}

// FromRow (mysql) Implements
impl FromRow for Game {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        let mut row_iter = row.clone().unwrap().into_iter();

        let date: Option<NaiveDate> = match row_iter.next() {
            Some(mysql::Value::Date(y, m, d, _, _, _, _)) => {
                NaiveDate::from_ymd_opt(y.into(), m.into(), d.into())
            }
            _ => None,
        };
        let game: Option<u8> = match row_iter.next() {
            Some(mysql::Value::Int(g)) => u8::try_from(g).ok(),
            _ => None,
        };

        if date.is_none() && game.is_none() {
            return Err(mysql::FromRowError(row));
        }

        let mut game = Game::build_with(Info::build_with(date.unwrap(), game.unwrap()));

        for i in 1..=9 {
            let throw1 = row_iter.next();
            let throw2 = row_iter.next();

            if let (Some(mysql::Value::Int(val1)), Some(mysql::Value::Int(val2))) = (throw1, throw2)
            {
                if let Some(frame) = game.frame_mut(i) {
                    *frame =
                        Frame::from((i, u8::try_from(val1).unwrap(), u8::try_from(val2).unwrap()))
                } else {
                    return Err(mysql::FromRowError(row));
                }
            }
        }

        let throw1 = row_iter.next();
        let throw2 = row_iter.next();
        let throw3 = row_iter.next();
        if let (
            Some(mysql::Value::Int(val1)),
            Some(mysql::Value::Int(val2)),
            Some(mysql::Value::Int(val3)),
        ) = (throw1, throw2, throw3)
        {
            if let Some(frame) = game.frame_mut(10) {
                *frame = Frame::from((
                    10,
                    u8::try_from(val1).unwrap(),
                    u8::try_from(val2).unwrap(),
                    u8::try_from(val3).unwrap(),
                ))
            } else {
                return Err(mysql::FromRowError(row));
            }
        }

        Ok(game)
    }

    fn from_row(row: mysql::Row) -> Self {
        let mut row_iter = row.unwrap().into_iter();

        let date: Option<NaiveDate> = match row_iter.next() {
            Some(mysql::Value::Date(y, m, d, _, _, _, _)) => {
                NaiveDate::from_ymd_opt(y.into(), m.into(), d.into())
            }
            _ => None,
        };
        let game: Option<u8> = match row_iter.next() {
            Some(mysql::Value::Int(g)) => u8::try_from(g).ok(),
            _ => None,
        };

        let mut game = Game::build_with(Info::build_with(date.unwrap(), game.unwrap()));

        for i in 1..=9 {
            let throw1 = row_iter.next();
            let throw2 = row_iter.next();
            if let (Some(mysql::Value::Int(val1)), Some(mysql::Value::Int(val2))) = (throw1, throw2)
            {
                if let Some(frame) = game.frame_mut(i) {
                    *frame =
                        Frame::from((i, u8::try_from(val1).unwrap(), u8::try_from(val2).unwrap()))
                }
            }
        }

        let throw1 = row_iter.next();
        let throw2 = row_iter.next();
        let throw3 = row_iter.next();
        if let (
            Some(mysql::Value::Int(val1)),
            Some(mysql::Value::Int(val2)),
            Some(mysql::Value::Int(val3)),
        ) = (throw1, throw2, throw3)
        {
            if let Some(frame) = game.frame_mut(10) {
                *frame = Frame::from((
                    10,
                    u8::try_from(val1).unwrap(),
                    u8::try_from(val2).unwrap(),
                    u8::try_from(val3).unwrap(),
                ))
            }
        }

        game
    }
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_score_perfect_game() {
        let mut test = Game::build_with(Info::build_with(
            NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(),
            1,
        ));

        *test.frame_mut(1).unwrap() = Frame::from((1, 10, 0));
        *test.frame_mut(2).unwrap() = Frame::from((2, 10, 0));
        *test.frame_mut(3).unwrap() = Frame::from((3, 10, 0));
        *test.frame_mut(4).unwrap() = Frame::from((4, 10, 0));
        *test.frame_mut(5).unwrap() = Frame::from((5, 10, 0));
        *test.frame_mut(6).unwrap() = Frame::from((6, 10, 0));
        *test.frame_mut(7).unwrap() = Frame::from((7, 10, 0));
        *test.frame_mut(8).unwrap() = Frame::from((8, 10, 0));
        *test.frame_mut(9).unwrap() = Frame::from((9, 10, 0));
        *test.frame_mut(10).unwrap() = Frame::from((10, 10, 10, 10));

        assert_eq!(test.calc_score(), 300);
    }

    #[test]
    fn calc_score_spare_game() {
        let mut test = Game::build_with(Info::build_with(
            NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(),
            1,
        ));

        *test.frame_mut(1).unwrap() = Frame::from((1, 9, 1));
        *test.frame_mut(2).unwrap() = Frame::from((2, 9, 1));
        *test.frame_mut(3).unwrap() = Frame::from((3, 9, 1));
        *test.frame_mut(4).unwrap() = Frame::from((4, 9, 1));
        *test.frame_mut(5).unwrap() = Frame::from((5, 9, 1));
        *test.frame_mut(6).unwrap() = Frame::from((6, 9, 1));
        *test.frame_mut(7).unwrap() = Frame::from((7, 9, 1));
        *test.frame_mut(8).unwrap() = Frame::from((8, 9, 1));
        *test.frame_mut(9).unwrap() = Frame::from((9, 9, 1));
        *test.frame_mut(10).unwrap() = Frame::from((10, 9, 1, 10));

        assert_eq!(test.calc_score(), 191);
    }

    #[test]
    fn calc_score_real_game_1() {
        let mut test = Game::build_with(Info::build_with(
            NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(),
            1,
        ));

        *test.frame_mut(1).unwrap() = Frame::from((1, 10, 0));
        *test.frame_mut(2).unwrap() = Frame::from((2, 10, 0));
        *test.frame_mut(3).unwrap() = Frame::from((3, 6, 3));
        *test.frame_mut(4).unwrap() = Frame::from((4, 8, 1));
        *test.frame_mut(5).unwrap() = Frame::from((5, 9, 1));
        *test.frame_mut(6).unwrap() = Frame::from((6, 8, 0));
        *test.frame_mut(7).unwrap() = Frame::from((7, 9, 0));
        *test.frame_mut(8).unwrap() = Frame::from((8, 6, 2));
        *test.frame_mut(9).unwrap() = Frame::from((9, 10, 0));
        *test.frame_mut(10).unwrap() = Frame::from((10, 10, 9, 1));

        assert_eq!(test.calc_score(), 155);
    }

    #[test]
    fn calc_score_real_game_2() {
        let mut test = Game::build_with(Info::build_with(
            NaiveDate::from_ymd_opt(2003, 2, 5).unwrap(),
            1,
        ));

        *test.frame_mut(1).unwrap() = Frame::from((1, 9, 0));
        *test.frame_mut(2).unwrap() = Frame::from((2, 9, 0));
        *test.frame_mut(3).unwrap() = Frame::from((3, 0, 7));
        *test.frame_mut(4).unwrap() = Frame::from((4, 8, 0));
        *test.frame_mut(5).unwrap() = Frame::from((5, 3, 3));
        *test.frame_mut(6).unwrap() = Frame::from((6, 9, 0));
        *test.frame_mut(7).unwrap() = Frame::from((7, 8, 0));
        *test.frame_mut(8).unwrap() = Frame::from((8, 10, 0));
        *test.frame_mut(9).unwrap() = Frame::from((9, 9, 1));
        *test.frame_mut(10).unwrap() = Frame::from((10, 10, 0, 9));

        assert_eq!(test.calc_score(), 115);
    }
}
