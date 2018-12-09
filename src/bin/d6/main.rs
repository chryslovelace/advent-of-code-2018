use itertools::iproduct;
use lazy_static::lazy_static;
use scan_fmt::scan_fmt;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref COORDINATES: Vec<(i32, i32)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (x, y) = scan_fmt!(line, "{d}, {d}", i32, i32);
            (x.unwrap(), y.unwrap())
        })
        .collect();
}

fn part1() {
    let mut hull = HashSet::new();
    let mut areas = HashMap::new();
    for (i, j) in iproduct!(0..500, 0..500) {
        if let Some(closest) = COORDINATES
            .iter()
            .single_min_by_key(|&&point| manhattan(point, (i, j)))
        {
            if i == 0 || i == 499 || j == 0 || j == 499 {
                hull.insert(closest);
            }
            *areas.entry(closest).or_insert(0) += 1;
        }
    }
    let (_, area) = areas
        .iter()
        .filter(|(&point, _)| !hull.contains(point))
        .max_by_key(|(_, &area)| area)
        .unwrap();
    println!("{}", area);
}

fn part2() {
    let safe_region_size = iproduct!(0..500, 0..500)
        .filter(|&a| COORDINATES.iter().map(|&b| manhattan(a, b)).sum::<i32>() < 10000)
        .count();
    println!("{}", safe_region_size);
}

fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

trait IteratorExt: Iterator {
    fn single(self) -> Option<Self::Item>;
    fn single_min_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B;
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

    fn single_min_by_key<B, F>(mut self, mut f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    {
        let mut candidate = self.next();
        let mut candidate_key = candidate.as_ref().map(|item| f(item));
        for item in self {
            let key = f(&item);
            if let Some(ref mut candidate_key) = candidate_key {
                match key.cmp(&candidate_key) {
                    Ordering::Less => {
                        *candidate_key = key;
                        candidate = Some(item);
                    }
                    Ordering::Equal => {
                        candidate = None;
                    }
                    Ordering::Greater => {}
                }
            }
        }
        candidate
    }
}
