use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref STEPS: Vec<Step> = {
        let mut map = HashMap::new();
        let constraints = include_str!("input.txt")
            .lines()
            .map(|line| (&line[5..6], &line[36..37]));
        for (prereq, step) in constraints {
            map.entry(step)
                .or_insert_with(|| Step::new(step))
                .prereqs
                .push_str(prereq);
            map.entry(prereq).or_insert_with(|| Step::new(prereq));
        }
        map.drain()
            .map(|(_, step)| step)
            .sorted_by_key(|step| step.name)
    };
}

fn part1() {
    let mut order = String::new();
    while let Some(next_step) = STEPS
        .iter()
        .find(|step| step.is_ready(&order, &order))
        .map(|step| step.name)
    {
        order.push_str(next_step);
    }
    println!("{}", order);
}

fn part2() {
    let mut worked_on = String::new();
    let mut completed = String::new();
    let mut time = 0;
    let mut workers = vec![Worker::default(); 5];

    while completed.len() < 26 {
        for worker in &mut workers {
            if let Some(step) = worker.step {
                if worker.elapsed >= time_needed(step) {
                    completed.push_str(step);
                    worker.step = None;
                    worker.elapsed = 0;
                }
            }
        }
        for worker in &mut workers {
            if worker.step.is_none() {
                worker.step = STEPS
                    .iter()
                    .find(|step| step.is_ready(&worked_on, &completed))
                    .map(|step| step.name);
            }
            if let Some(step) = worker.step {
                if !worked_on.contains(step) {
                    worked_on.push_str(step);
                }
                worker.elapsed += 1;
            }
        }
        time += 1;
    }
    println!("{}", time - 1);

    #[derive(Clone, Default)]
    struct Worker {
        step: Option<&'static str>,
        elapsed: u8,
    }

    fn time_needed(step: &str) -> u8 {
        step.as_bytes()[0] - b'A' + 61
    }
}

struct Step {
    name: &'static str,
    prereqs: String,
}

impl Step {
    fn new(name: &'static str) -> Self {
        Step {
            name,
            prereqs: String::new(),
        }
    }

    fn is_ready(&self, started: &str, completed: &str) -> bool {
        !started.contains(&self.name)
            && self
                .prereqs
                .chars()
                .all(|prereq| completed.contains(prereq))
    }
}
