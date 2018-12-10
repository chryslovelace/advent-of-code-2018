use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref FREQUENCY_CHANGES: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

fn part1() {
    println!("{}", FREQUENCY_CHANGES.iter().sum::<i32>());
}

fn part2() {
    let mut set = HashSet::new();
    let mut freq = 0;
    for n in FREQUENCY_CHANGES.iter().cycle() {
        if set.contains(&freq) {
            break;
        }
        set.insert(freq);
        freq += n;
    }
    println!("{}", freq);
}

fn main() {
    part1();
    part2();
}
