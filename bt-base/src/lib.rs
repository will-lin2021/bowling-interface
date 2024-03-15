pub mod base;
pub mod database;
pub mod util;

pub mod prelude {
    pub use crate::scan;
    pub use crate::{
        base::{Date, Frame, Game, Games},
        database::DatabaseConn,
        util::helper::{get_user_input, parse_date, parse_options, parse_scores, MenuOption},
    };
}

//
// Date Tests
//
#[cfg(test)]
mod date_tests {
    use super::base::Date;

    use chrono::{Datelike, Local, NaiveDate};
    use std::cmp::Ordering;

    #[test]
    fn build() {
        let today = Local::now();

        let test = Date::build();

        assert!(test.year() == today.year() as u16);
        assert!(test.month() == today.month() as u8);
        assert!(test.day() == today.day() as u8);
    }

    #[test]
    fn build_with() {
        let date = NaiveDate::from_ymd_opt(2024, 02, 05).unwrap();

        let test = Date::build_with(2024, 02, 05);

        assert!(test.year() == date.year() as u16);
        assert!(test.month() == date.month() as u8);
        assert!(test.day() == date.day() as u8);
    }

    #[test]
    fn from_tuple() {
        let tuple = (2024, 02, 05);
        let date = NaiveDate::from_ymd_opt(2024, 02, 05).unwrap();

        let test_from = Date::from(tuple);

        assert!(test_from.year() == date.year() as u16);
        assert!(test_from.month() == date.month() as u8);
        assert!(test_from.day() == date.day() as u8);
    }

    #[test]
    fn from_naivedate() {
        let date = NaiveDate::from_ymd_opt(2024, 02, 05).unwrap();

        let test_from = Date::from(date);

        assert!(test_from.year() == date.year() as u16);
        assert!(test_from.month() == date.month() as u8);
        assert!(test_from.day() == date.day() as u8);
    }

    #[test]
    fn partialeq() {
        let date = Date::build_with(2024, 02, 05);

        let yesterday = Date::build_with(date.year(), date.month(), date.day() - 1);

        let tomorrow = Date::build_with(date.year(), date.month(), date.day() + 1);

        assert_eq!(date, date);

        assert_ne!(date, yesterday);
        assert_ne!(yesterday, date);

        assert_ne!(date, tomorrow);
        assert_ne!(tomorrow, date);

        assert_ne!(yesterday, tomorrow);
    }

    #[test]
    fn partialord_past() {
        let date = Date::build_with(2024, 02, 05);

        let last_year = Date::build_with(date.year() - 1, date.month(), date.day());
        let last_month = Date::build_with(date.year(), date.month() - 1, date.day());
        let yesterday = Date::build_with(date.year(), date.month(), date.day() - 1);

        let tomorrow = Date::build_with(date.year(), date.month(), date.day() + 1);
        let next_month = Date::build_with(date.year(), date.month() + 1, date.day());
        let next_year = Date::build_with(date.year() + 1, date.month(), date.day());

        assert_eq!(date.partial_cmp(&last_year), Some(Ordering::Greater));
        assert_eq!(date.partial_cmp(&last_month), Some(Ordering::Greater));
        assert_eq!(date.partial_cmp(&yesterday), Some(Ordering::Greater));

        assert_eq!(date.partial_cmp(&date), Some(Ordering::Equal));

        assert_eq!(date.partial_cmp(&tomorrow), Some(Ordering::Less));
        assert_eq!(date.partial_cmp(&next_month), Some(Ordering::Less));
        assert_eq!(date.partial_cmp(&next_year), Some(Ordering::Less));

        assert_eq!(last_year.partial_cmp(&last_year), Some(Ordering::Equal));
        assert_eq!(last_year.partial_cmp(&last_month), Some(Ordering::Less));
        assert_eq!(last_year.partial_cmp(&yesterday), Some(Ordering::Less));

        assert_eq!(last_month.partial_cmp(&last_year), Some(Ordering::Greater));
        assert_eq!(last_month.partial_cmp(&last_month), Some(Ordering::Equal));
        assert_eq!(last_month.partial_cmp(&yesterday), Some(Ordering::Less));

        assert_eq!(yesterday.partial_cmp(&last_year), Some(Ordering::Greater));
        assert_eq!(yesterday.partial_cmp(&last_month), Some(Ordering::Greater));
        assert_eq!(yesterday.partial_cmp(&yesterday), Some(Ordering::Equal));

        assert_eq!(tomorrow.partial_cmp(&tomorrow), Some(Ordering::Equal));
        assert_eq!(tomorrow.partial_cmp(&next_month), Some(Ordering::Less));
        assert_eq!(tomorrow.partial_cmp(&next_year), Some(Ordering::Less));

        assert_eq!(next_month.partial_cmp(&tomorrow), Some(Ordering::Greater));
        assert_eq!(next_month.partial_cmp(&next_month), Some(Ordering::Equal));
        assert_eq!(next_month.partial_cmp(&next_year), Some(Ordering::Less));

        assert_eq!(next_year.partial_cmp(&tomorrow), Some(Ordering::Greater));
        assert_eq!(next_year.partial_cmp(&next_month), Some(Ordering::Greater));
        assert_eq!(next_year.partial_cmp(&next_year), Some(Ordering::Equal));
    }
}

