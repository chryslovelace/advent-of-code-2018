use std::ops::Range;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use chrono::{Timelike, NaiveDate, NaiveDateTime};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct Entry {
    date: NaiveDateTime,
    kind: EntryKind
}

#[derive(Debug)]
pub enum EntryKind {
    ShiftStart(u32),
    FallsAsleep,
    WakesUp
}

#[derive(Debug)]
pub struct Shift {
    pub date: NaiveDate,
    pub id: u32,
    pub sleeping: Vec<Range<u32>>
}

pub fn parsed_input() -> Vec<Shift> {
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
    runs.iter().map(|run| {
        let id = if let EntryKind::ShiftStart(id) = run[0].kind { id } else { unreachable!() };
        let datetime = run[0].date;
        let date = if datetime.hour() != 0 { datetime.date().succ() } else { datetime.date() };
        let sleeping = run[1..].chunks(2).map(|chunk| chunk[0].date.minute()..chunk[1].date.minute()).collect::<Vec<_>>();
        Shift {id, date, sleeping}
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        println!("{:?}", &parsed_input()[..5]);
    }
}