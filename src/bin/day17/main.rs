#![feature(range_contains)]

use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use scan_fmt::scan_fmt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::repeat,
    ops::RangeInclusive,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Clay,
    FlowingWater,
    RestingWater,
}

impl Tile {
    fn is_passable(&self) -> bool {
        match *self {
            Tile::Sand => true,
            Tile::Clay => false,
            Tile::FlowingWater => true,
            Tile::RestingWater => false,
        }
    }
}

lazy_static! {
    static ref CLAY_POSITIONS: Vec<(usize, usize)> = {
        include_str!("input.txt")
            .lines()
            .flat_map(|line| {
                let (xmin, xmax, ymin, ymax) = if line.starts_with('x') {
                    let (x, ymin, ymax) = scan_fmt!(line, "x={d}, y={d}..{d}", usize, usize, usize);
                    (x.unwrap(), x.unwrap(), ymin.unwrap(), ymax.unwrap())
                } else {
                    let (y, xmin, xmax) = scan_fmt!(line, "y={d}, x={d}..{d}", usize, usize, usize);
                    (xmin.unwrap(), xmax.unwrap(), y.unwrap(), y.unwrap())
                };
                iproduct!(xmin..=xmax, ymin..=ymax)
            })
            .collect()
    };
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum FlowTask {
    FlowDown(usize, usize),
    FillOrSpill(usize, usize),
}

impl FlowTask {
    fn pos(&self) -> (usize, usize) {
        match *self {
            FlowTask::FlowDown(x, y) => (x, y),
            FlowTask::FillOrSpill(x, y) => (x, y),
        }
    }
}

enum FlowEnd {
    Wall(usize),
    Edge(usize),
}

struct State {
    tiles: HashMap<(usize, usize), Tile>,
    xbounds: RangeInclusive<usize>,
    ybounds: RangeInclusive<usize>,
    flowtasks: VecDeque<FlowTask>,
    seen: HashSet<FlowTask>,
}

impl State {
    fn initial() -> Self {
        let tiles = CLAY_POSITIONS
            .iter()
            .cloned()
            .zip(repeat(Tile::Clay))
            .collect::<HashMap<(usize, usize), Tile>>();
        let (xmin, xmax) = tiles
            .keys()
            .map(|&(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (ymin, ymax) = tiles
            .keys()
            .map(|&(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        let mut flowtasks = VecDeque::new();
        flowtasks.push_back(FlowTask::FlowDown(500, ymin));
        State {
            tiles,
            xbounds: xmin - 1..=xmax + 1,
            ybounds: ymin..=ymax,
            flowtasks,
            seen: HashSet::new(),
        }
    }

    fn at(&self, x: usize, y: usize) -> Tile {
        *self.tiles.get(&(x, y)).unwrap_or(&Tile::Sand)
    }

    fn insert(&mut self, x: usize, y: usize, tile: Tile) {
        if self.xbounds.contains(&x) && self.ybounds.contains(&y) {
            self.tiles.insert((x, y), tile);
        }
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        self.xbounds.contains(&x) && self.ybounds.contains(&y)
    }

    fn task_inbounds(&self, task: &FlowTask) -> bool {
        let (x, y) = task.pos();
        self.inbounds(x, y)
    }

    fn run(&mut self) {
        while let Some(task) = self.flowtasks.pop_front() {
            if self.task_inbounds(&task) && !self.seen.contains(&task) {
                self.seen.insert(task);
                let next = self.perform_task(task);
                self.flowtasks.extend(next);
            }
        }
    }

    fn perform_task(&mut self, task: FlowTask) -> Vec<FlowTask> {
        let mut next = Vec::new();
        match task {
            FlowTask::FlowDown(x, mut y) => {
                while self.inbounds(x, y) && self.at(x, y).is_passable() {
                    self.insert(x, y, Tile::FlowingWater);
                    y += 1;
                }
                if self.inbounds(x, y) {
                    next.push(FlowTask::FillOrSpill(x, y - 1));
                }
            }
            FlowTask::FillOrSpill(x, y) => match self.flow_ends(x, y) {
                (FlowEnd::Wall(left), FlowEnd::Wall(right)) => {
                    for x in left + 1..right {
                        self.insert(x, y, Tile::RestingWater);
                    }
                    next.push(FlowTask::FillOrSpill(x, y - 1));
                }
                (FlowEnd::Wall(left), FlowEnd::Edge(right)) => {
                    for x in left + 1..=right {
                        self.insert(x, y, Tile::FlowingWater);
                    }
                    if self.at(right, y + 1) != Tile::FlowingWater {
                        next.push(FlowTask::FlowDown(right, y + 1));
                    }
                }
                (FlowEnd::Edge(left), FlowEnd::Wall(right)) => {
                    for x in left..right {
                        self.insert(x, y, Tile::FlowingWater);
                    }
                    if self.at(left, y + 1) != Tile::FlowingWater {
                        next.push(FlowTask::FlowDown(left, y + 1));
                    }
                }
                (FlowEnd::Edge(left), FlowEnd::Edge(right)) => {
                    for x in left..=right {
                        self.insert(x, y, Tile::FlowingWater);
                    }
                    if self.at(right, y + 1) != Tile::FlowingWater {
                        next.push(FlowTask::FlowDown(right, y + 1));
                    }
                    if self.at(left, y + 1) != Tile::FlowingWater {
                        next.push(FlowTask::FlowDown(left, y + 1));
                    }
                }
            },
        };
        next
    }

    fn flow_ends(&self, x: usize, y: usize) -> (FlowEnd, FlowEnd) {
        let (mut left, mut right) = (x - 1, x + 1);
        let left = loop {
            if !self.at(left, y).is_passable() {
                break FlowEnd::Wall(left);
            } else if self.at(left, y + 1).is_passable() {
                break FlowEnd::Edge(left);
            } else {
                left -= 1;
            }
        };
        let right = loop {
            if !self.at(right, y).is_passable() {
                break FlowEnd::Wall(right);
            } else if self.at(right, y + 1).is_passable() {
                break FlowEnd::Edge(right);
            } else {
                right += 1;
            }
        };
        (left, right)
    }

    fn watered_tiles(&self) -> usize {
        self.tiles
            .values()
            .filter(|&&tile| tile == Tile::FlowingWater || tile == Tile::RestingWater)
            .count()
    }

    fn resting_water(&self) -> usize {
        self.tiles
            .values()
            .filter(|&&tile| tile == Tile::RestingWater)
            .count()
    }

    fn render(&mut self) {
        use gif::{Encoder, Frame, Repeat, SetParameter};
        use std::fs::File;

        let (width, height) = (
            self.xbounds.clone().count() as u16,
            self.ybounds.clone().count() as u16,
        );
        let color_map = [0, 0, 0, 0xff, 0xff, 0xff, 0, 0, 0xff];
        let mut image = File::create("day17.gif").unwrap();
        let mut encoder = Encoder::new(&mut image, width, height, &color_map).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        while let Some(task) = self.flowtasks.pop_front() {
            if self.task_inbounds(&task) && !self.seen.contains(&task) {
                let frame = Frame {
                    width,
                    height,
                    buffer: self.buffer().into(),
                    delay: 0,
                    ..Frame::default()
                };
                encoder.write_frame(&frame).unwrap();
                self.seen.insert(task);
                let next = self.perform_task(task);
                self.flowtasks.extend(next);
            }
        }
        let frame = Frame {
            width,
            height,
            buffer: self.buffer().into(),
            ..Frame::default()
        };
        encoder.write_frame(&frame).unwrap();
    }

    fn buffer(&self) -> Vec<u8> {
        let mut buffer =
            Vec::with_capacity(self.xbounds.clone().count() * self.ybounds.clone().count());
        for y in self.ybounds.clone() {
            for x in self.xbounds.clone() {
                buffer.push(match self.at(x, y) {
                    Tile::Sand => 0,
                    Tile::Clay => 1,
                    Tile::FlowingWater | Tile::RestingWater => 2,
                });
            }
        }
        buffer
    }
}

fn main() {
    let mut state = State::initial();
    state.run();
    println!("{}", state.watered_tiles());
    println!("{}", state.resting_water());
    State::initial().render();
}
