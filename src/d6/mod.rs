const INPUT: &str = include_str!("input.txt");

pub fn coordinates() -> impl Iterator<Item = (i32, i32)> {
    INPUT.lines().map(|line| {
        let mut split = line.split(", ");
        (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    })
}

pub fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
