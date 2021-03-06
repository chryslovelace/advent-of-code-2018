use itertools::iproduct;
use std::cmp::max;

const SERIAL_NUMBER: i32 = 3613;

fn get_power((x, y): (i32, i32)) -> i32 {
    let rack_id = x + 10;
    (rack_id * y + SERIAL_NUMBER) * rack_id / 100 % 10 - 5
}

fn part1() {
    let (x, y) = iproduct!(1..299, 1..299)
        .max_by_key(|&(x, y)| iproduct!(x..x + 3, y..y + 3).map(get_power).sum::<i32>())
        .unwrap();
    println!("{},{}", x, y);
}

fn part2() {
    let (x, y, size, _) = iproduct!(1..301, 1..301)
        .flat_map(|(x, y)| {
            (1..(302 - max(x, y))).scan(0, move |power, size| {
                *power += (x..x + size)
                    .map(|x| get_power((x, y + size - 1)))
                    .sum::<i32>();
                *power += (y..y + size - 1)
                    .map(|y| get_power((x + size - 1, y)))
                    .sum::<i32>();
                Some((x, y, size, *power))
            })
        })
        .max_by_key(|&(_, _, _, power)| power)
        .unwrap();
    println!("{},{},{}", x, y, size);
}

fn main() {
    part1();
    part2();
}