//
// Frame Tests
//
#[cfg(test)]
mod frame_tests {
    use super::base::Frame;

    use std::cmp::Ordering;

    #[test]
    fn build() {
        let test = Frame::build();

        assert_eq!(test, Frame::Uninit);
    }

    #[test]
    fn is_valid_uninit() {
        let uninit = Frame::Uninit;

        assert!(!uninit.is_valid());
    }

    #[test]
    fn is_valid_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                assert!(
                    test.is_valid(),
                    "Frame with ({}, {}) should be valid",
                    i,
                    10 - i
                );
            }
        }
    }

    #[test]
    fn is_valid_two_edge() {
        let edge1 = Frame::TwoFrame(11, 0);
        let edge2 = Frame::TwoFrame(0, 11);
        let edge3 = Frame::TwoFrame(10, 1);
        let edge4 = Frame::TwoFrame(1, 10);

        assert!(!edge1.is_valid());
        assert!(!edge2.is_valid());
        assert!(!edge3.is_valid());
        assert!(!edge4.is_valid());
    }

    #[test]
    fn is_valid_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert!(test.is_valid());
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                assert!(test.is_valid());
            }
        }

        for i in 0..=10 {
            let test = Frame::ThreeFrame(i, 10 - i, 10);

            assert!(test.is_valid());
        }

        for i in 0..=9 {
            for j in 0..=9 - i {
                for k in 0..=10 {
                    let test = Frame::ThreeFrame(i, j, k);

                    assert!(!test.is_valid());
                }
            }
        }
    }

    #[test]
    fn is_valid_three_edge() {
        let edge1 = Frame::ThreeFrame(11, 0, 10);
        let edge2 = Frame::ThreeFrame(0, 11, 10);
        let edge3 = Frame::ThreeFrame(10, 0, 11);
        let edge4 = Frame::ThreeFrame(0, 10, 11);

        assert!(!edge1.is_valid());
        assert!(!edge2.is_valid());
        assert!(!edge3.is_valid());
        assert!(!edge4.is_valid());
    }

    // TODO: Make tests for is_valid_no

    #[test]
    fn score_unit() {
        let uninit = Frame::Uninit;

        assert_eq!(uninit.score(), 0);
    }

    #[test]
    fn score_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                assert_eq!(test.score(), i + j);
            }
        }
    }

    #[test]
    fn score_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert_eq!(test.score(), 20 + i);
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                assert_eq!(test.score(), 10 + i + j);
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                assert_eq!(test.score(), 10 + j);
            }
        }
    }

    #[test]
    fn is_strike_uninit() {
        let uninit = Frame::Uninit;

        assert!(!uninit.is_strike());
    }

    #[test]
    fn is_strike_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                if i == 10 {
                    assert!(test.is_strike());
                } else {
                    assert!(!test.is_strike());
                }
            }
        }
    }

    #[test]
    fn is_strike_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert!(test.is_strike());
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                assert!(test.is_strike());
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                if i == 10 {
                    assert!(test.is_strike());
                } else {
                    assert!(!test.is_strike());
                }
            }
        }
    }

    #[test]
    fn is_spare_uninit() {
        let uninit = Frame::Uninit;

        assert!(!uninit.is_spare());
    }

    #[test]
    fn is_spare_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                if i == 10 {
                    assert!(!test.is_spare());
                } else if i + j == 10 {
                    assert!(test.is_spare());
                } else {
                    assert!(!test.is_spare());
                }
            }
        }
    }

    #[test]
    fn is_spare_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert!(!test.is_spare());
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                assert!(!test.is_spare());
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                if i == 10 {
                    assert!(!test.is_spare());
                } else {
                    assert!(test.is_spare())
                }
            }
        }
    }

    #[test]
    fn num_strikes_uninit() {
        let uninit = Frame::Uninit;

        assert_eq!(uninit.num_strikes(), 0);
    }

    #[test]
    fn num_strikes_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                if i == 10 {
                    assert_eq!(test.num_strikes(), 1);
                } else {
                    assert_eq!(test.num_strikes(), 0);
                }
            }
        }
    }

    #[test]
    fn num_strikes_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            if i == 10 {
                assert_eq!(test.num_strikes(), 3);
            } else {
                assert_eq!(test.num_strikes(), 2);
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                if i == 10 {
                    assert_eq!(test.num_strikes(), 2);
                } else {
                    assert_eq!(test.num_strikes(), 1);
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                if i == 10 {
                    assert_eq!(test.num_strikes(), 1);
                } else if j == 10 {
                    assert_eq!(test.num_strikes(), 1);
                } else {
                    assert_eq!(test.num_strikes(), 0);
                }
            }
        }
    }

    #[test]
    fn num_spares_uninit() {
        let uninit = Frame::Uninit;

        assert_eq!(uninit.num_spares(), 0);
    }

    #[test]
    fn num_spares_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                if i != 10 && i + j == 10 {
                    assert_eq!(test.num_spares(), 1);
                } else {
                    assert_eq!(test.num_spares(), 0);
                }
            }
        }
    }

    #[test]
    fn num_spares_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert_eq!(test.num_spares(), 0);
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                if i != 10 && i + j == 10 {
                    assert_eq!(test.num_spares(), 1);
                } else {
                    assert_eq!(test.num_spares(), 0);
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                if i == 10 {
                    if j == 10 {
                        assert_eq!(test.num_spares(), 1);
                    } else {
                        assert_eq!(test.num_spares(), 0);
                    }
                } else {
                    assert_eq!(test.num_spares(), 1);
                }
            }
        }
    }

    #[test]
    fn strike_chances_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                assert_eq!(test.strike_chances(), 1);
            }
        }
    }

    #[test]
    fn strike_chances_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert_eq!(test.strike_chances(), 3);
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                if i == 10 {
                    assert_eq!(test.strike_chances(), 3);
                } else {
                    assert_eq!(test.strike_chances(), 2);
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                if i == 10 {
                    assert_eq!(test.strike_chances(), 2);
                } else {
                    assert_eq!(test.strike_chances(), 1);
                }
            }
        }
    }

    #[test]
    fn spare_chance_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::TwoFrame(i, j);

                if i == 10 {
                    assert_eq!(test.spare_chances(), 0);
                } else {
                    assert_eq!(test.spare_chances(), 1);
                }
            }
        }
    }

    #[test]
    fn spare_chance_three() {
        for i in 0..=10 {
            let test = Frame::ThreeFrame(10, 10, i);

            assert_eq!(test.spare_chances(), 0);
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test = Frame::ThreeFrame(10, i, j);

                if i == 10 {
                    assert_eq!(test.spare_chances(), 0);
                } else {
                    assert_eq!(test.spare_chances(), 1);
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let test = Frame::ThreeFrame(i, 10 - i, j);

                assert_eq!(test.spare_chances(), 1);
            }
        }
    }

    #[test]
    fn from_two_tuple() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let tuple = (i, j);

                let test = Frame::from(tuple);

                assert!(matches!(test, Frame::TwoFrame(t1, t2) if (t1, t2) == tuple));
            }
        }
    }

    #[test]
    fn from_three_tuple() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let tuple = (10, i, j);

                let test = Frame::from(tuple);

                assert!(matches!(test, Frame::ThreeFrame(t1, t2, t3) if (t1, t2, t3) == tuple));
            }
        }

        for i in 0..=10 {
            for j in 0..=10 {
                let tuple = (i, 10 - i, j);

                let test = Frame::from(tuple);

                assert!(matches!(test, Frame::ThreeFrame(t1, t2, t3) if (t1, t2, t3) == tuple));
            }
        }
    }

    #[test]
    fn partialeq() {
        let test_tuple = (5, 5);

        let test: Frame = test_tuple.into();

        let same: Frame = test_tuple.into();
        let diff = Frame::TwoFrame(10, 0);

        assert_eq!(test, same);
        assert_ne!(test, diff);
    }

    #[test]
    fn partialord() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                let test1 = Frame::TwoFrame(i, j);

                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test2 = Frame::TwoFrame(k, l);

                        if i + j < k + l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Less)));
                        } else if i + j > k + l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Greater)));
                        } else {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Equal)));
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            let test1 = Frame::ThreeFrame(10, 10, i);

            for j in 0..=10 {
                let test2 = Frame::ThreeFrame(10, 10, j);

                if i < j {
                    assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Less)));
                } else if i > j {
                    assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Greater)));
                } else {
                    assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Equal)));
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test1 = Frame::ThreeFrame(10, i, j);

                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test2 = Frame::ThreeFrame(10, k, l);

                        if i + j < k + l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Less)));
                        } else if i + j > k + l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Greater)));
                        } else {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Equal)));
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                let test1 = Frame::ThreeFrame(i, 10 - i, j);

                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test2 = Frame::ThreeFrame(k, 10 - k, l);

                        if j < l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Less)));
                        } else if j > l {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Greater)));
                        } else {
                            assert!(matches!(test1.partial_cmp(&test2), Some(Ordering::Equal)));
                        }
                    }
                }
            }
        }
    }
}

