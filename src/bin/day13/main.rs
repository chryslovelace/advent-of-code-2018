use std::fmt;

#[derive(Debug, Clone, Copy)]
enum TrackType {
    Empty,
    Vertical,     // |
    Horizontal,   // -
    Diagonal,     // \
    Antidiagonal, // /
    Intersection, // +
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn turn_right(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[derive(Debug)]
struct Cart {
    pos: (usize, usize),
    dir: Direction,
    intersections: usize,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Cart {
            pos: (x, y),
            dir,
            intersections: 0,
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        let (x, y) = self.pos;
        match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn next_dir(&self, track_type: TrackType) -> Direction {
        use self::{Direction::*, TrackType::*};
        match (self.dir, track_type) {
            (dir, Vertical) => dir,
            (dir, Horizontal) => dir,
            (Up, Diagonal) => Left,
            (Down, Diagonal) => Right,
            (Left, Diagonal) => Up,
            (Right, Diagonal) => Down,
            (Up, Antidiagonal) => Right,
            (Down, Antidiagonal) => Left,
            (Left, Antidiagonal) => Down,
            (Right, Antidiagonal) => Up,
            (dir, Intersection) => match self.intersections % 3 {
                0 => dir.turn_left(),
                1 => dir,
                2 => dir.turn_right(),
                _ => unreachable!("problem"),
            },
            (dir, tt) => {
                println!("{:?}, {:?}, {:?}", self.pos, dir, tt);
                unreachable!("also problem")
            }
        }
    }
}

#[derive(Debug)]
struct Track {
    track: String,
    width: usize,
}

impl Track {
    fn at(&self, x: usize, y: usize) -> TrackType {
        let i = self.width * y + x;
        match &self.track[i..i + 1] {
            "|" => TrackType::Vertical,
            "-" => TrackType::Horizontal,
            "\\" => TrackType::Diagonal,
            "/" => TrackType::Antidiagonal,
            "+" => TrackType::Intersection,
            _ => TrackType::Empty,
        }
    }
}

#[derive(Debug)]
struct State {
    carts: Vec<Cart>,
    track: Track,
}

impl State {
    fn new(input: &str) -> Self {
        let width = input.chars().position(|c| c == '\n' || c == '\r').unwrap();
        let mut carts = Vec::new();
        let mut track = String::new();
        for (i, mut c) in input.chars().enumerate() {
            match c {
                // look at these cute friends!!!!!!!!!!
                '^' | 'v' | '<' | '>' => {
                    let (dir, c) = match c {
                        '^' => (Direction::Up, '|'),
                        'v' => (Direction::Down, '|'),
                        '<' => (Direction::Left, '-'),
                        '>' => (Direction::Right, '-'),
                        _ => unreachable!("somethings bad"),
                    };
                    carts.push(Cart::new(i % width, i / width, dir));
                    track.push(c);
                }
                _ => track.push(c),
            }
        }

        carts.sort_by_key(|&Cart { pos: (x, y), .. }| (y, x));

        State {
            carts,
            track: Track { track, width },
        }
    }

    fn pos(&self, i: usize) -> (usize, usize) {
        (i % self.track.width, i / self.track.width)
    }

    fn cart_at(carts: &[Cart], x: usize, y: usize) -> Option<&Cart> {
        carts
            .binary_search_by_key(&(y, x), |&Cart { pos: (x, y), .. }| (y, x))
            .ok()
            .map(|i| &carts[i])
    }

    fn tick(&mut self) -> Result<(), (usize, usize)> {
        let mut next_carts = Vec::new();
        for (i, cart) in self.carts.iter().enumerate() {
            let (x, y) = cart.next_pos();
            if let Some(collision) = State::cart_at(&next_carts, x, y)
                .or_else(|| State::cart_at(&self.carts[i + 1..], x, y))
            {
                return Err(collision.pos);
            }
            let track_type = self.track.at(x, y);
            let dir = cart.next_dir(track_type);
            let intersections = if let TrackType::Intersection = track_type {
                cart.intersections + 1
            } else {
                cart.intersections
            };
            let i = next_carts
                .binary_search_by_key(&(y, x), |&Cart { pos: (x, y), .. }| (y, x))
                .unwrap_or_else(|x| x);
            next_carts.insert(
                i,
                Cart {
                    pos: (x, y),
                    dir,
                    intersections,
                },
            );
        }
        self.carts = next_carts;
        Ok(())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.track.track.chars().enumerate() {
            let (x, y) = self.pos(i);
            if let Some(cart) = State::cart_at(&self.carts, x, y) {
                let c = match cart.dir {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };
                c.fmt(f)?;
            } else {
                c.fmt(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut state = State::new(include_str!("input.txt"));
    let (x, y) = loop {
        println!("{}", state);
        if let Err(pos) = state.tick() {
            break pos;
        }
    };
    println!("{},{}", x, y);
}
