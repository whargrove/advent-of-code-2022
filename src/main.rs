use std::{fs::File, io::{BufReader, BufRead}, collections::BinaryHeap};

fn main() {
    pretty_env_logger::init();
    let input = File::open("./day1-input").unwrap();
    let lines = BufReader::new(input).lines()
        .map(|l| l.unwrap());
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut buf = 0;
    for line in lines {
        // add i32 value to buf if parse
        // or check if buf is greater than max and swap
        if let Some(calorie) = line.parse::<i32>().ok() {
            buf = buf + calorie;
        } else {
            heap.push(buf);
            buf = 0;
        }
    }
    let mut sum = 0;
    for _ in 1..=3 {
        sum = sum + heap.pop().unwrap();
    }
    println!("Max carried calories: {sum}");
}
