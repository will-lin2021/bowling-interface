use std::cmp::Ordering;

use super::Frame;

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
