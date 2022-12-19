// parse procedure
// let (quantity, source, target) = procedure_str
//     .split(' ')
//     .filter(first char is numeric?)
//     .next_tuple()

// for each procedure
// pop quantity from source and push into target

use std::{fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;
use log::info;

fn main() {
    pretty_env_logger::init();

    //         [C] [B] [H]
    // [W]     [D] [J] [Q] [B]
    // [P] [F] [Z] [F] [B] [L]
    // [G] [Z] [N] [P] [J] [S] [V]
    // [Z] [C] [H] [Z] [G] [T] [Z]     [C]
    // [V] [B] [M] [M] [C] [Q] [C] [G] [H]
    // [S] [V] [L] [D] [F] [F] [G] [L] [F]
    // [B] [J] [V] [L] [V] [G] [L] [N] [J]
    //  1   2   3   4   5   6   7   8   9

    // create the initial stack of crates
    // use Vec<Char> so each will work as a Stack data structure

    let mut stacks = vec![
        vec!['B', 'S', 'V', 'Z', 'G', 'P', 'W'],
        vec!['J', 'V', 'B', 'C', 'Z', 'F'],
        vec!['V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C'],
        vec!['L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B'],
        vec!['V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H'],
        vec!['G', 'F', 'Q', 'T', 'S', 'L', 'B'],
        vec!['L', 'G', 'C', 'Z', 'V'],
        vec!['N', 'L', 'G'],
        vec!['J', 'F', 'H', 'C'],
    ];

    let input = File::open("./day5-input").unwrap();
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    // each line is a procedure with format: move {quantity} from {source} to {target}
    // where quantity is the number of crates that should be moved from a source stack to a target stack
    lines
        .for_each(|procedure_string| {
            let (quantity, source, target): (i32, i32, i32) = procedure_string
                .split(' ')
                .filter(|&s| s.chars().nth(0).unwrap().is_ascii_digit())
                .map(|s| (*s).parse().unwrap())
                .next_tuple()
                .unwrap();
            // need to clone the source since we cannot have two mutable borrows at the same time against stacks
            // do not clone the target since we are pushing items (one at a time) from the source into the target
            let items_to_move = &mut stacks[(source - 1) as usize].clone();
            for _ in 0..quantity {
                let target_stack = &mut stacks[(target - 1) as usize];
                target_stack.push(items_to_move.pop().unwrap());
            }
        });

    let tops: String = stacks
        .iter_mut()
        .flat_map(|stack: &mut Vec<char>| stack.pop())
        .collect();
    info!("TOPS: {tops}");
    
}
