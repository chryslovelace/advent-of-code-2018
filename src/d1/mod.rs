const INPUT: &str = include_str!("input.txt");

pub fn frequency_changes() -> impl Iterator<Item = i32> + Clone {
    INPUT.lines().map(|line| line.parse().unwrap())
}
