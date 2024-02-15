pub mod base;
pub mod util;
pub mod database;

pub mod prelude {
    pub use super::base::{Date, Frame, Game, Games};
    pub use crate::scan;
    pub use super::database::DatabaseConn;
}

//
// Date Tests
//
#[cfg(test)]
mod date_tests {
    // TODO: Create tests for Date
    use super::base::Date;
}

//
// Frame Tests
//
#[cfg(test)]
mod frame_tests {
    // TODO: Create tests for Frame
    use super::base::Frame;

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
    // TODO: Create tests for Game
    use super::base::Game;
}

//
// Games Tests
//
#[cfg(test)]
mod games_tests {
    // TODO: Create tests for Games
    use super::prelude::*;

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
