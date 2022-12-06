use std::{fs::File, io::{BufReader, BufRead}};
use log::{info};

fn main() {
    pretty_env_logger::init();
    let input = File::open("./day1-input").unwrap();
    let lines = BufReader::new(input).lines()
        .map(|l| l.unwrap());
    let mut max = 0;
    let mut buf = 0;
    for line in lines {
        // add i32 value to buf if parse
        // or check if buf is greater than max and swap
        if let Some(calorie) = line.parse::<i32>().ok() {
            buf = buf + calorie;
            info!("+{calorie}, buf: {buf}");
        } else {
            info!("buf: {buf}, max: {max}");
            if buf > max {
                info!("!!! buf: {buf} is the new max");
                max = buf;
            }
            buf = 0;
        }
    }
    println!("Max carried calories: {max}");
}
