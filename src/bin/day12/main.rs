use lazy_static::lazy_static;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct State(BTreeSet<isize>);

impl State {
    fn has_plant(&self, pos: isize) -> bool {
        self.0.contains(&pos)
    }

    fn min(&self) -> isize {
        *self.0.iter().next().unwrap()
    }

    fn max(&self) -> isize {
        *self.0.iter().next_back().unwrap()
    }

    fn matches(&self, pos: isize, rule: &Rule) -> bool {
        rule.pattern.iter().zip(pos - 2..).all(|(&pat, pos)| self.has_plant(pos) == pat)
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
                .filter_map(|(i, c)| if c == '#' { Some(i as isize) } else { None })
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
                let result = line.chars().next_back().unwrap() == '#';
                Rule { pattern, result }
            })
            .collect()
    };
}

fn main() {
    let mut state = INITIAL_STATE.clone();
    for _ in 0..20 {
        state.next_state(&*RULES);
    }
    println!("{}", state.0.iter().sum::<isize>());
    
    for _ in 0..50000000000isize - 20 {
        state.next_state(&*RULES);
    }
    println!("{}", state.0.iter().sum::<isize>());
}
