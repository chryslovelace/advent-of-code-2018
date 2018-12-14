use std::cmp::max;

struct State {
    recipes: Vec<u8>,
    elves: Vec<usize>,
}

impl State {
    fn new() -> Self {
        State {
            recipes: vec![3, 7],
            elves: vec![0, 1],
        }
    }

    fn run(&mut self) {
        let sum: u8 = self.elves.iter().map(|&i| self.recipes[i]).sum();
        if sum >= 10 {
            self.recipes.push(sum / 10);
            self.recipes.push(sum % 10);
        } else {
            self.recipes.push(sum);
        }
        for elf in &mut self.elves {
            *elf += 1 + self.recipes[*elf] as usize;
            *elf %= self.recipes.len();
        }
    }

    fn back_offset(&self) -> usize {
        max(8, self.recipes.len()) - 8
    }

    fn back(&self) -> &[u8] {
        &self.recipes[self.back_offset()..]
    }
}

fn part1() {
    let mut state = State::new();
    let input = 793031;
    while state.recipes.len() < input + 10 {
        state.run();
    }
    for score in &state.recipes[input..input + 10] {
        print!("{}", score);
    }
    println!();
}

fn part2() {
    let mut state = State::new();
    let input = [7, 9, 3, 0, 3, 1];
    let pos = loop {
        state.run();
        if let Some(matched) = state.back().windows(6).position(|w| w == &input[..]) {
            break state.back_offset() + matched;
        }
    };
    println!("{}", pos);
}

fn main() {
    part1();
    part2();
}
