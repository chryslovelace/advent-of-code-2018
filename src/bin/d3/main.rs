#![feature(try_trait)]

use itertools::iproduct;
use lazy_static::lazy_static;
use scan_fmt::scan_fmt;
use std::{collections::HashMap, option::NoneError, str::FromStr};

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref CLAIMS: Vec<Claim> = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    static ref FABRIC_CLAIMED: HashMap<(u32, u32), u32> = {
        let mut map = HashMap::new();
        for claim in CLAIMS.iter() {
            for x in claim.x..claim.x + claim.width {
                for y in claim.y..claim.y + claim.height {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        map
    };
}

fn part1() {
    println!("{}", FABRIC_CLAIMED.values().filter(|&&v| v > 1).count());
}

fn part2() {
    let claim = CLAIMS
        .iter()
        .find(|claim| {
            iproduct!(
                claim.x..claim.x + claim.width,
                claim.y..claim.y + claim.height
            )
            .all(|x| FABRIC_CLAIMED[&x] == 1)
        })
        .unwrap();

    println!("{}", claim.id);
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = NoneError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, x, y, width, height) =
            scan_fmt!(s, "#{d} @ {d},{d}: {d}x{d}", u32, u32, u32, u32, u32);
        Ok(Claim {
            id: id?,
            x: x?,
            y: y?,
            width: width?,
            height: height?,
        })
    }
}
