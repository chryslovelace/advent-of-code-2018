const INPUT: &str = include_str!("input.txt");

pub fn coordinates() -> impl Iterator<Item = (u32, u32)> {
    INPUT.lines().map(|line| {
        let mut split = line.split(", ");
        (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
    })
}