//
// Game Tests
//
#[cfg(test)]
mod game_tests {
    use super::base::Frame;
    use super::base::Game;

    use std::{cmp::Ordering, collections::HashMap};

    fn sample_games() -> HashMap<u16, Game> {
        let mut games: HashMap<u16, Game> = HashMap::new();

        games.insert(
            300,
            Game::build_with(
                u8::MAX,
                (1..=10)
                    .map(|n| {
                        if n != 10 {
                            Frame::TwoFrame(10, 0)
                        } else {
                            Frame::ThreeFrame(10, 10, 10)
                        }
                    })
                    .collect(),
            ),
        );
        games.insert(
            200,
            Game::build_with(
                200,
                (1..=10)
                    .map(|n| {
                        if n != 10 && n % 2 == 1 {
                            Frame::TwoFrame(10, 0)
                        } else if n != 10 && n % 2 == 0 {
                            Frame::TwoFrame(9, 1)
                        } else {
                            Frame::ThreeFrame(9, 1, 10)
                        }
                    })
                    .collect(),
            ),
        );
        games.insert(
            191,
            Game::build_with(
                191,
                (1..=10)
                    .map(|n| {
                        if n != 10 {
                            Frame::TwoFrame(9, 1)
                        } else {
                            Frame::ThreeFrame(9, 1, 10)
                        }
                    })
                    .collect(),
            ),
        );
        games.insert(
            150,
            Game::build_with(
                150,
                (1..=10)
                    .map(|n| {
                        if n != 10 {
                            Frame::TwoFrame(5, 5)
                        } else {
                            Frame::ThreeFrame(5, 5, 5)
                        }
                    })
                    .collect(),
            ),
        );
        games.insert(
            80,
            Game::build_with(
                80,
                (1..=10)
                    .map(|n| {
                        if n != 10 {
                            Frame::TwoFrame(4, 4)
                        } else {
                            Frame::TwoFrame(4, 4)
                        }
                    })
                    .collect(),
            ),
        );
        games.insert(
            40,
            Game::build_with(40, (1..=10).map(|_| Frame::TwoFrame(2, 2)).collect()),
        );
        games.insert(
            20,
            Game::build_with(40, (1..=10).map(|_| Frame::TwoFrame(1, 1)).collect()),
        );
        games.insert(
            0,
            Game::build_with(0, (1..=10).map(|_| Frame::TwoFrame(0, 0)).collect()),
        );

        games
    }

