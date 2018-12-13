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

#[derive(Debug, Clone, Copy)]
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
struct Track(Vec<Vec<TrackType>>);
    
impl Track {
    fn at(&self, x: usize, y: usize) -> TrackType {        
        self.0[y][x] 
    }
}

#[derive(Debug)]
struct State {
    carts: Vec<Cart>,
    track: Track,
}

impl State {
    fn new(input: &str) -> Self {
        let mut carts = Vec::new();
        let track = input.lines().enumerate().map(|(y, line)| line.bytes().enumerate().map(|(x, c)| {
            use self::{Direction::*, TrackType::*};
            match c {
                b'|' => Vertical,
                b'-' => Horizontal,
                b'\\' => Diagonal,
                b'/' => Antidiagonal,
                b'+' => Intersection,
                b'^' => {
                    carts.push(Cart::new(x, y, Up));
                    Vertical
                },
                b'v' => {
                    carts.push(Cart::new(x, y, Down));
                    Vertical
                },
                b'<' => {
                    carts.push(Cart::new(x, y, Left));
                    Horizontal
                },
                b'>' => {
                    carts.push(Cart::new(x, y, Right));
                    Horizontal
                },
                _ => Empty,
            }
        }).collect()).collect();

        carts.sort_by_key(|&Cart { pos: (x, y), .. }| (y, x));

        State {
            carts,
            track: Track(track),
        }
    }


    fn cart_at(carts: &[Cart], x: usize, y: usize) -> Option<usize> {
        carts
            .binary_search_by_key(&(y, x), |&Cart { pos: (x, y), .. }| (y, x))
            .ok()
    }

    fn tick(&mut self) -> Vec<(usize, usize)> {
        let mut next_carts = Vec::new();
        let mut collisions = Vec::new();
        let mut collided_idxs = Vec::new();
        for (i, cart) in self.carts.iter().enumerate() {
            if collided_idxs.contains(&i) {
                continue;
            }
            let (x, y) = cart.next_pos();
            if let Some(collision) = State::cart_at(&next_carts, x, y)
            {
                collisions.push((x, y));
                next_carts.remove(collision);
            } else if let Some(collision) = State::cart_at(&self.carts[i + 1..], x, y) {
                collisions.push((x, y));
                collided_idxs.push(collision + i + 1);
            } else {                
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
        }
        self.carts = next_carts;
        collisions
    }
}

fn main() {
    let mut state = State::new(include_str!("input.txt"));
    let (x, y) = loop {
        let collisions = state.tick();
        if !collisions.is_empty() {
            break collisions[0];
        }
    };
    println!("{},{}", x, y);
    while state.carts.len() > 1 {
        state.tick();
    }
    let (x, y) = state.carts[0].pos;
    println!("{},{}", x, y);
}
