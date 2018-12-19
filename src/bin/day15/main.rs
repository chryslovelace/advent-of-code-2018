#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
}

#[derive(Eq, PartialEq)]
enum Allegiance {
    Elf,
    Goblin,
}

struct Unit {
    allegiance: Allegiance,
    pos: (usize, usize),
    hp: usize,
}

impl Unit {
    fn new(allegiance: Allegiance, pos: (usize, usize)) -> Self {
        Unit {
            allegiance,
            pos,
            hp: 300,
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    units: Vec<Unit>,
    rounds_completed: usize,
}

impl Board {
    fn new(input: &str) -> Self {
        let mut units = Vec::new();
        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Floor,
                        'E' => {
                            units.push(Unit::new(Allegiance::Elf, (x, y)));
                            Tile::Floor
                        }
                        'G' => {
                            units.push(Unit::new(Allegiance::Goblin, (x, y)));
                            Tile::Floor
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Board {
            tiles,
            units,
            rounds_completed: 0,
        }
    }

    fn is_open(&self, (x, y): (usize, usize)) -> bool {
        self.tiles[y][x] == Tile::Floor && self.units.iter().all(|u| u.pos != (x, y))
    }

    fn adjacent_open_tiles(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut candidates = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            candidates.push((x - 1, y));
        }
        if y > 0 {
            candidates.push((x, y - 1));
        }
        candidates.retain(|&pos| self.is_open(pos));
        candidates.sort_by_key(|&(x, y)| (y, x));
        candidates
    }

    fn round(&mut self) -> bool {
        for i in 0..self.units.len() {
            let enemies = self.units[..i]
                .iter()
                .chain(self.units[i + 1..].iter())
                .filter(|u| u.allegiance == self.units[i].allegiance);
        }
        self.rounds_completed += 1;
        true
    }

    fn outcome(&self) -> usize {
        self.rounds_completed * self.units.iter().map(|u| u.hp).sum::<usize>()
    }
}

fn main() {
    let mut board = Board::new(include_str!("input.txt"));
    while board.round() {}
    println!("{}", board.outcome());
}