    #[test]
    fn build() {
        let test = Game::build();

        assert_eq!(test.game_num(), 1);
        assert_eq!(test.frames().len(), 10);
        test.frames()
            .iter()
            .for_each(|f| assert!(matches!(f, Frame::Uninit)));
    }

    #[test]
    fn build_with() {
        let frames: Vec<Frame> = (1..=10)
            .map(|n| {
                if n != 10 {
                    Frame::TwoFrame(10, 0)
                } else {
                    Frame::ThreeFrame(10, 10, 10)
                }
            })
            .collect();

        let frames_clone = frames.clone();

        let test = Game::build_with(10, frames);

        assert_eq!(test.game_num(), 10);
        assert_eq!(test.frames().len(), 10);
        assert_eq!(test.frames(), frames_clone)
    }

    #[test]
    #[should_panic]
    fn build_with_fail() {
        let frames: Vec<Frame> = (1..=11).map(|_| Frame::Uninit).collect();

        let _ = Game::build_with(10, frames);
    }

    #[test]
    fn game_num_mut() {
        let mut test = Game::build_with(
            10,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(10, 0)
                    } else {
                        Frame::ThreeFrame(10, 10, 10)
                    }
                })
                .collect(),
        );

        assert_eq!(test.game_num(), 10);

        *test.game_num_mut() = 1;

        assert_eq!(test.game_num(), 1);
    }

    #[test]
    fn frames_mut() {
        let mut test = Game::build_with(
            10,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(10, 0)
                    } else {
                        Frame::ThreeFrame(10, 10, 10)
                    }
                })
                .collect(),
        );

        assert!(matches!(test.frames()[4], Frame::TwoFrame(10, 0)));

        test.frames_mut()[4] = Frame::TwoFrame(5, 5);

        assert!(matches!(test.frames()[4], Frame::TwoFrame(5, 5)));
    }

    #[test]
    fn is_valid_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert!(test.is_valid());
                    }
                }
            }
        }
    }

    #[test]
    fn is_valid_two_edge() {
        // TODO: Test is_valid TwoFrame edge cases
        // Test invalid two frames
        // Pin Count sums above 10
        // Strike or Spare in 10th frame
    }

    #[test]
    fn is_valid_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert!(test.is_valid());
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert!(test.is_valid());
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert!(test.is_valid());
                    }
                }
            }
        }
    }

    #[test]
    fn is_valid_three_edge() {
        // TODO: Test is_valid ThreeFrame edge cases
        // Test invalid third frames
        // No strike or spares
    }

    #[test]
    fn score() {
        let games = sample_games();

        for key in games.keys() {
            assert_eq!(games[key].score(), *key);
        }
    }

    // TODO: Make test for score_n

    #[test]
    fn pin_count_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let mut sum = 0;

                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        sum += k + l;
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        sum += i + j;
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(test.pin_count(), sum);
                    }
                }
            }
        }
    }

    #[test]
    fn pin_count_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let mut sum = 0;

                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    sum += 20 + k;
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    sum += i + j;
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(test.pin_count(), sum);
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let mut sum = 0;

                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        sum += 10 + k + l;
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        sum += i + j;
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(test.pin_count(), sum);
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let mut sum = 0;

                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        sum += 10 + l;
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        sum += i + j;
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(test.pin_count(), sum);
                    }
                }
            }
        }
    }

    #[test]
    fn num_strikes_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i == 10 {
                            assert_eq!(test.num_strikes(), 9);
                        } else {
                            assert_eq!(test.num_strikes(), 0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn num_strikes_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i == 10 {
                        if k == 10 {
                            assert_eq!(test.num_strikes(), 12);
                        } else {
                            assert_eq!(test.num_strikes(), 11);
                        }
                    } else {
                        if k == 10 {
                            assert_eq!(test.num_strikes(), 3);
                        } else {
                            assert_eq!(test.num_strikes(), 2);
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i == 10 {
                            if k == 10 {
                                assert_eq!(test.num_strikes(), 11);
                            } else {
                                assert_eq!(test.num_strikes(), 10);
                            }
                        } else {
                            if k == 10 {
                                assert_eq!(test.num_strikes(), 2);
                            } else {
                                assert_eq!(test.num_strikes(), 1);
                            }
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i == 10 {
                            if k == 10 || l == 10 {
                                assert_eq!(test.num_strikes(), 10);
                            } else {
                                assert_eq!(test.num_strikes(), 9);
                            }
                        } else {
                            if k == 10 || l == 10 {
                                assert_eq!(test.num_strikes(), 1);
                            } else {
                                assert_eq!(test.num_strikes(), 0, "{} {} {} {}", i, j, k, l);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn num_spares_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i != 10 && i + j == 10 {
                            assert_eq!(test.num_spares(), 9);
                        } else {
                            assert_eq!(test.num_spares(), 0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn num_spares_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i != 10 && i + j == 10 {
                        assert_eq!(test.num_spares(), 9);
                    } else {
                        assert_eq!(test.num_spares(), 0);
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i != 10 && i + j == 10 {
                            if k != 10 && k + l == 10 {
                                assert_eq!(test.num_spares(), 10);
                            } else {
                                assert_eq!(test.num_spares(), 9);
                            }
                        } else {
                            if k != 10 && k + l == 10 {
                                assert_eq!(test.num_spares(), 1);
                            } else {
                                assert_eq!(test.num_spares(), 0);
                            }
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i != 10 && i + j == 10 {
                            if k != 10 || l == 10 {
                                assert_eq!(test.num_spares(), 10);
                            } else {
                                assert_eq!(test.num_spares(), 9);
                            }
                        } else {
                            if k != 10 || l == 10 {
                                assert_eq!(test.num_spares(), 1);
                            } else {
                                assert_eq!(test.num_spares(), 0);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn open_frames_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i + j == 10 {
                            assert_eq!(test.open_frames(), 1);
                        } else {
                            assert_eq!(test.open_frames(), 10);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn open_frames_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i + j == 10 {
                        assert_eq!(test.open_frames(), 0);
                    } else {
                        assert_eq!(test.open_frames(), 9);
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i + j == 10 {
                            assert_eq!(test.open_frames(), 0);
                        } else {
                            assert_eq!(test.open_frames(), 9);
                        }
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        if i + j == 10 {
                            assert_eq!(test.open_frames(), 0);
                        } else {
                            assert_eq!(test.open_frames(), 9);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn avg_first_ball_pinfall_two() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=9 {
                    for l in 0..=9 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::TwoFrame(k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(
                            test.avg_first_ball_pinfall(),
                            ((i * 9 + k) as f32) / (10 as f32)
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn avg_first_ball_pinfall_three() {
        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, 10, k)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(
                        test.avg_first_ball_pinfall(),
                        ((i * 9 + 10) as f32) / (10 as f32)
                    )
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 - k {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(10, k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(
                            test.avg_first_ball_pinfall(),
                            ((i * 9 + 10) as f32) / (10 as f32)
                        )
                    }
                }
            }
        }

        for i in 0..=10 {
            for j in 0..=10 - i {
                for k in 0..=10 {
                    for l in 0..=10 {
                        let test = Game::build_with(
                            1,
                            (1..=10)
                                .map(|n| {
                                    if n == 10 {
                                        Frame::ThreeFrame(k, 10 - k, l)
                                    } else {
                                        Frame::TwoFrame(i, j)
                                    }
                                })
                                .collect(),
                        );

                        assert_eq!(
                            test.avg_first_ball_pinfall(),
                            ((i * 9 + k) as f32) / (10 as f32)
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn partialeq() {
        let games = sample_games();

        for k1 in games.keys() {
            for k2 in games.keys() {
                if k1 == k2 {
                    assert_eq!(games[k1], games[k2]);
                } else {
                    assert_ne!(games[k1], games[k2]);
                }
            }
        }
    }

    #[test]
    fn partialord() {
        let games = sample_games();

        for k1 in games.keys() {
            for k2 in games.keys() {
                if k1 < k2 {
                    assert!(matches!(
                        games[k1].partial_cmp(&games[k2]),
                        Some(Ordering::Less)
                    ));
                } else if k1 > k2 {
                    assert!(matches!(
                        games[k1].partial_cmp(&games[k2]),
                        Some(Ordering::Greater)
                    ));
                } else {
                    assert!(matches!(
                        games[k1].partial_cmp(&games[k2]),
                        Some(Ordering::Equal)
                    ));
                }
            }
        }
    }
}

//
// Games Tests
//
#[cfg(test)]
mod games_tests {
    use super::prelude::*;

    use std::collections::VecDeque;

    fn sample_games() -> VecDeque<Game> {
        let mut games = VecDeque::new();

        games.push_back(Game::build_with(
            1,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(10, 0)
                    } else {
                        Frame::ThreeFrame(10, 10, 10)
                    }
                })
                .collect(),
        ));
        games.push_back(Game::build_with(
            2,
            (1..=10)
                .map(|n| {
                    if n != 10 && n % 2 == 1 {
                        Frame::TwoFrame(10, 0)
                    } else if n != 10 && n % 2 == 0 {
                        Frame::TwoFrame(9, 1)
                    } else {
                        Frame::ThreeFrame(9, 1, 10)
                    }
                })
                .collect(),
        ));
        games.push_back(Game::build_with(
            3,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(9, 1)
                    } else {
                        Frame::ThreeFrame(9, 1, 10)
                    }
                })
                .collect(),
        ));
        games.push_back(Game::build_with(
            4,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(5, 5)
                    } else {
                        Frame::ThreeFrame(5, 5, 5)
                    }
                })
                .collect(),
        ));
        games.push_back(Game::build_with(
            5,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(4, 4)
                    } else {
                        Frame::TwoFrame(4, 4)
                    }
                })
                .collect(),
        ));
        games.push_back(Game::build_with(
            6,
            (1..=10).map(|_| Frame::TwoFrame(2, 2)).collect(),
        ));
        games.push_back(Game::build_with(
            7,
            (1..=10).map(|_| Frame::TwoFrame(0, 0)).collect(),
        ));

        games
    }

    #[test]
    fn build() {
        let today = Date::build();

        let test = Games::build();

        assert!(test.date().year() == today.year());
        assert!(test.date().month() == today.month());
        assert!(test.date().day() == today.day());

        assert_eq!(test.games().len(), 0);
    }

    #[test]
    fn build_with() {
        let date = Date::build_with(2024, 2, 5);

        let mut games = sample_games();

        let test = Games::build_with(date, games.pop_front().unwrap());

        assert!(test.date().year() == date.year());
        assert!(test.date().month() == date.month());
        assert!(test.date().day() == date.day());

        assert_eq!(test.games().len(), 1);
    }

    #[test]
    fn build_from_vec() {
        let date = Date::build_with(2024, 2, 5);

        let games = sample_games();

        let test = Games::build_from_vec(date, games.into());

        assert!(test.date().year() == date.year());
        assert!(test.date().month() == date.month());
        assert!(test.date().day() == date.day());

        assert_eq!(test.games().len(), 7);
    }

    #[test]
    fn games_mut() {
        let games = sample_games();

        let mut test = Games::build_from_vec(Date::build(), games.into());

        assert!(matches!(test.games()[6].frames()[5], Frame::TwoFrame(0, 0)));

        let game = test.games_mut();

        game[6].frames_mut()[5] = Frame::TwoFrame(5, 5);

        assert!(matches!(test.games()[6].frames()[5], Frame::TwoFrame(5, 5)));
    }

    #[test]
    fn add_game() {
        let mut test = Games::build();

        let mut games = sample_games();
        let mut games_duplicate = sample_games();

        for _ in 0..games.len() {
            let temp = games.pop_front().unwrap();

            test.add_game(temp);
        }

        for (i, g) in (1..).zip(test.games().iter()) {
            assert_eq!(g.game_num(), i);

            let temp = games_duplicate.pop_front().unwrap();

            assert_eq!(*g, temp);
        }
    }

    #[test]
    fn average() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let avg = test.average();

        let games = sample_games();

        let calc_avg = games.iter().fold(0, |acc, g| acc + g.score()) as f32 / games.len() as f32;

        assert_eq!(avg, calc_avg);
    }

    #[test]
    fn strike_rate() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let strike_rate = test.strike_rate();

        let games = sample_games();

        let calc_strike_rate = games.iter().fold(0, |acc, g| acc + g.num_strikes()) as f32
            / games.iter().fold(0, |acc, g| acc + g.strike_chances()) as f32;

        println!("{}", strike_rate);

        assert_eq!(strike_rate, calc_strike_rate)
    }

    #[test]
    fn spare_rate() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let spare_rate = test.spare_rate();

        let games = sample_games();

        let calc_spare_rate = games.iter().fold(0, |acc, g| acc + g.num_spares()) as f32
            / games.iter().fold(0, |acc, g| acc + g.spare_chances()) as f32;

        println!("{}", spare_rate);

        assert_eq!(spare_rate, calc_spare_rate)
    }

    #[test]
    fn open_frame_rate() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let open_frame_rate = test.open_frame_rate();

        let games = sample_games();

        let calc_open_frame_rate =
            games.iter().fold(0, |acc, g| acc + g.open_frames()) as f32 / (games.len() * 10) as f32;

        println!("{}", open_frame_rate);

        assert_eq!(open_frame_rate, calc_open_frame_rate)
    }

    #[test]
    fn clean_frames_rate() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let clean_frame_rate = test.clean_frame_rate();

        let games = sample_games();

        let calc_clean_frame_rate = games.iter().fold(0, |acc, g| acc + g.clean_frames()) as f32
            / (games.len() * 10) as f32;

        println!("{}", clean_frame_rate);

        assert_eq!(clean_frame_rate, calc_clean_frame_rate)
    }

    #[test]
    fn avg_first_ball_pinfall() {
        let test = Games::build_from_vec(Date::build(), sample_games().into());

        let avg = test.avg_first_ball_pinfall();

        let games = sample_games();

        let calc_sum_of_avgs: f32 = games
            .iter()
            .map(|g| g.avg_first_ball_pinfall())
            .collect::<Vec<f32>>()
            .iter()
            .sum();

        let calc_avg = calc_sum_of_avgs / games.len() as f32;

        println!("{}", avg);

        assert_eq!(avg, calc_avg);
    }
}

#[cfg(test)]
mod db_conn_tests {
    use super::prelude::DatabaseConn;

    use std::{collections::HashMap, env};

    use dotenvy::dotenv;

    #[test]
    fn serial_tests() {
        dotenv().expect(".env not found");

        let mut conn_info: HashMap<&str, String> = HashMap::new();

        conn_info.insert("user", env::var("TEST_NAME").unwrap());
        conn_info.insert("pass", env::var("TEST_PASS").unwrap());
        conn_info.insert("auth", env::var("TEST_AUTH").unwrap());
        conn_info.insert("host", env::var("DB_HOST").unwrap());
        conn_info.insert("port", env::var("DB_PORT").unwrap());
        conn_info.insert("db", env::var("TEST_DB_NAME").unwrap());
        conn_info.insert("coll", env::var("TEST_COLL_NAME").unwrap());

        connect(&conn_info);
        connect_db(&conn_info);

        let db_conn = connect_full(&conn_info);

        set_database(&db_conn);
        unset_database(&db_conn);
        set_collection(&db_conn);
        unset_collection(&db_conn);

        add_game(&db_conn);
        add_games(&db_conn);
        get_game(&db_conn);
        get_games(&db_conn);
        num_games(&db_conn);
        modify_game(&db_conn);
        modify_games(&db_conn);
        remove_game(&db_conn);
        remove_games(&db_conn);
    }

    fn connect(conn_info: &HashMap<&str, String>) -> DatabaseConn {
        DatabaseConn::connect(
            conn_info.get("user").unwrap(),
            conn_info.get("pass").unwrap(),
            conn_info.get("host").unwrap(),
            conn_info.get("port").unwrap(),
            conn_info.get("auth").unwrap(),
        )
    }

    fn connect_db(conn_info: &HashMap<&str, String>) -> DatabaseConn {
        DatabaseConn::connect_db(
            conn_info.get("user").unwrap(),
            conn_info.get("pass").unwrap(),
            conn_info.get("host").unwrap(),
            conn_info.get("port").unwrap(),
            conn_info.get("auth").unwrap(),
            conn_info.get("db").unwrap(),
        )
    }

    fn connect_full(conn_info: &HashMap<&str, String>) -> DatabaseConn {
        DatabaseConn::connect_full(
            conn_info.get("user").unwrap(),
            conn_info.get("pass").unwrap(),
            conn_info.get("host").unwrap(),
            conn_info.get("port").unwrap(),
            conn_info.get("auth").unwrap(),
            conn_info.get("db").unwrap(),
            conn_info.get("coll").unwrap(),
        )
    }

    fn set_database(db_conn: &DatabaseConn) {
        todo!()
    }

    fn unset_database(db_conn: &DatabaseConn) {
        todo!()
    }

    fn set_collection(db_conn: &DatabaseConn) {
        todo!()
    }

    fn unset_collection(db_conn: &DatabaseConn) {
        todo!()
    }

    fn add_game(db_conn: &DatabaseConn) {
        todo!()
    }

    fn add_games(db_conn: &DatabaseConn) {
        todo!()
    }

    fn get_game(db_conn: &DatabaseConn) {
        todo!()
    }

    fn get_games(db_conn: &DatabaseConn) {
        todo!()
    }

    fn num_games(db_conn: &DatabaseConn) {
        todo!()
    }

    fn modify_game(db_conn: &DatabaseConn) {
        todo!()
    }

    fn modify_games(db_conn: &DatabaseConn) {
        todo!()
    }

    fn remove_game(db_conn: &DatabaseConn) {
        todo!()
    }

    fn remove_games(db_conn: &DatabaseConn) {
        todo!()
    }

    fn drop_all(db_conn: &DatabaseConn) {
        todo!()
    }
}
