use advent_of_code_2018::{
    d6::{coordinates, manhattan},
    util::IteratorExt,
};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut hull = HashSet::new();
    let mut areas = HashMap::new();
    for i in 0..500 {
        for j in 0..500 {
            if let Some(closest) =
                coordinates().single_min_by_key(|&point| manhattan(point, (i, j)))
            {
                if i == 0 || i == 499 || j == 0 || j == 499 {
                    hull.insert(closest);
                }
                *areas.entry(closest).or_insert(0) += 1;
            }
        }
    }
    let (_, area) = areas
        .iter()
        .filter(|(point, _)| !hull.contains(&point))
        .max_by_key(|(_, &area)| area)
        .unwrap();
    println!("{}", area);
}
