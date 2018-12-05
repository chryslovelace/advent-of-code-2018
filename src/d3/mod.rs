use scan_fmt::scan_fmt;
use std::{collections::HashMap, option::NoneError, str::FromStr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct ClaimParseError;

impl From<NoneError> for ClaimParseError {
    fn from(_err: NoneError) -> Self {
        ClaimParseError
    }
}

impl FromStr for Claim {
    type Err = ClaimParseError;
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

pub fn claims() -> impl Iterator<Item = Claim> {
    INPUT.lines().map(|x| x.parse().unwrap())
}

pub fn fabric_claimed() -> HashMap<(u32, u32), u32> {
    let mut map = HashMap::new();
    for claim in claims() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    map
}
