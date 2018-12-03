use advent_of_code_2018::d3::*;
use std::collections::HashMap;
use itertools::iproduct;

fn main() {
    let mut map = HashMap::new();
    for claim in parsed_input() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    for claim in parsed_input() {
        if iproduct!(claim.x..claim.x + claim.width, claim.y..claim.y + claim.height).all(|x| map[&x] == 1) {
            println!("{}", claim.id);
            return;
        }
    }
}
