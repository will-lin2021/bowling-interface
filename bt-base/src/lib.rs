pub mod types;

//
// Game Info Tests
//
#[cfg(test)]
mod game_info_tests {
    use super::types::GameInfo;

    use std::cmp::Ordering;

    use chrono::{Duration, Local, NaiveDate};

    #[test]
    fn build() {
        let info = GameInfo::build();

        assert!(matches!(info, GameInfo::None), "Expected `GameInfo::None`");
    }

    #[test]
    fn build_date() {
        let date = Local::now().date_naive();

        let info = GameInfo::build_date(date.clone());

        assert!(
            matches!(info, GameInfo::Partial(..)),
            "Expected `GameInfo::Partial`"
        );
        assert_eq!(info.date(), &date, "Expected `{}` as date", &date);
    }

    #[test]
    fn build_full() {
        let date = Local::now().date_naive();
        let num = 21;

        let info = GameInfo::build_with(date.clone(), num);

        assert!(
            matches!(info, GameInfo::Full(..)),
            "Expected `GameInfo::Full`"
        );
        assert_eq!(info.date(), &date, "Expected `{}` as date", &date);
        assert_eq!(info.game(), num, "Expected `{}` as game number", num);
    }

    #[test]
    fn date() {
        let date = Local::now().date_naive();

        let info_date = GameInfo::build_date(date.clone());
        assert_eq!(info_date.date(), &date, "Expected `{}`", &date);

        let info_full = GameInfo::build_with(date.clone(), 69);
        assert_eq!(info_full.date(), &date, "Expected `{}`", &date);
    }

    #[test]
    fn date_mut() {
        let date1 = NaiveDate::from_ymd_opt(2000, 01, 01).unwrap();
        let date2 = Local::now().date_naive();

        let mut info_date = GameInfo::build_date(date1.clone());
        *info_date.date_mut() = date2.clone();
        assert_eq!(info_date.date(), &date2, "Expected `{}`", &date2);

        let mut info_full = GameInfo::build_with(date1.clone(), 69);
        *info_full.date_mut() = date2.clone();
        assert_eq!(info_date.date(), &date2, "Expected `{}`", &date2);
    }

    #[test]
    fn game() {
        let num = 69;

        let info_full = GameInfo::build_with(Local::now().date_naive(), num);
        assert_eq!(info_full.game(), num, "Expected `{}`", num)
    }

    #[test]
    fn game_mut() {
        let num1 = 69;
        let num2 = 1;

        let mut info_full = GameInfo::build_with(Local::now().date_naive(), num1);
        *info_full.game_mut() = num2;
        assert_eq!(info_full.game(), num2, "Expected {}", num2);
    }

    #[test]
    fn eq() {
        let none = GameInfo::build();
        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert_eq!(none, none);
        assert_ne!(none, part);
        assert_ne!(none, full);
        assert_eq!(part, part);
        assert_ne!(part, full);
        assert_eq!(full, full)
    }

