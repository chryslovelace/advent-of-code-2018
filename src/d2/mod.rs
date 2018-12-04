const INPUT: &str = include_str!("input.txt");

pub fn ids() -> impl Iterator<Item = &'static str> {
    INPUT.lines()
}
