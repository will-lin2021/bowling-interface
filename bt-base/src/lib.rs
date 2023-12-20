pub mod types;

//
// Game Info Tests
//
#[cfg(test)]
mod game_info_tests {
    // TODO: Create more elaborate tests
    use super::types::GameInfo;

    use chrono::{Duration, Local, NaiveDate};

    #[test]
    fn build() {
        let info = GameInfo::build();

        assert!(
            matches!(info, GameInfo::None),
            "build() doesn't initialize GameInfo::None"
        );
    }

    #[test]
    fn build_date() {
        let info = GameInfo::build_date(Local::now().date_naive());

        assert!(
            matches!(info, GameInfo::Partial(..)),
            "build_date() not creating GameInfo::Partial"
        );
        assert_eq!(
            info.date(),
            &Local::now().date_naive(),
            "build_date() not initializing correct date"
        );
    }

    #[test]
    fn build_full() {
        let info = GameInfo::build_with(Local::now().date_naive(), 21);

        assert!(
            matches!(info, GameInfo::Full(..)),
            "build_with() not creating GameInfo::Full"
        );
        assert_eq!(
            info.date(),
            &Local::now().date_naive(),
            "build_with() not initializing correct date"
        );
        assert_eq!(
            info.game(),
            21,
            "build_with() not initializing correct game number"
        );
    }

    #[test]
    fn date() {
        let info_date = GameInfo::build_date(Local::now().date_naive());
        assert_eq!(
            info_date.date(),
            &Local::now().date_naive(),
            "date() not getting date for GameInfo::Partial"
        );

        let info_full = GameInfo::build_with(Local::now().date_naive(), 69);
        assert_eq!(
            info_full.date(),
            &Local::now().date_naive(),
            "date() not getting date for GameInfo::Full"
        );
    }

    #[test]
    fn date_mut() {
        let mut info_date = GameInfo::build_date(NaiveDate::from_ymd_opt(2000, 01, 01).unwrap());
        *info_date.date_mut() = Local::now().date_naive();
        assert_eq!(
            info_date.date(),
            &Local::now().date_naive(),
            "date_mut() mutable reference doesn't work for GameInfo::Partial"
        );

        let mut info_full =
            GameInfo::build_with(NaiveDate::from_ymd_opt(2000, 01, 01).unwrap(), 69);
        *info_full.date_mut() = Local::now().date_naive();
        assert_eq!(
            info_date.date(),
            &Local::now().date_naive(),
            "date_mut() mutable reference doesn't work for GameInfo::Full"
        );
    }

    #[test]
    fn game() {
        let info_full = GameInfo::build_with(Local::now().date_naive(), 69);
        assert_eq!(
            info_full.game(),
            69,
            "game() not getting game number for GameInfo::Full"
        )
    }

    #[test]
    fn game_mut() {
        let mut info_full = GameInfo::build_with(Local::now().date_naive(), 69);
        *info_full.game_mut() = 1;
        assert_eq!(
            info_full.game(),
            1,
            "game_mut() mutable reference doesn't work for GameInfo::Full"
        );
    }

