use std::collections::VecDeque;

use super::*;

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

    let calc_clean_frame_rate =
        games.iter().fold(0, |acc, g| acc + g.clean_frames()) as f32 / (games.len() * 10) as f32;

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
