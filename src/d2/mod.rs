const INPUT: &str = include_str!("input.txt");

pub fn parsed_input() -> impl Iterator<Item = &'static str> {
    INPUT.lines()
}
