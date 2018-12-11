use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Star {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Star {
    fn take_steps(&mut self, steps: i32) {
        self.pos = (
            self.pos.0 + self.vel.0 * steps,
            self.pos.1 + self.vel.1 * steps,
        );
    }
}

struct Rect {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl Rect {
    fn area(&self) -> i64 {
        i64::from(self.right - self.left) * i64::from(self.bottom - self.top)
    }
}

lazy_static! {
    static ref STARS: Vec<Star> = {
        include_str!("input.txt")
            .lines()
            .map(|line| Star {
                pos: (
                    line[10..16].trim().parse().unwrap(),
                    line[18..24].trim().parse().unwrap(),
                ),
                vel: (
                    line[36..38].trim().parse().unwrap(),
                    line[40..42].trim().parse().unwrap(),
                ),
            })
            .collect()
    };
}

fn get_bbox(stars: &[Star]) -> Rect {
    let (left, right) = stars
        .iter()
        .map(|s| s.pos.0)
        .minmax()
        .into_option()
        .unwrap();
    let (top, bottom) = stars
        .iter()
        .map(|s| s.pos.1)
        .minmax()
        .into_option()
        .unwrap();
    Rect {
        top,
        left,
        bottom,
        right,
    }
}

fn render(stars: &[Star], bbox: &Rect) {
    let positions: HashSet<_> = stars.iter().map(|s| s.pos).collect();
    for y in bbox.top..=bbox.bottom {
        for x in bbox.left..=bbox.right {
            if positions.contains(&(x, y)) {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    let mut stars = STARS.clone();
    let mut bboxes = Vec::with_capacity(20000);
    for _ in 0..20000 {
        bboxes.push(get_bbox(&stars));
        for star in &mut stars {
            star.take_steps(1);
        }
    }
    let (step, bbox) = bboxes
        .iter()
        .enumerate()
        .min_by_key(|(_, bbox)| bbox.area())
        .unwrap();

    for star in &mut stars {
        star.take_steps(step as i32 - 20000);
    }
    render(&stars, &bbox);
    println!("{}", step);
}
