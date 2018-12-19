use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt, usize,
};

#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Allegiance {
    Elf,
    Goblin,
}

#[derive(Eq, PartialEq, Clone, Copy)]
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

    fn adjacent_tiles(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut candidates = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            candidates.push((x - 1, y));
        }
        if y > 0 {
            candidates.push((x, y - 1));
        }
        candidates.sort_by_key(|&(x, y)| (y, x));
        candidates
    }

    fn adjacent_open_tiles(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut candidates = self.adjacent_tiles(pos);
        candidates.retain(|&pos| self.is_open(pos));
        candidates
    }

    fn round(&mut self) -> bool {
        for unit in &self.units {
            if let Some(enemy) = self
                .adjacent_tiles(unit.pos)
                .iter()
                .filter_map(|&pos| {
                    self.units
                        .iter()
                        .find(|u| u.pos == pos && u.allegiance != unit.allegiance)
                })
                .next()
            {
                // attack enemy
            } else {
                let (_, (_, next_pos)) = self
                    .units
                    .iter()
                    .filter(|u| u.allegiance != unit.allegiance)
                    .flat_map(|u| self.adjacent_open_tiles(u.pos))
                    .filter_map(|pos| self.shortest_path(unit.pos, pos).map(|sp| (pos, sp)))
                    .min_by_key(|&((x, y), (len, _))| (len, y, x))
                    .unwrap();
                // move to new position
            }
        }
        self.rounds_completed += 1;
        true
    }

    fn shortest_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(usize, (usize, usize))> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            dist: usize,
            pos: (usize, usize),
            first_step: Option<(usize, usize)>,
        }

        impl Ord for State {
            fn cmp(&self, other: &State) -> Ordering {
                other
                    .dist
                    .cmp(&self.dist)
                    .then_with(|| self.pos.cmp(&other.pos))
                    .then_with(|| flip(other.pos).cmp(&flip(self.pos)))
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &State) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();
        distances.insert(start, 0);
        heap.push(State {
            dist: 0,
            pos: start,
            first_step: None,
        });

        while let Some(State {
            dist,
            pos,
            first_step,
        }) = heap.pop()
        {
            if pos == goal {
                return Some((dist, first_step.unwrap()));
            }
            if dist > *distances.entry(pos).or_insert(usize::MAX) {
                continue;
            }
            for next_step in self.adjacent_open_tiles(pos) {
                let next = State {
                    dist: dist + 1,
                    pos: next_step,
                    first_step: first_step.or(Some(next_step)),
                };

                if next.dist < *distances.entry(pos).or_insert(usize::MAX) {
                    heap.push(next);
                    distances.insert(next.pos, next.dist);
                }
            }
        }

        None
    }

    fn outcome(&self) -> usize {
        self.rounds_completed * self.units.iter().map(|u| u.hp).sum::<usize>()
    }
}

fn flip((x, y): (usize, usize)) -> (usize, usize) {
    (y, x)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(unit) = self.units.iter().find(|u| u.pos == (x, y)) {
                    match unit.allegiance {
                        Allegiance::Elf => 'E'.fmt(f)?,
                        Allegiance::Goblin => 'G'.fmt(f)?,
                    }
                } else {
                    match tile {
                        Tile::Wall => '#'.fmt(f)?,
                        Tile::Floor => '.'.fmt(f)?,
                    }
                };
            }
            '\n'.fmt(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new(include_str!("input.txt"));
    while board.round() {
        println!("{}", board);
    }
    println!("{}", board.outcome());
}