    #[test]
    fn partial_ord_ge() {
        assert!(
            GameInfo::build() >= GameInfo::build(),
            "GameInfo::None == GameInfo::None"
        );

        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert!(!(part >= full), "GameInfo::Partial != GameInfo::Full");

        let main = GameInfo::build_date(Local::now().date_naive());
        let dupe = GameInfo::build_date(Local::now().date_naive());
        let less = GameInfo::build_date(Local::now().date_naive() - Duration::days(1));
        let more = GameInfo::build_date(Local::now().date_naive() + Duration::days(1));

        assert!(main >= dupe, "{} >= {}", main.date(), dupe.date());
        assert!(main >= less, "{} >= {}", main.date(), less.date());
        assert!(!(main >= more), "{} not >= {}", main.date(), more.date());

        let main = GameInfo::build_with(Local::now().date_naive(), 2);
        let dupe = GameInfo::build_with(Local::now().date_naive(), 2);
        let less_date = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 2);
        let less_game = GameInfo::build_with(Local::now().date_naive(), 1);
        let more_date = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 2);
        let more_game = GameInfo::build_with(Local::now().date_naive(), 3);

        assert!(main >= dupe, "{} == {}", main.date(), dupe.date());
        assert!(main >= less_date, "{} >= {}", main.date(), less_date.date());
        assert!(main >= less_game, "{} >= {}", main.game(), less_game.game());
        assert!(
            !(main >= more_date),
            "{} not >= {}",
            main.game(),
            more_date.date()
        );
        assert!(
            !(main >= more_game),
            "{} not >= {}",
            main.game(),
            more_game.game()
        );
    }

    #[test]
    fn partial_ord_gt() {
        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert!(!(part > full));

        let main = GameInfo::build_date(Local::now().date_naive());
        let dupe = GameInfo::build_date(Local::now().date_naive());
        let less = GameInfo::build_date(Local::now().date_naive() - Duration::days(1));
        let more = GameInfo::build_date(Local::now().date_naive() + Duration::days(1));

        assert!(!(main > dupe));
        assert!(main > less);
        assert!(!(main > more));

        let main = GameInfo::build_with(Local::now().date_naive(), 2);
        let dupe = GameInfo::build_with(Local::now().date_naive(), 2);
        let less_date = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 2);
        let less_game = GameInfo::build_with(Local::now().date_naive(), 1);
        let more_date = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 2);
        let more_game = GameInfo::build_with(Local::now().date_naive(), 3);

        assert!(!(main > dupe));
        assert!(main > less_date);
        assert!(main > less_game);
        assert!(!(main > more_date));
        assert!(!(main > more_game));
    }

    #[test]
    fn partial_ord_le() {
        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert!(!(part <= full));

        let main = GameInfo::build_date(Local::now().date_naive());
        let dupe = GameInfo::build_date(Local::now().date_naive());
        let less = GameInfo::build_date(Local::now().date_naive() - Duration::days(1));
        let more = GameInfo::build_date(Local::now().date_naive() + Duration::days(1));

        assert!(main <= dupe);
        assert!(!(main <= less));
        assert!(main <= more);

        let main = GameInfo::build_with(Local::now().date_naive(), 2);
        let dupe = GameInfo::build_with(Local::now().date_naive(), 2);
        let less_date = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 2);
        let less_game = GameInfo::build_with(Local::now().date_naive(), 1);
        let more_date = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 2);
        let more_game = GameInfo::build_with(Local::now().date_naive(), 3);

        assert!(main <= dupe);
        assert!(!(main <= less_date));
        assert!(!(main <= less_game));
        assert!(main <= more_date);
        assert!(main <= more_game);
    }

    #[test]
    fn partial_ord_lt() {
        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert!(!(part < full));

        let main = GameInfo::build_date(Local::now().date_naive());
        let dupe = GameInfo::build_date(Local::now().date_naive());
        let less = GameInfo::build_date(Local::now().date_naive() - Duration::days(1));
        let more = GameInfo::build_date(Local::now().date_naive() + Duration::days(1));

        assert!(!(main < dupe));
        assert!(!(main < less));
        assert!(main < more);

        let main = GameInfo::build_with(Local::now().date_naive(), 2);
        let dupe = GameInfo::build_with(Local::now().date_naive(), 2);
        let less_date = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 2);
        let less_game = GameInfo::build_with(Local::now().date_naive(), 1);
        let more_date = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 2);
        let more_game = GameInfo::build_with(Local::now().date_naive(), 3);

        assert!(!(main < dupe));
        assert!(!(main < less_date));
        assert!(!(main < less_game));
        assert!(main < more_date);
        assert!(main < more_game);
    }

    #[test]
    fn partial_ord_partial_cmp() {
        todo!()
    }

    #[test]
    fn info_from_naive() {
        todo!()
    }
}

//
// Frame Tests
//
#[cfg(test)]
mod frame_tests {
    // TODO: Create more elaborate tests
    use super::types::Frame;

