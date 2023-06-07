// Internal Libraries

// External Libraries
use chrono::Local;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Info {
    date: NaiveDate,
    game_num: Option<u8>,
}

// Constructors
impl Info {
    pub fn build() -> Self {
        Info {
            date: Local::now().date_naive(),
            game_num: None,
        }
    }

    pub fn build_with(date: NaiveDate, game_num: u8) -> Self {
        if game_num == 0 {
            Info {
                date,
                game_num: None,
            }
        } else {
            Info {
                date,
                game_num: Some(game_num),
            }
        }
    }
}

// Getters
impl Info {
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn game_num(&self) -> Option<u8> {
        self.game_num
    }

    pub fn date_mut(&mut self) -> &mut NaiveDate {
        &mut self.date
    }

    pub fn game_num_mut(&mut self) -> &mut Option<u8> {
        &mut self.game_num
    }
}

// Type-related Implements
impl Info {
    pub fn full(&self) -> bool {
        matches!(
            self,
            Info {
                date: _,
                game_num: Some(_),
            }
        )
    }
}

// PartialEq Implements
impl std::cmp::PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.game_num == other.game_num
    }
}

// From Implements
impl std::convert::From<NaiveDate> for Info {
    fn from(item: NaiveDate) -> Self {
        Info {
            date: item,
            game_num: None,
        }
    }
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_full_info() {
        let test = Info {
            date: Local::now().date_naive(),
            game_num: Some(1),
        };

        assert!(test.full());
    }

    #[test]
    fn full_partial_info() {
        let test = Info {
            date: Local::now().date_naive(),
            game_num: None,
        };

        assert!(!test.full());
    }
}
