use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref IDS: Vec<&'static str> = include_str!("input.txt").lines().collect();
}

fn part1() {
    let mut twos = 0;
    let mut threes = 0;
    for id in IDS.iter() {
        let counts = letter_counts(id);
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);

    fn letter_counts(id: &str) -> HashSet<u8> {
        let mut map = HashMap::new();
        for c in id.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map.values().cloned().collect::<HashSet<_>>()
    }
}

fn part2() {
    for id1 in IDS.iter() {
        for id2 in IDS.iter() {
            let unmatched_pair = id1
                .chars()
                .zip(id2.chars())
                .filter(|(c, d)| c != d)
                .single();
            if let Some((c, _)) = unmatched_pair {
                println!("{}", id1.replace(c, ""));
                return;
            }
        }
    }
}

trait IteratorExt: Iterator {
    fn single(self) -> Option<Self::Item>;
}

impl<I: Iterator> IteratorExt for I {
    fn single(mut self) -> Option<Self::Item> {
        self.next().and_then(|elem| {
            if self.next().is_none() {
                Some(elem)
            } else {
                None
            }
        })
    }
}