    #[test]
    fn frame_create_def() {
        let frame = Frame::build();

        assert!(matches!(frame, Frame::Uninit));
    }

    #[test]
    fn frame_accessor() {
        let uninit_frame = Frame::Uninit;
        let two_frame = Frame::TwoFrame(1, 5, 5);
        let three_frame = Frame::ThreeFrame(10, 10, 10, 10);

        assert_eq!(uninit_frame.frame(), &uninit_frame);
        assert_eq!(two_frame.frame(), &two_frame);
        assert_eq!(three_frame.frame(), &three_frame);
    }

    #[test]
    fn frame_accessor_mut() {
        let mut frame = Frame::Uninit;

        assert!(matches!(frame, Frame::Uninit));

        *frame.frame_mut() = Frame::TwoFrame(1, 5, 5);

        assert!(matches!(frame, Frame::TwoFrame(..)));

        *frame.frame_mut() = Frame::ThreeFrame(10, 10, 10, 10);

        assert!(matches!(frame, Frame::ThreeFrame(..)));
    }

    #[test]
    fn frame_partialeq() {
        let two_frame = Frame::TwoFrame(1, 5, 5);
        let two_frame_copy = Frame::TwoFrame(1, 5, 5);
        let two_frame_diff = Frame::TwoFrame(5, 2, 8);
        let three_frame = Frame::ThreeFrame(10, 10, 10, 10);
        let three_frame_copy = Frame::ThreeFrame(10, 10, 10, 10);
        let three_frame_diff = Frame::ThreeFrame(10, 10, 10, 5);

        assert_ne!(two_frame, Frame::Uninit);
        assert_ne!(three_frame, Frame::Uninit);

        assert_eq!(two_frame, two_frame_copy);
        assert_ne!(two_frame, two_frame_diff);
        assert_ne!(two_frame, three_frame);
        assert_ne!(two_frame, three_frame_copy);
        assert_ne!(two_frame, three_frame_diff);

        assert_eq!(three_frame, three_frame_copy);
        assert_ne!(three_frame, three_frame_diff);
        assert_ne!(three_frame, two_frame);
        assert_ne!(three_frame, two_frame_copy);
        assert_ne!(three_frame, two_frame_diff);
    }

    #[test]
    fn frame_from_twotuple() {
        let frame = Frame::from((1, 5, 5));

        assert!(matches!(frame, Frame::TwoFrame(..)));
        assert_eq!(frame, Frame::TwoFrame(1, 5, 5));
    }

    #[test]
    fn frame_from_threetuple() {
        let frame = Frame::from((10, 10, 10, 10));

        assert!(matches!(frame, Frame::ThreeFrame(..)));
        assert_eq!(frame, Frame::ThreeFrame(10, 10, 10, 10));
    }

    #[test]
    fn frame_is_valid_valid() {
        let frame_1 = Frame::TwoFrame(1, 0, 0);
        let frame_2 = Frame::TwoFrame(5, 1, 1);
        let frame_3 = Frame::TwoFrame(10, 5, 5);
        let frame_4 = Frame::TwoFrame(10, 10, 0);

        assert!(frame_1.is_valid());
        assert!(frame_2.is_valid());
        assert!(frame_3.is_valid());
        assert!(frame_4.is_valid());

        let frame_1 = Frame::ThreeFrame(10, 10, 10, 10);
        let frame_2 = Frame::ThreeFrame(10, 5, 5, 10);
        let frame_3 = Frame::ThreeFrame(10, 1, 9, 5);
        let frame_4 = Frame::ThreeFrame(10, 0, 10, 0);

        assert!(frame_1.is_valid());
        assert!(frame_2.is_valid());
        assert!(frame_3.is_valid());
        assert!(frame_4.is_valid());
    }

