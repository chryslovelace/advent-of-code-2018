use advent_of_code_2018::d2::parsed_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut twos = 0;
    let mut threes = 0;
    for id in parsed_input() {
        let counts = letter_counts(id);
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
}

fn letter_counts(id: &str) -> HashSet<u8> {
    let mut map = HashMap::new();
    for c in id.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map.values().cloned().collect::<HashSet<_>>()
}