    #[test]
    fn partial_cmp() {
        let none = GameInfo::build();
        let part = GameInfo::build_date(Local::now().date_naive());
        let full = GameInfo::build_with(Local::now().date_naive(), 1);

        assert_eq!(none.partial_cmp(&part), None);
        assert_eq!(none.partial_cmp(&full), None);
        assert_eq!(part.partial_cmp(&full), None);

        let today = GameInfo::build_date(Local::now().date_naive());
        let today2 = GameInfo::build_date(Local::now().date_naive());
        let yesterday = GameInfo::build_date(Local::now().date_naive() - Duration::days(1));
        let tomorrow = GameInfo::build_date(Local::now().date_naive() + Duration::days(1));

        assert_eq!(
            today.partial_cmp(&today2),
            Some(Ordering::Equal),
            "Expected `Ordering::Equal`"
        );
        assert_eq!(
            today.partial_cmp(&yesterday),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`"
        );
        assert_eq!(
            today.partial_cmp(&tomorrow),
            Some(Ordering::Less),
            "Expected `Ordering::Less`"
        );

        let today = GameInfo::build_with(Local::now().date_naive(), 2);
        let todayg2 = GameInfo::build_with(Local::now().date_naive(), 2);
        let todayg1 = GameInfo::build_with(Local::now().date_naive(), 1);
        let yesterdayg2 = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 2);
        let yesterdayg1 = GameInfo::build_with(Local::now().date_naive() - Duration::days(1), 1);
        let tomorrowg2 = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 2);
        let tomorrowg1 = GameInfo::build_with(Local::now().date_naive() + Duration::days(1), 1);

        assert_eq!(
            today.partial_cmp(&todayg2),
            Some(Ordering::Equal),
            "Expected `Ordering::Equal`: {} == {} and {} == {}",
            today.date(),
            today2.date(),
            today.game(),
            today2.game()
        );
        assert_eq!(
            today.partial_cmp(&todayg1),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`: {} == {}, but {} > {}",
            today.date(),
            todayg1.date(),
            today.game(),
            todayg1.game()
        );
        assert_eq!(
            today.partial_cmp(&yesterdayg2),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`: {} > {}",
            today.date(),
            yesterdayg2.date()
        );
        assert_eq!(
            today.partial_cmp(&yesterdayg1),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`: {} > {}",
            today.date(),
            yesterdayg1.date()
        );
        assert_eq!(
            yesterdayg2.partial_cmp(&yesterdayg1),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`: {} == {}, but {} > {}",
            yesterdayg2.date(),
            yesterdayg1.date(),
            yesterdayg2.game(),
            yesterdayg1.game()
        );
        assert_eq!(
            today.partial_cmp(&tomorrowg2),
            Some(Ordering::Less),
            "Expected `Ordering::Less`: {} < {}",
            today.date(),
            tomorrowg2.date()
        );
        assert_eq!(
            today.partial_cmp(&tomorrowg1),
            Some(Ordering::Less),
            "Expected `Ordering::Less`: {} < {}",
            today.date(),
            tomorrowg1.date()
        );
        assert_eq!(
            tomorrowg2.partial_cmp(&tomorrowg1),
            Some(Ordering::Greater),
            "Expected `Ordering::Greater`: {} == {}, but {} > {}",
            tomorrowg2.date(),
            tomorrowg1.date(),
            tomorrowg2.game(),
            tomorrowg1.game()
        );
    }

    #[test]
    fn from_naive_date() {
        let date = Local::now().date_naive();

        let part = GameInfo::from(date);

        assert!(
            matches!(part, GameInfo::Partial(..)),
            "Expected `GameInfo::Partial`"
        );
        assert_eq!(part.date(), &date, "Expected `{}` as date", &date);
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
    fn build() {}

    #[test]
    fn frame() {}

    #[test]
    fn frame_mut() {}

    #[test]
    fn is_valid() {}

    #[test]
    fn score() {}

    #[test]
    fn is_strike() {}

    #[test]
    fn is_spare() {}

    #[test]
    fn from_two_tuple() {}

    #[test]
    fn from_three_tuple() {}

    #[test]
    fn eq() {}

    #[test]
    fn partial_cmp() {}
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
    fn build() {}

    #[test]
    fn build_date() {}

    #[test]
    fn build_with() {}

    #[test]
    fn date() {}

    #[test]
    fn frame() {}

    #[test]
    fn frame_opt() {}

    #[test]
    fn frames() {}

    #[test]
    fn frames_as_vec() {}

    #[test]
    fn date_mut() {}

    #[test]
    fn game_mut() {}

    #[test]
    fn frame_mut() {}

    #[test]
    fn frames_mut_opt() {}

    #[test]
    fn frames_mut() {}

    #[test]
    fn is_valid() {}

    #[test]
    fn score() {}
}
