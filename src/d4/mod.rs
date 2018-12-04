use std::collections::HashMap;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use chrono::{NaiveDateTime, Timelike};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Entry {
    date: NaiveDateTime,
    kind: EntryKind
}

#[derive(Debug)]
enum EntryKind {
    ShiftStart(u32),
    FallsAsleep,
    WakesUp
}

pub fn sleepytime() -> HashMap<u32, HashMap<u32, u32>> {
    let entries = INPUT.lines().map(|line| {
        let date = NaiveDateTime::parse_from_str(&line[1..17], "%Y-%m-%d %H:%M").unwrap();
        let kind = if let Some(id) = scan_fmt!(&line[19..], "Guard #{d} begins shift", u32) {
            EntryKind::ShiftStart(id)
        } else if &line[19..] == "falls asleep" {
            EntryKind::FallsAsleep
        } else if &line[19..] == "wakes up" {
            EntryKind::WakesUp
        } else {
            unreachable!()
        };
        Entry {date, kind}
    }).sorted_by_key(|entry| entry.date);

    let mut runs = Vec::new();
    for entry in entries {
        if let EntryKind::ShiftStart(_) = entry.kind {
            runs.push(Vec::new());
        }
        runs.last_mut().unwrap().push(entry);
    }

    let mut sleepytime = HashMap::new();
    for run in runs {
        let id = if let EntryKind::ShiftStart(id) = run[0].kind { id } else { unreachable!() };
        let minutes = sleepytime.entry(id).or_insert_with(HashMap::new);
        let naps = run[1..].chunks(2).map(|chunk| chunk[0].date.minute()..chunk[1].date.minute());
        for nap in naps {
            for minute in nap {
                *minutes.entry(minute).or_insert(0) += 1;
            }
        }
    }
    sleepytime
}
