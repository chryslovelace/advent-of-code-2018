use std::{str::FromStr, option::NoneError};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default)]
pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
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
        let (id, x, y, width, height) = scan_fmt!(s, "#{d} @ {d},{d}: {d}x{d}", u32, u32, u32, u32, u32);
        Ok(Claim {
            id: id?,
            x: x?,
            y: y?,
            width: width?,
            height: height?
        })
    }
}

pub fn parsed_input() -> impl Iterator<Item = Claim> {
    INPUT.lines().map(|x| x.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::parsed_input;

    #[test]
    fn test_parse(){
        println!("{:?}", parsed_input().next().unwrap());
    }
}
