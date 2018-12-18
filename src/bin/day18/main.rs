use itertools::iproduct;
use lazy_static::lazy_static;
use std::{
    cmp::min,
    collections::{hash_map::Entry, HashMap},
    fmt,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Acre {
    Ground,
    Trees,
    Lumberyard,
}

#[derive(Clone)]
struct State(Vec<Vec<Acre>>);

struct Counts {
    trees: usize,
    lumberyard: usize,
}

impl State {
    fn adjacent(&self, x: usize, y: usize) -> Counts {
        let xmin = if x == 0 { 0 } else { x - 1 };
        let xmax = min(x + 1, self.0[0].len() - 1);
        let ymin = if y == 0 { 0 } else { y - 1 };
        let ymax = min(y + 1, self.0.len() - 1);
        let (mut trees, mut lumberyard) = (0, 0);
        for acre in iproduct!(xmin..=xmax, ymin..=ymax)
            .filter(|&p| p != (x, y))
            .map(|(x, y)| self.0[y][x])
        {
            match acre {
                Acre::Ground => {}
                Acre::Trees => trees += 1,
                Acre::Lumberyard => lumberyard += 1,
            };
        }
        Counts { trees, lumberyard }
    }

    fn next_acre(&self, x: usize, y: usize) -> Acre {
        let adjacent = self.adjacent(x, y);
        match self.0[y][x] {
            Acre::Ground => {
                if adjacent.trees >= 3 {
                    Acre::Trees
                } else {
                    Acre::Ground
                }
            }
            Acre::Trees => {
                if adjacent.lumberyard >= 3 {
                    Acre::Lumberyard
                } else {
                    Acre::Trees
                }
            }
            Acre::Lumberyard => {
                if adjacent.lumberyard >= 1 && adjacent.trees >= 1 {
                    Acre::Lumberyard
                } else {
                    Acre::Ground
                }
            }
        }
    }

    fn next_state(&self) -> State {
        State(
            (0..self.0.len())
                .map(|y| (0..self.0[0].len()).map(|x| self.next_acre(x, y)).collect())
                .collect(),
        )
    }

    fn resource_value(&self) -> usize {
        let (mut trees, mut lumberyard) = (0, 0);
        for row in &self.0 {
            for acre in row {
                match acre {
                    Acre::Trees => trees += 1,
                    Acre::Lumberyard => lumberyard += 1,
                    Acre::Ground => {}
                }
            }
        }
        trees * lumberyard
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for acre in row {
                match acre {
                    Acre::Ground => '.'.fmt(f)?,
                    Acre::Trees => '|'.fmt(f)?,
                    Acre::Lumberyard => '#'.fmt(f)?,
                }
            }
            '\n'.fmt(f)?;
        }
        Ok(())
    }
}

lazy_static! {
    static ref INITIAL_STATE: State = {
        State(
            include_str!("input.txt")
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Acre::Ground,
                            '|' => Acre::Trees,
                            '#' => Acre::Lumberyard,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        )
    };
}

const ONE_BILLION: usize = 1_000_000_000;

fn part1() {
    let mut state = INITIAL_STATE.clone();
    for _ in 0..10 {
        state = state.next_state();
    }
    println!("{}", state.resource_value());
}

fn part2() {
    // thanks again jewel!!!!
    let mut state = INITIAL_STATE.clone();
    let mut seen = HashMap::new();
    let mut step = 0;
    let cycle_start = loop {
        match seen.entry(state.to_string()) {
            Entry::Vacant(entry) => entry.insert(step),
            Entry::Occupied(entry) => break *entry.get(),
        };
        step += 1;
        state = state.next_state();
    };
    let cycle_length = step - cycle_start;
    let num_cycles = (ONE_BILLION - cycle_start) / cycle_length - 1;
    step += num_cycles * cycle_length;
    for _ in step..ONE_BILLION {
        state = state.next_state();
    }
    println!("{}", state.resource_value());
}

fn main() {
    part1();
    part2();
}