    #[test]
    fn frame_is_valid_invalid() {
        let frame1 = Frame::Uninit;
        let frame2 = Frame::TwoFrame(1, 10, 1);
        let frame3 = Frame::TwoFrame(0, 5, 6);
        let frame4 = Frame::TwoFrame(11, 5, 6);

        assert!(!frame1.is_valid());
        assert!(!frame2.is_valid());
        assert!(!frame3.is_valid());
        assert!(!frame4.is_valid());

        let frame1 = Frame::ThreeFrame(1, 0, 0, 10);
        let frame2 = Frame::ThreeFrame(10, 5, 4, 5);
        let frame3 = Frame::ThreeFrame(10, 10, 10, 11);
        let frame4 = Frame::ThreeFrame(0, 10, 10, 10);
        let frame5 = Frame::ThreeFrame(11, 10, 10, 10);

        assert!(!frame1.is_valid());
        assert!(!frame2.is_valid());
        assert!(!frame3.is_valid());
        assert!(!frame4.is_valid());
        assert!(!frame5.is_valid());
    }

    #[test]
    fn frame_score() {
        let uninit_frame = Frame::Uninit;

        assert_eq!(uninit_frame.score(), 0);

        let frame1 = Frame::TwoFrame(1, 0, 0);
        let frame2 = Frame::TwoFrame(1, 1, 1);
        let frame3 = Frame::TwoFrame(1, 5, 5);
        let frame4 = Frame::TwoFrame(1, 10, 0);
        let frame5 = Frame::TwoFrame(1, 0, 10);

        assert_eq!(frame1.score(), 0);
        assert_eq!(frame2.score(), 2);
        assert_eq!(frame3.score(), 10);
        assert_eq!(frame4.score(), 10);
        assert_eq!(frame5.score(), 10);

        let frame1 = Frame::ThreeFrame(10, 10, 0, 0);
        let frame2 = Frame::ThreeFrame(10, 5, 5, 10);
        let frame3 = Frame::ThreeFrame(10, 10, 10, 10);
        let frame4 = Frame::ThreeFrame(10, 0, 10, 10);
        let frame5 = Frame::ThreeFrame(10, 5, 5, 5);

        assert_eq!(frame1.score(), 10);
        assert_eq!(frame2.score(), 20);
        assert_eq!(frame3.score(), 30);
        assert_eq!(frame4.score(), 20);
        assert_eq!(frame5.score(), 15);
    }

    #[test]
    fn frame_is_strike() {
        let strike = Frame::TwoFrame(1, 10, 0);
        let spare1 = Frame::TwoFrame(1, 0, 10);
        let spare2 = Frame::TwoFrame(1, 5, 5);
        let normal = Frame::TwoFrame(1, 5, 0);

        assert!(strike.is_strike());
        assert!(!spare1.is_strike());
        assert!(!spare2.is_strike());
        assert!(!normal.is_strike());

        let strike1 = Frame::ThreeFrame(10, 10, 10, 10);
        let strike2 = Frame::ThreeFrame(10, 10, 0, 0);
        let spare1 = Frame::ThreeFrame(10, 0, 10, 5);
        let spare2 = Frame::ThreeFrame(10, 5, 5, 5);

        assert!(strike1.is_strike());
        assert!(strike2.is_strike());
        assert!(!spare1.is_strike());
        assert!(!spare2.is_strike());
    }

    #[test]
    fn frame_is_spare() {
        let strike = Frame::TwoFrame(1, 10, 0);
        let spare1 = Frame::TwoFrame(1, 0, 10);
        let spare2 = Frame::TwoFrame(1, 5, 5);
        let normal = Frame::TwoFrame(1, 0, 0);

        assert!(!strike.is_spare());
        assert!(spare1.is_spare());
        assert!(spare2.is_spare());
        assert!(!normal.is_spare());

        let strike = Frame::ThreeFrame(10, 10, 0, 0);
        let spare1 = Frame::ThreeFrame(10, 0, 10, 0);
        let spare2 = Frame::ThreeFrame(10, 5, 5, 0);

        assert!(!strike.is_spare());
        assert!(spare1.is_spare());
        assert!(spare2.is_spare());
    }
}

//
// Game Tests
//
#[cfg(test)]
mod game_tests {
    // TODO: Create more elaborate tests
    use super::types::{Frame, Game, GameInfo};

