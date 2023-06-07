// Internal Libraries

// External Libraries

#[derive(Debug, Clone)]
pub struct Frame {
    frame_num: usize,
    throw1: Option<u8>,
    throw2: Option<u8>,
    throw3: Option<u8>,
}

// Constructors
impl Frame {
    pub fn build() -> Self {
        Frame {
            frame_num: 0,
            throw1: None,
            throw2: None,
            throw3: None,
        }
    }

    pub fn build_with(frame_num: usize) -> Self {
        Frame {
            frame_num,
            throw1: None,
            throw2: None,
            throw3: None,
        }
    }
}

// From Implements
impl std::convert::From<(usize, u8, u8)> for Frame {
    fn from(item: (usize, u8, u8)) -> Self {
        Frame {
            frame_num: item.0,
            throw1: Some(item.1),
            throw2: Some(item.2),
            throw3: None,
        }
    }
}

impl std::convert::From<(usize, u8, u8, u8)> for Frame {
    fn from(item: (usize, u8, u8, u8)) -> Self {
        Frame {
            frame_num: item.0,
            throw1: Some(item.1),
            throw2: Some(item.2),
            throw3: Some(item.3),
        }
    }
}

// Getters
impl Frame {
    pub fn frame_num(&self) -> usize {
        self.frame_num
    }

    pub fn throw(&self, throw_num: usize) -> Option<u8> {
        match throw_num {
            1 => self.throw1,
            2 => self.throw2,
            3 => self.throw3,
            _ => None,
        }
    }

    pub fn frame_num_mut(&mut self) -> &mut usize {
        &mut self.frame_num
    }

    pub fn throw_mut(&mut self, throw_num: u8) -> Option<&mut u8> {
        match throw_num {
            1 => {
                if self.throw1.is_none() {
                    self.throw1 = Some(0);
                }

                self.throw1.as_mut()
            }
            2 => {
                if self.throw2.is_none() {
                    self.throw2 = Some(0);
                }

                self.throw2.as_mut()
            }
            3 => {
                if self.throw3.is_none() {
                    self.throw3 = Some(0);
                }

                self.throw3.as_mut()
            }
            _ => None,
        }
    }
}

// Type-related Implements
impl Frame {
    pub fn valid(&self) -> bool {
        match self {
            Frame {
                frame_num: 1..=10,
                throw1: None,
                throw2: None,
                throw3: None,
            } => true,
            Frame {
                frame_num: 1..=9,
                throw1: Some(val1),
                throw2: Some(val2),
                throw3: None,
            } => 10 >= val1 + val2,
            Frame {
                frame_num: 10,
                throw1: Some(val1),
                throw2: Some(val2),
                throw3: None,
            } => 10 > val1 + val2,
            Frame {
                frame_num: 10,
                throw1: Some(val1),
                throw2: Some(val2),
                throw3: Some(val3),
            } => !(val1 + val2 < 10 || 30 < val1 + val2 + val3),
            _ => false,
        }
    }

    pub fn complete(&self) -> bool {
        match self {
            Frame {
                frame_num: 1..=9,
                throw1: Some(_),
                throw2: Some(_),
                throw3: None,
            } => true,
            Frame {
                frame_num: 10,
                throw1: Some(t1),
                throw2: Some(t2),
                throw3: None,
            } => t1 + t2 < 10,
            Frame {
                frame_num: 10,
                throw1: Some(_),
                throw2: Some(_),
                throw3: Some(_),
            } => true,
            _ => false,
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Frame {
                frame_num: 1..=9,
                throw1: Some(throw1),
                throw2: Some(throw2),
                throw3: None,
            } => throw1 + throw2,
            Frame {
                frame_num: 10,
                throw1: Some(throw1),
                throw2: Some(throw2),
                throw3: None,
            } => throw1 + throw2,
            Frame {
                frame_num: 10,
                throw1: Some(throw1),
                throw2: Some(throw2),
                throw3: Some(throw3),
            } => throw1 + throw2 + throw3,
            _ => 0,
        }
    }

    pub fn strike(&self) -> bool {
        matches!(&self.throw(1), Some(10))
    }

    pub fn spare(&self) -> bool {
        match (self.throw(1), self.throw(2)) {
            (Some(throw1), Some(throw2)) => throw1 + throw2 == 10,
            _ => false,
        }
    }
}

// PartialEq Implements
impl std::cmp::PartialEq for Frame {
    fn eq(&self, other: &Frame) -> bool {
        self.frame_num == other.frame_num
            && self.throw1 == other.throw1
            && self.throw2 == other.throw2
            && self.throw3 == other.throw3
    }
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_valid() {
        // Empty frame is valid
        let empty = Frame::build_with(1);

        // Frame 1-9: Strike
        let strike = Frame::from((1, 10, 0));

        // Frame 1-9: Spare
        let spare = Frame::from((1, 5, 5));

        // Frame 1-9: Bad
        let terrible = Frame::from((1, 0, 0));

        // Frame 10: Triple
        let triple_strike = Frame::from((10, 10, 10, 10));

        // Frame 10: Double
        let double_strike = Frame::from((10, 10, 10, 0));

        // Frame 10: Spare + Strike
        let spare_then_strike = Frame::from((10, 5, 5, 10));

        assert!(empty.valid());
        assert!(strike.valid());
        assert!(spare.valid());
        assert!(terrible.valid());
        assert!(triple_strike.valid());
        assert!(double_strike.valid());
        assert!(spare_then_strike.valid());
    }

    #[test]
    fn valid_invalid() {
        // Score more than max (10)
        let more_than_max = Frame::from((1, 10, 1));

        // Can't have 3rd throw in Frame 1-9
        let frame_1_9_throw_3 = Frame::from((1, 10, 10, 10));

        // Score more than max (30)
        let triple_more_than_max = Frame::from((10, 10, 10, 11));

        // Throw 3 is not possible
        let no_3rd_throw = Frame::from((10, 1, 1, 1));

        // Throw 3 is not possible
        let another_no_3rd_throw = Frame::from((10, 5, 4, 4));

        assert!(!more_than_max.valid());
        assert!(!frame_1_9_throw_3.valid());
        assert!(!triple_more_than_max.valid());
        assert!(!no_3rd_throw.valid());
        assert!(!another_no_3rd_throw.valid());
    }

    #[test]
    fn complete_complete() {}

    #[test]
    fn complete_incomplete() {}
}
