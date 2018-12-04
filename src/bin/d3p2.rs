use advent_of_code_2018::d3::{claims, fabric_claimed};
use itertools::iproduct;

fn main() {
    let claimed = fabric_claimed();
    for claim in claims() {
        if iproduct!(claim.x..claim.x + claim.width, claim.y..claim.y + claim.height).all(|x| claimed[&x] == 1) {
            println!("{}", claim.id);
            return;
        }
    }
}
