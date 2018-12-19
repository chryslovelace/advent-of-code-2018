use lazy_static::lazy_static;
use scan_fmt::scan_fmt;
use std::str::FromStr;

type Registers = [usize; 6];

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

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Opcode::*;
        Ok(match s {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => return Err(()),
        })
    }
}

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
            Gtir => {
                if self.a > registers[self.b] {
                    1
                } else {
                    0
                }
            }
            Gtri => {
                if registers[self.a] > self.b {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                if registers[self.a] > registers[self.b] {
                    1
                } else {
                    0
                }
            }
            Eqir => {
                if self.a == registers[self.b] {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                if registers[self.a] == self.b {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                if registers[self.a] == registers[self.b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

struct Program {
    ip: usize,
    insts: Vec<Inst>,
}

impl Program {
    fn run(&self, reg: &mut Registers) {
        while reg[self.ip] < self.insts.len() {
            self.insts[reg[self.ip]].exec(reg);
            reg[self.ip] += 1;
        }
    }
}

lazy_static! {
    static ref PROGRAM: Program = {
        let mut lines = include_str!("input.txt").lines();
        let ip = lines.next().unwrap()[4..5].parse().unwrap();
        let insts = lines
            .map(|line| {
                let (opcode, a, b, c) = scan_fmt!(line, "{} {} {} {}", Opcode, usize, usize, usize);
                Inst::new(opcode.unwrap(), a.unwrap(), b.unwrap(), c.unwrap())
            })
            .collect();
        Program { ip, insts }
    };
}

fn part1() {
    let mut reg = [0; 6];
    PROGRAM.run(&mut reg);
    println!("{}", reg[0]);
}

fn part2() {
    let mut reg = [1, 0, 0, 0, 0, 0];
    PROGRAM.run(&mut reg);
    println!("{}", reg[0]);
}

fn main() {
    part1();
    part2();
}
