use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

use log::{info, debug};

fn main() {

    let rubric_str = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut item_rubric: HashMap<char, i32> = HashMap::new();
    for (index, char) in rubric_str.char_indices() {
        // lowercase
        let lowercase_v = index as i32 + 1;
        item_rubric.insert(char, lowercase_v);
        // uppercase
        item_rubric.insert(char.to_ascii_uppercase(), lowercase_v + 26);
    }

    pretty_env_logger::init();
    let input = File::open("./day3-input").unwrap();
    let lines = BufReader::new(input).lines()
        .map(|l|l.unwrap());
    let mut shared_items: Vec<char> = Vec::new();
    for rucksack in lines {
        info!("rucksack contains: {rucksack}");
        let mut c1_items: HashSet<char> = HashSet::new();
        let mut c2_items: HashSet<char> = HashSet::new();
        let midpoint = rucksack.len() / 2; // this is OK because only ASCII
        for (index, char) in rucksack.char_indices() {
            if index < midpoint {
                c1_items.insert(char);
            } else {
                c2_items.insert(char);
            }
        }
        info!("c1_items: {:?}", c1_items);
        info!("c2_items: {:?}", c2_items);
        for item in c1_items.intersection(&c2_items) {
            shared_items.push(*item);
        }
    }
    let sum:i32 = shared_items.iter()
        .map(|item| {
            debug!(">>{item}");
            item_rubric.get(item).unwrap()
        }).sum();
    info!("sum of shared item priorities: {sum}")
}
