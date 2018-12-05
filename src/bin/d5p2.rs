use advent_of_code_2018::d5::{input, react};

fn main() {
    let result = replaced_inputs().map(|s| react(&s).len()).min().unwrap();
    println!("{}", result);
}

fn replaced_inputs() -> impl Iterator<Item = String> {
    (b'a'..=b'z').map(|c| input().replace(|d: char| c as char == d.to_ascii_lowercase(), ""))
}
