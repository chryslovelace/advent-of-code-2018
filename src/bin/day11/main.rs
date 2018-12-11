use itertools::iproduct;
use std::collections::HashMap;
use std::cmp::max;

const SERIAL_NUMBER: i64 = 3613;

fn get_power((x, y): (i64, i64)) -> i64 {
    let rack_id = x + 10;
    (rack_id * y + SERIAL_NUMBER) * rack_id / 100 % 10 - 5
}

fn part1() {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    for (x, y) in iproduct!(1..299, 1..299) {
        let power = iproduct!(x..x+3, y..y+3).map(get_power).sum();
        map.insert((x, y), power);
    }
    let (point, _) = map.iter().max_by_key(|(_, &power)| power).unwrap();
    println!("{:?}", point);
}

fn part2() {
    let mut map: HashMap<(i64, i64, i64), i64> = HashMap::new();
    for (x, y) in iproduct!(1..=300, 1..=300) {
        let mut power = 0;
        for size in 1..=(301 - max(x, y)) {            
            power += (x..=x+size).zip(y..=y+size).map(get_power).sum::<i64>();
            map.insert((x, y, size), power);
        }
    }
    let (square, _) = map.iter().max_by_key(|(_, &power)| power).unwrap();
    println!("{:?}", square);
}

fn main() {
    part1();
    //part2();
}