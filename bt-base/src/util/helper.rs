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

    pub fn get_opt(&self) -> Option<&str> {
        self.option.as_deref()
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
        // m-d-Y
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        // m/d/Y
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // Y-m-d
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
        // Y/m/d
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(
        // m-d
        format!("{}-{}", date_str, Date::build().year()).as_str(),
        "%m-%d-%Y",
    ) {
        Some(date.into())
    } else if let Ok(date) = NaiveDate::parse_from_str(
        // m/d
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

// Parses &str into a vector of scores, if anything is not a number or greater than 10, return None
// Guarantees that the output vector is full of possible scores, values between 0-10
pub fn parse_scores(input_str: &str) -> Option<Vec<u8>> {
    let temp: Vec<String> = input_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let parsed = temp.iter().map(|s| s.parse::<u8>());

    let mut scores = Vec::new();
    for score in parsed {
        if score.is_err() {
            // Not a number
            return None;
        } else {
            let score = score.unwrap();

            if score > 10 {
                // Not a valid score
                return None;
            }

            scores.push(score);
        }
    }

    // If input_str represents a strike '10'
    if scores.len() == 1 && scores[0] == 10 {
        scores.push(0);
    }

    Some(scores)
}
