use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    pretty_env_logger::init();
    let input = File::open("./day4-input").unwrap();
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let count = lines
        .map(|line| {
            line.split(',')
                .map(|range_str| {
                    let mut range_it = range_str.split('-');
                    let start: i32 = range_it.next().unwrap().parse().unwrap();
                    let end: i32 = range_it.next().unwrap().parse().unwrap();
                    start..=end
                })
                .next_tuple()
                .unwrap()
        })
        .filter(|(r1, r2)| r1.end() >= r2.start() && r2.end() >= r1.start())
        .count() as i32;
    println!("{count}");
}
