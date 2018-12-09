use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

const NUM_PLAYERS: usize = 455;
const NUM_MARBLES: usize = 71223;

struct Marble {
    value: usize,
    link: LinkedListLink,
}

impl Marble {
    fn new(value: usize) -> Box<Self> {
        Box::new(Marble {
            value,
            link: LinkedListLink::new(),
        })
    }
}

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

macro_rules! cw {
    ($cursor:expr) => {
        let cursor = &mut $cursor;
        cursor.move_next();
        if cursor.is_null() {
            cursor.move_next();
        }
    };
}

macro_rules! ccw {
    ($cursor:expr) => {
        let cursor = &mut $cursor;
        cursor.move_prev();
        if cursor.is_null() {
            cursor.move_prev();
        }
    };
}

struct Game {
    num_players: usize,
    num_marbles: usize,
}

impl Game {
    fn new(num_players: usize, num_marbles: usize) -> Self {
        Game {
            num_players,
            num_marbles,
        }
    }

    fn get_high_score(&self) -> usize {
        let mut scores = HashMap::new();
        let mut circle = LinkedList::new(MarbleAdapter::new());
        circle.push_front(Marble::new(0));
        let mut curr_player = 0;
        let mut curr_marble = circle.front_mut();
        for marble in 1..=self.num_marbles {
            if marble % 23 == 0 {
                let player_score = scores.entry(curr_player).or_insert(0);
                *player_score += marble;
                for _ in 0..7 {
                    ccw!(curr_marble);
                }
                let removed = curr_marble.remove().unwrap();
                *player_score += removed.value;
            } else {
                cw!(curr_marble);
                curr_marble.insert_after(Marble::new(marble));
                cw!(curr_marble);
            }
            curr_player = (curr_player + 1) % self.num_players;
        }
        *scores.values().max().unwrap()
    }
}

fn part1() {
    let game = Game::new(NUM_PLAYERS, NUM_MARBLES);
    println!("{}", game.get_high_score());
}

fn part2() {
    let game = Game::new(NUM_PLAYERS, NUM_MARBLES * 100);
    println!("{}", game.get_high_score());
}
