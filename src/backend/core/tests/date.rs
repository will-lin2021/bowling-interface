use chrono::{Datelike, Local, NaiveDate};
use std::cmp::Ordering;

use super::Date;

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
