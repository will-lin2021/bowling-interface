use std::{cmp::Ordering, collections::HashMap};

use super::{Frame, Game};

fn sample_games() -> HashMap<u16, Game> {
    let mut games: HashMap<u16, Game> = HashMap::new();

    games.insert(
        300,
        Game::build_with(
            u8::MAX,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(10, 0)
                    } else {
                        Frame::ThreeFrame(10, 10, 10)
                    }
                })
                .collect(),
        ),
    );
    games.insert(
        200,
        Game::build_with(
            200,
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
        ),
    );
    games.insert(
        191,
        Game::build_with(
            191,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(9, 1)
                    } else {
                        Frame::ThreeFrame(9, 1, 10)
                    }
                })
                .collect(),
        ),
    );
    games.insert(
        150,
        Game::build_with(
            150,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(5, 5)
                    } else {
                        Frame::ThreeFrame(5, 5, 5)
                    }
                })
                .collect(),
        ),
    );
    games.insert(
        80,
        Game::build_with(
            80,
            (1..=10)
                .map(|n| {
                    if n != 10 {
                        Frame::TwoFrame(4, 4)
                    } else {
                        Frame::TwoFrame(4, 4)
                    }
                })
                .collect(),
        ),
    );
    games.insert(
        40,
        Game::build_with(40, (1..=10).map(|_| Frame::TwoFrame(2, 2)).collect()),
    );
    games.insert(
        20,
        Game::build_with(40, (1..=10).map(|_| Frame::TwoFrame(1, 1)).collect()),
    );
    games.insert(
        0,
        Game::build_with(0, (1..=10).map(|_| Frame::TwoFrame(0, 0)).collect()),
    );

    games
}

#[test]
fn build() {
    let test = Game::build(1);

    assert_eq!(test.game_num(), 1);
    assert_eq!(test.frames().len(), 10);
    test.frames()
        .iter()
        .for_each(|f| assert!(matches!(f, Frame::Uninit)));
}

#[test]
fn build_with() {
    let frames: Vec<Frame> = (1..=10)
        .map(|n| {
            if n != 10 {
                Frame::TwoFrame(10, 0)
            } else {
                Frame::ThreeFrame(10, 10, 10)
            }
        })
        .collect();

    let frames_clone = frames.clone();

    let test = Game::build_with(10, frames);

    assert_eq!(test.game_num(), 10);
    assert_eq!(test.frames().len(), 10);
    assert_eq!(test.frames(), frames_clone)
}

#[test]
#[should_panic]
fn build_with_fail() {
    let frames: Vec<Frame> = (1..=11).map(|_| Frame::Uninit).collect();

    let _ = Game::build_with(10, frames);
}

#[test]
fn game_num_mut() {
    let mut test = Game::build_with(
        10,
        (1..=10)
            .map(|n| {
                if n != 10 {
                    Frame::TwoFrame(10, 0)
                } else {
                    Frame::ThreeFrame(10, 10, 10)
                }
            })
            .collect(),
    );

    assert_eq!(test.game_num(), 10);

    *test.game_num_mut() = 1;

    assert_eq!(test.game_num(), 1);
}

#[test]
fn frames_mut() {
    let mut test = Game::build_with(
        10,
        (1..=10)
            .map(|n| {
                if n != 10 {
                    Frame::TwoFrame(10, 0)
                } else {
                    Frame::ThreeFrame(10, 10, 10)
                }
            })
            .collect(),
    );

    assert!(matches!(test.frames()[4], Frame::TwoFrame(10, 0)));

    test.frames_mut()[4] = Frame::TwoFrame(5, 5);

    assert!(matches!(test.frames()[4], Frame::TwoFrame(5, 5)));
}

#[test]
fn is_valid_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::TwoFrame(k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert!(test.is_valid());
                }
            }
        }
    }
}

#[test]
fn is_valid_two_edge() {
    // TODO: Test is_valid TwoFrame edge cases
    // Test invalid two frames
    // Pin Count sums above 10
    // Strike or Spare in 10th frame
}

#[test]
fn is_valid_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                assert!(test.is_valid());
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert!(test.is_valid());
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert!(test.is_valid());
                }
            }
        }
    }
}

#[test]
fn is_valid_three_edge() {
    // TODO: Test is_valid ThreeFrame edge cases
    // Test invalid third frames
    // No strike or spares
}

#[test]
fn score() {
    let games = sample_games();

    for key in games.keys() {
        assert_eq!(games[key].score(), *key);
    }
}

// TODO: Make test for score_n

#[test]
fn pin_count_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let mut sum = 0;

                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    sum += k + l;
                                    Frame::TwoFrame(k, l)
                                } else {
                                    sum += i + j;
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(test.pin_count(), sum);
                }
            }
        }
    }
}

#[test]
fn pin_count_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let mut sum = 0;

                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                sum += 20 + k;
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                sum += i + j;
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                assert_eq!(test.pin_count(), sum);
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let mut sum = 0;

                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    sum += 10 + k + l;
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    sum += i + j;
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(test.pin_count(), sum);
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let mut sum = 0;

                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    sum += 10 + l;
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    sum += i + j;
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(test.pin_count(), sum);
                }
            }
        }
    }
}

#[test]
fn num_strikes_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::TwoFrame(k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i == 10 {
                        assert_eq!(test.num_strikes(), 9);
                    } else {
                        assert_eq!(test.num_strikes(), 0);
                    }
                }
            }
        }
    }
}

