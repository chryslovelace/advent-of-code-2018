use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn constraints() -> impl Iterator<Item = (&'static str, &'static str)> {
    INPUT.lines().map(|line| (&line[5..6], &line[36..37]))
}

#[derive(Debug)]
pub struct Step {
    pub name: &'static str,
    pub prereqs: String
}

impl Step {
    fn new(name: &'static str) -> Self {
        Step {
            name,
            prereqs: String::new()
        }
    }
}

pub fn steps() -> Vec<Step> { 
    let mut map = HashMap::new();
    for (prereq, step) in constraints() {
        map.entry(step).or_insert_with(|| Step::new(step)).prereqs.push_str(prereq);
        map.entry(prereq).or_insert_with(|| Step::new(prereq));
    }
    map.drain().map(|(_, step)| step).sorted_by_key(|step| step.name)
}