const INPUT : &'static str = include_str!("input.txt");

pub fn parsed_input() -> impl Iterator<Item = i32> + Clone {
    INPUT.lines().map(|line| {
        let neg = line.starts_with("-");
        let mut num: i32 = line[1..].parse().unwrap();
        if neg { num *= -1; }
        num
    })
}