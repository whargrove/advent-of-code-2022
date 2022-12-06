use std::{fs::File, io::{BufReader, BufRead}};

enum Move {
    Rock,
    Paper,
    Scissors
}

impl From<String> for Move {
    fn from(move_str: String) -> Self {
        match move_str.as_str() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!()
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl From<String> for Outcome {
    fn from(outcome_str: String) -> Self {
        match outcome_str.as_str() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
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
            let mut parts = l.split(" ");
            let theirs: Move = parts.next().unwrap().to_string().into();
            let outcome: Outcome = parts.next().unwrap().to_string().into();
            let mine = match outcome {
                Outcome::Lose => match theirs {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                },
                Outcome::Draw => match theirs {
                    Move::Rock => Move::Rock,
                    Move::Paper => Move::Paper,
                    Move::Scissors => Move::Scissors,
                },
                Outcome::Win => match theirs {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                },
            };
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
        println!("Score: {sum}")
}