#[test]
fn num_strikes_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                if i == 10 {
                    if k == 10 {
                        assert_eq!(test.num_strikes(), 12);
                    } else {
                        assert_eq!(test.num_strikes(), 11);
                    }
                } else {
                    if k == 10 {
                        assert_eq!(test.num_strikes(), 3);
                    } else {
                        assert_eq!(test.num_strikes(), 2);
                    }
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i == 10 {
                        if k == 10 {
                            assert_eq!(test.num_strikes(), 11);
                        } else {
                            assert_eq!(test.num_strikes(), 10);
                        }
                    } else {
                        if k == 10 {
                            assert_eq!(test.num_strikes(), 2);
                        } else {
                            assert_eq!(test.num_strikes(), 1);
                        }
                    }
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i == 10 {
                        if k == 10 || l == 10 {
                            assert_eq!(test.num_strikes(), 10);
                        } else {
                            assert_eq!(test.num_strikes(), 9);
                        }
                    } else {
                        if k == 10 || l == 10 {
                            assert_eq!(test.num_strikes(), 1);
                        } else {
                            assert_eq!(test.num_strikes(), 0, "{} {} {} {}", i, j, k, l);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn num_spares_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::TwoFrame(k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i != 10 && i + j == 10 {
                        assert_eq!(test.num_spares(), 9);
                    } else {
                        assert_eq!(test.num_spares(), 0);
                    }
                }
            }
        }
    }
}

#[test]
fn num_spares_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                if i != 10 && i + j == 10 {
                    assert_eq!(test.num_spares(), 9);
                } else {
                    assert_eq!(test.num_spares(), 0);
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i != 10 && i + j == 10 {
                        if k != 10 && k + l == 10 {
                            assert_eq!(test.num_spares(), 10);
                        } else {
                            assert_eq!(test.num_spares(), 9);
                        }
                    } else {
                        if k != 10 && k + l == 10 {
                            assert_eq!(test.num_spares(), 1);
                        } else {
                            assert_eq!(test.num_spares(), 0);
                        }
                    }
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i != 10 && i + j == 10 {
                        if k != 10 || l == 10 {
                            assert_eq!(test.num_spares(), 10);
                        } else {
                            assert_eq!(test.num_spares(), 9);
                        }
                    } else {
                        if k != 10 || l == 10 {
                            assert_eq!(test.num_spares(), 1);
                        } else {
                            assert_eq!(test.num_spares(), 0);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn open_frames_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::TwoFrame(k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i + j == 10 {
                        assert_eq!(test.open_frames(), 1);
                    } else {
                        assert_eq!(test.open_frames(), 10);
                    }
                }
            }
        }
    }
}

#[test]
fn open_frames_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                if i + j == 10 {
                    assert_eq!(test.open_frames(), 0);
                } else {
                    assert_eq!(test.open_frames(), 9);
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i + j == 10 {
                        assert_eq!(test.open_frames(), 0);
                    } else {
                        assert_eq!(test.open_frames(), 9);
                    }
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    if i + j == 10 {
                        assert_eq!(test.open_frames(), 0);
                    } else {
                        assert_eq!(test.open_frames(), 9);
                    }
                }
            }
        }
    }
}

#[test]
fn avg_first_ball_pinfall_two() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=9 {
                for l in 0..=9 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::TwoFrame(k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(
                        test.avg_first_ball_pinfall(),
                        ((i * 9 + k) as f32) / (10 as f32)
                    )
                }
            }
        }
    }
}

#[test]
fn avg_first_ball_pinfall_three() {
    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                let test = Game::build_with(
                    1,
                    (1..=10)
                        .map(|n| {
                            if n == 10 {
                                Frame::ThreeFrame(10, 10, k)
                            } else {
                                Frame::TwoFrame(i, j)
                            }
                        })
                        .collect(),
                );

                assert_eq!(
                    test.avg_first_ball_pinfall(),
                    ((i * 9 + 10) as f32) / (10 as f32)
                )
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 - k {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(10, k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(
                        test.avg_first_ball_pinfall(),
                        ((i * 9 + 10) as f32) / (10 as f32)
                    )
                }
            }
        }
    }

    for i in 0..=10 {
        for j in 0..=10 - i {
            for k in 0..=10 {
                for l in 0..=10 {
                    let test = Game::build_with(
                        1,
                        (1..=10)
                            .map(|n| {
                                if n == 10 {
                                    Frame::ThreeFrame(k, 10 - k, l)
                                } else {
                                    Frame::TwoFrame(i, j)
                                }
                            })
                            .collect(),
                    );

                    assert_eq!(
                        test.avg_first_ball_pinfall(),
                        ((i * 9 + k) as f32) / (10 as f32)
                    )
                }
            }
        }
    }
}

#[test]
fn partialeq() {
    let games = sample_games();

    for k1 in games.keys() {
        for k2 in games.keys() {
            if k1 == k2 {
                assert_eq!(games[k1], games[k2]);
            } else {
                assert_ne!(games[k1], games[k2]);
            }
        }
    }
}

#[test]
fn partialord() {
    let games = sample_games();

    for k1 in games.keys() {
        for k2 in games.keys() {
            if k1 < k2 {
                assert!(matches!(
                    games[k1].partial_cmp(&games[k2]),
                    Some(Ordering::Less)
                ));
            } else if k1 > k2 {
                assert!(matches!(
                    games[k1].partial_cmp(&games[k2]),
                    Some(Ordering::Greater)
                ));
            } else {
                assert!(matches!(
                    games[k1].partial_cmp(&games[k2]),
                    Some(Ordering::Equal)
                ));
            }
        }
    }
}
