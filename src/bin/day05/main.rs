use lazy_static::lazy_static;
use std::mem::swap;

lazy_static! {
    static ref POLYMER: &'static str = include_str!("input.txt").trim();
}

fn react(s: &str) -> String {
    let mut curr = s.to_string();
    let mut next = String::with_capacity(curr.len());
    loop {
        let mut chars = curr.chars().peekable();
        while let Some(a) = chars.next() {
            if reacts(a, chars.peek()) {
                let _ = chars.next();
            } else {
                next.push(a)
            }
        }
        if curr == next {
            return curr;
        }
        swap(&mut curr, &mut next);
        next.clear();
    }

    fn reacts(a: char, b: Option<&char>) -> bool {
        if let Some(b) = b {
            if a.is_ascii_uppercase() {
                a.to_ascii_lowercase() == *b
            } else {
                a.to_ascii_uppercase() == *b
            }
        } else {
            false
        }
    }
}

fn part1() {
    println!("{}", react(*POLYMER).len());
}

fn part2() {
    let result = (b'a'..=b'z')
        .map(|c| react(&POLYMER.replace(|d: char| c as char == d.to_ascii_lowercase(), "")).len())
        .min()
        .unwrap();
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
