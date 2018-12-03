use advent_of_code_2018::d3::*;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    for claim in parsed_input() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    println!("{}", map.values().filter(|&&v| v > 1).count());
}