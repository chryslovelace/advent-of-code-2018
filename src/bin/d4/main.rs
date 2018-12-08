use chrono::NaiveDateTime;
use itertools::Itertools;
use lazy_static::lazy_static;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref SLEEPYTIME: HashMap<u32, [u32; 60]> = {
        let mut entries = include_str!("input.txt")
            .lines()
            .sorted_by_key(|line| {
                NaiveDateTime::parse_from_str(&line[1..17], "%Y-%m-%d %H:%M").unwrap()
            })
            .into_iter()
            .peekable();

        let mut sleepytime = HashMap::new();
        while let Some(id_line) = entries.next() {
            let id = scan_fmt!(&id_line[19..], "Guard #{d} begins shift", u32).unwrap();
            let minutes = sleepytime.entry(id).or_insert_with(|| [0; 60]);
            while let Some(line) = entries.peek() {
                if line.ends_with("begins shift") {
                    break;
                }
                let sleeps: usize = entries.next().unwrap()[15..17].parse().unwrap();
                let wakes: usize = entries.next().unwrap()[15..17].parse().unwrap();
                for minute in minutes.iter_mut().take(wakes).skip(sleeps) {
                    *minute += 1;
                }
            }
        }
        sleepytime
    };
}

fn part1() {
    let (id, minutes) = SLEEPYTIME
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        .unwrap();
    let minute = (0..60).max_by_key(|&i| minutes[i as usize]).unwrap();
    println!("{}", id * minute);
}

fn part2() {
    let (id, minutes) = SLEEPYTIME
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().max())
        .unwrap();
    let minute = (0..60).max_by_key(|&i| minutes[i as usize]).unwrap();
    println!("{}", id * minute);
}
