use advent_of_code_2018::d3::fabric_claimed;

fn main() {
    println!("{}", fabric_claimed().values().filter(|&&v| v > 1).count());
}
