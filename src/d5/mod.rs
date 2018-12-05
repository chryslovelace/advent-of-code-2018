use std::mem::swap;

const INPUT: &str = include_str!("input.txt");

pub fn input() -> &'static str {
    INPUT.trim()
}

pub fn react(s: &str) -> String {
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
