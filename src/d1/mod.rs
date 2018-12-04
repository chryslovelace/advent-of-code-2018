const INPUT: &str = include_str!("input.txt");

pub fn frequency_changes() -> impl Iterator<Item = i32> + Clone {
    INPUT.lines().map(|line| {
        let mut num: i32 = line[1..].parse().unwrap();
        if line.starts_with('-') {
            num *= -1;
        }
        num
    })
}
