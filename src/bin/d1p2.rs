use advent_of_code_2018::d1::parsed_input;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    let mut freq = 0;
    set.insert(freq);
    for n in parsed_input().cycle() {
        freq += n;
        if set.contains(&freq) { break; } 
        else { set.insert(freq); }
    };
    println!("{}", freq);
}