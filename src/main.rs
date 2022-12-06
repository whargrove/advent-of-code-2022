use std::{fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;

enum Move {
    Rock,
    Paper,
    Scissors
}

impl From<String> for Move {
    fn from(move_str: String) -> Self {
        match move_str.as_str() {
            "A" => Move::Rock,
            "X" => Move::Rock,
            "B" => Move::Paper,
            "Y" => Move::Paper,
            "C" => Move::Scissors,
            "Z" => Move::Scissors,
            _ => panic!()
        }
    }
}

fn main() {
    pretty_env_logger::init();
    let input = File::open("./day2-input").unwrap();
    let sum:i32 = BufReader::new(input).lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (theirs, mine) = l.split(" ")
                .map(|s| s.to_string())
                .map_into::<Move>()
                .collect_tuple().unwrap();
            let round_score = match mine {
                Move::Rock => {
                    match theirs {
                        Move::Scissors => {
                            1 + 6
                        },
                        Move::Rock => {
                            1 + 3
                        },
                        _ => 1,
                    }
                },
                Move::Paper => {
                    match theirs {
                        Move::Rock => {
                            2 + 6
                        },
                        Move::Paper => {
                            2 + 3
                        }
                        _ => 2,
                    }
                },
                Move::Scissors => {
                    match theirs {
                        Move::Paper => {
                            3 + 6
                        },
                        Move::Scissors => {
                            3 + 3
                        }
                        _ => 3,
                    }
                },
            };
            round_score
        }).sum();
        print!("Score: {sum}")
}
