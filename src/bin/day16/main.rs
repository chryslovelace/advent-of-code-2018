use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet, hash_map::Entry};
use itertools::Itertools;
use lazy_static::lazy_static;

type Registers = [usize; 4];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

struct Inst {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Inst {
    fn new(opcode: Opcode, a: usize, b: usize, c: usize) -> Self {
        Inst { opcode, a, b, c }
    }

    fn exec(&self, registers: &mut Registers) {
        use self::Opcode::*;
        registers[self.c] = match self.opcode {
            Addr => registers[self.a] + registers[self.b],
            Addi => registers[self.a] + self.b,
            Mulr => registers[self.a] * registers[self.b],
            Muli => registers[self.a] * self.b,
            Banr => registers[self.a] & registers[self.b],
            Bani => registers[self.a] & self.b,
            Borr => registers[self.a] | registers[self.b],
            Bori => registers[self.a] | self.b,
            Setr => registers[self.a],
            Seti => self.a,
            Gtir => if self.a > registers[self.b] { 1 } else { 0 },
            Gtri => if registers[self.a] > self.b { 1 } else { 0 },
            Gtrr => if registers[self.a] > registers[self.b] { 1 } else { 0 },
            Eqir => if self.a == registers[self.b] { 1 } else { 0 },
            Eqri => if registers[self.a] == self.b { 1 } else { 0 },
            Eqrr => if registers[self.a] == registers[self.b] { 1 } else { 0 },
        }
    }
}

struct Sample {
    before: Registers,
    after: Registers,
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Sample {
    fn opcode_candidates(&self) -> HashSet<Opcode> {
        OPCODES
            .iter()
            .cloned()
            .filter(|&opcode| {
                let inst = Inst::new(opcode, self.a, self.b, self.c);
                let mut test = self.before.clone();
                inst.exec(&mut test);
                test == self.after
            })
            .collect()
    }
}

fn line_to_regs(line: &str) -> Registers {
    [
        line[9..10].parse().unwrap(), 
        line[12..13].parse().unwrap(),
        line[15..16].parse().unwrap(),
        line[18..19].parse().unwrap(),
    ]
}

lazy_static! {
    static ref SAMPLES: Vec<Sample> = {
        include_str!("input1.txt")
            .lines()
            .tuples()
            .map(|(before, inst, after, _)| {
                let before = line_to_regs(before);
                let after = line_to_regs(after);
                let (opcode, a, b, c) = scan_fmt!(inst, "{d} {d} {d} {d}", usize, usize, usize, usize);
                let (opcode, a, b, c) = (opcode.unwrap(), a.unwrap(), b.unwrap(), c.unwrap());
                Sample {
                    before,
                    after,
                    opcode,
                    a,
                    b,
                    c,
                }
            })
            .collect()
    };    
}

fn map_opcodes() -> HashMap<usize, Opcode> {
    let mut candidates: HashMap<usize, HashSet<Opcode>> = HashMap::new();
    for sample in SAMPLES.iter() {
        let sample_candidates = sample.opcode_candidates();
        match candidates.entry(sample.opcode) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().retain(|opcode| sample_candidates.contains(opcode));

            },
            Entry::Vacant(entry) => {
                entry.insert(sample_candidates);
            }
        }
    }
    let mut mapping = HashMap::new();
    let mut mapped = HashSet::new();
    for (&n, candidates) in candidates.iter().cycle() {
        if let Some(&opcode) = candidates.difference(&mapped).single() {
            mapping.insert(n, opcode);
            mapped.insert(opcode);
        }

        if mapping.len() >= OPCODES.len() { break; }
    }

    mapping
}

trait IteratorExt: Iterator {
    fn single(self) -> Option<Self::Item>;
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
}

fn parse_program(opcode_mapping: &HashMap<usize, Opcode>) -> Vec<Inst> {
    include_str!("input2.txt")
        .lines()
        .map(|line| {
            let (opcode, a, b, c) = scan_fmt!(line, "{d} {d} {d} {d}", usize, usize, usize, usize);
            let (opcode, a, b, c) = (opcode_mapping[&opcode.unwrap()], a.unwrap(), b.unwrap(), c.unwrap());
            Inst::new(opcode, a, b, c)
        })
        .collect()
}

fn part1() {
    println!("{}", SAMPLES.iter().filter(|s| s.opcode_candidates().len() >= 3).count());
}

fn part2() {
    let map = map_opcodes();
    let program = parse_program(&map);
    let mut reg = [0; 4];
    for inst in program {
        inst.exec(&mut reg);
    }
    println!("{}", reg[0]);
}

fn main() {
    part1();
    part2();
}