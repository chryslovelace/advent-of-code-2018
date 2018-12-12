use lazy_static::lazy_static;
use std::collections::{hash_map::Entry, BTreeSet, HashMap};

#[derive(Debug, Clone)]
struct State(BTreeSet<i64>);

impl State {
    fn has_plant(&self, pos: i64) -> bool {
        self.0.contains(&pos)
    }

    fn min(&self) -> i64 {
        *self.0.iter().next().unwrap()
    }

    fn max(&self) -> i64 {
        *self.0.iter().next_back().unwrap()
    }

    fn matches(&self, pos: i64, rule: &Rule) -> bool {
        rule.pattern
            .iter()
            .zip(pos - 2..)
            .all(|(&pat, pos)| self.has_plant(pos) == pat)
    }

    fn next_state(&mut self, rules: &[Rule]) {
        let mut add = Vec::new();
        let mut remove = Vec::new();
        for pos in self.min() - 2..self.max() + 3 {
            if let Some(matched_rule) = rules.iter().find(|rule| self.matches(pos, rule)) {
                if matched_rule.result {
                    add.push(pos);
                } else {
                    remove.push(pos);
                }
            }
        }
        for pos in add {
            self.0.insert(pos);
        }
        for pos in remove {
            self.0.remove(&pos);
        }
    }

    fn sum(&self) -> i64 {
        self.0.iter().sum()
    }

    fn signature(&self) -> String {
        (self.min()..=self.max())
            .map(|i| if self.has_plant(i) { '#' } else { '.' })
            .collect()
    }

    fn shift(&self, amount: i64) -> State {
        State(self.0.iter().map(|i| i + amount).collect())
    }
}

struct Rule {
    pattern: [bool; 5],
    result: bool,
}

const INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref INITIAL_STATE: State = {
        State(
            INPUT.lines().next().unwrap()[15..]
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i as i64) } else { None })
                .collect(),
        )
    };
    static ref RULES: Vec<Rule> = {
        INPUT
            .lines()
            .skip(2)
            .map(|line| {
                let mut pattern = [false; 5];
                for (i, c) in line[..5].chars().enumerate() {
                    pattern[i] = c == '#';
                }
                let result = line.ends_with('#');
                Rule { pattern, result }
            })
            .collect()
    };
}

fn part1() {
    let mut state = INITIAL_STATE.clone();
    for _ in 0..20 {
        state.next_state(&*RULES);
    }
    println!("{}", state.sum());
}

const FIFTY_BILLION: i64 = 50_000_000_000;

fn part2() {
    // thanks to jewel for this idea
    // in this case the cycle length happens to be 1 so most of this is unnecessary,
    // but presumably this would work for any cycle length
    let mut state = INITIAL_STATE.clone();
    let mut seen = HashMap::new();
    let mut step = 0;
    let (cycle_start, start_min) = loop {
        match seen.entry(state.signature()) {
            Entry::Vacant(entry) => entry.insert((step, state.min())),
            Entry::Occupied(entry) => break entry.remove(),
        };
        step += 1;
        state.next_state(&*RULES);
    };
    let cycle_length = step - cycle_start;
    let cycle_shift = state.min() - start_min;
    let num_cycles = (FIFTY_BILLION - cycle_start) / cycle_length - 1;
    state = state.shift(num_cycles * cycle_shift);
    step += num_cycles * cycle_length;
    for _ in step..FIFTY_BILLION {
        state.next_state(&*RULES);
    }
    println!("{}", state.sum());
}

fn main() {
    part1();
    part2();
}
