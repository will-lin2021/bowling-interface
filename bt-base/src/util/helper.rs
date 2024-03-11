use crate::base::Date;

use std::io::stdin;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct MenuOption {
    option: Option<String>,
    args: Vec<String>,
}

impl MenuOption {
    pub fn is_empty(&self) -> bool {
        self.option.is_none()
    }

    pub fn get_opt(&self) -> &str {
        self.option.as_ref().unwrap()
    }

    pub fn get_args(&self) -> &[String] {
        &self.args
    }
}

// TODO: Documetation
#[macro_export]
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ), + ) => {
        {
            let mut iter = $string.split($sep);

            ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
        }
    };
}

pub fn get_user_input() -> String {
    let mut temp = String::new();
    stdin().read_line(&mut temp).ok();

    temp.trim().to_string()
}

pub fn parse_date(date_str: &str) -> Option<Date> {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m-%d-%Y") {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(
        format!("{}-{}", date_str, Date::build().year()).as_str(),
        "%m-%d-%Y",
    ) {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(
        format!("{}/{}", date_str, Date::build().year()).as_str(),
        "%m/%d/%Y",
    ) {
        Some(date.into())
    } else {
        None
    }
}

pub fn parse_options(input_str: &str) -> MenuOption {
    let mut option_iter = input_str.split_whitespace().map(|token| token.to_string());

    MenuOption {
        option: option_iter.next(),
        args: option_iter.collect(),
    }
}

pub fn parse_scores(input_str: &str) -> Vec<u8> {
    input_str
        .split_whitespace()
        .map(|s| s.parse())
        .take_while(|r| r.is_ok())
        .map(|n| n.unwrap())
        .take_while(|u| *u <= 10)
        .collect()
}
