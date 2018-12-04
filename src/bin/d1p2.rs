use advent_of_code_2018::d1::frequency_changes;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    let mut freq = 0;
    for n in frequency_changes().cycle() {
        if set.contains(&freq) {
            break;
        }
        set.insert(freq);
        freq += n;
    }
    println!("{}", freq);
}
