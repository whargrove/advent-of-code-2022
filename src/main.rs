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

    // this vec will contain each rucksack
    // when the len is 3 we will find the shared items by intersecting all three lines
    let mut group_buf: Vec<String> = Vec::new();
    let mut shared_items: Vec<char> = Vec::new();
    for rucksack in lines {
        if group_buf.len() < 3 {
            group_buf.push(rucksack);
        }
        if group_buf.len() == 3 {
            let sets: Vec<HashSet<char>> = group_buf.iter()
                .map(|b| {
                    let mut s = HashSet::new();
                    for c in b.chars() {
                        s.insert(c);
                    }
                    s
                }).collect();
            let shared = sets.iter()
                .cloned()
                .reduce(|accum, item| {
                    let intersection: HashSet<char> = accum.intersection(&item).into_iter().cloned().collect();
                    return intersection;
                }).unwrap();
            for c in shared {
                shared_items.push(c);
            }
            group_buf.clear();
        }
    }
    let sum:i32 = shared_items.iter()
        .map(|item| {
            debug!(">>{item}");
            item_rubric.get(item).unwrap()
        }).sum();
    info!("sum of shared item priorities: {sum}")
}