    use chrono::{Local, NaiveDate};

    #[test]
    fn game_create_def() {
        let game = Game::build();

        assert_eq!(game.date(), &Local::now().date_naive());
        assert_eq!(game.game(), 1);
        assert_eq!(game.frames().len(), 10);
    }

    #[test]
    fn game_create_date() {
        let game = Game::build_date(Local::now().date_naive());

        assert_eq!(game.date(), &Local::now().date_naive());
        assert_eq!(game.game(), 1);
        assert_eq!(game.frames().len(), 10);
    }

    #[test]
    fn game_create_info() {
        let info = GameInfo::Partial(NaiveDate::from_ymd_opt(2020, 2, 5).unwrap());

        let game = Game::build_with(info);

        assert_eq!(game.date(), &NaiveDate::from_ymd_opt(2020, 2, 5).unwrap());
        assert_eq!(game.game(), 1);
        assert_eq!(game.frames().len(), 10);

        let info = GameInfo::Full(NaiveDate::from_ymd_opt(2020, 2, 10).unwrap(), 5);

        let game = Game::build_with(info);
        assert_eq!(game.date(), &NaiveDate::from_ymd_opt(2020, 2, 10).unwrap());
        assert_eq!(game.game(), 5);
        assert_eq!(game.frames().len(), 10);
    }

    #[test]
    fn game_accessor() {
        let game = Game::build();

        assert_eq!(game.date(), &Local::now().date_naive());
        assert_eq!(game.game(), 1);
        assert_eq!(game.frames().len(), 10);

        assert_eq!(game.frame_opt(1), Some(&Frame::Uninit));
        assert_eq!(game.frame_opt(5), Some(&Frame::Uninit));
        assert_eq!(game.frame_opt(10), Some(&Frame::Uninit));

        assert_eq!(game.frame_opt(0), None);
        assert_eq!(game.frame_opt(11), None);
        assert_eq!(game.frame_opt(15), None);
    }

    #[test]
    fn game_accessor_mut() {
        let mut game = Game::build();

        *game.date_mut() = NaiveDate::from_ymd_opt(2020, 2, 5).unwrap();
        assert_eq!(game.date(), &NaiveDate::from_ymd_opt(2020, 2, 5).unwrap());

        *game.game_mut() = 10;
        assert_eq!(game.game(), 10);

        if let Some(frame) = game.frame_mut_opt(1) {
            *frame = Frame::TwoFrame(1, 10, 0);
        }
        if let Some(frame) = game.frame_mut_opt(5) {
            *frame = Frame::TwoFrame(5, 5, 5);
        }
        if let Some(frame) = game.frame_mut_opt(10) {
            *frame = Frame::ThreeFrame(10, 10, 10, 10);
        }
        assert_eq!(game.frame_opt(1), Some(&Frame::TwoFrame(1, 10, 0)));
        assert_eq!(game.frame_opt(5), Some(&Frame::TwoFrame(5, 5, 5)));
        assert_eq!(game.frame_opt(10), Some(&Frame::ThreeFrame(10, 10, 10, 10)));

        assert_eq!(game.frame_mut_opt(0), None);
        assert_eq!(game.frame_mut_opt(11), None);
        assert_eq!(game.frame_mut_opt(15), None);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_allstrikes() {
        let game = Game::from((
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 10, 10,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 300);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_moststrikes() {
        let game = Game::from((
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            5, 5, 10,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 275);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_moststrikesspare() {
        let game = Game::from((
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            5, 5, 10,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 275);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_moststrikeschoke() {
        let game = Game::from((
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            10, 0,
            5, 4,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 263);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_91spares() {
        let game = Game::from((
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1,
            9, 1, 10,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 191);
    }

    #[rustfmt::skip]
    #[test]
    fn game_valid_and_score_55spares() {
        let game = Game::from((
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5,
            5, 5, 5,
        ));

        assert!(game.is_valid());
        assert_eq!(game.score(), 150);
    }
}
