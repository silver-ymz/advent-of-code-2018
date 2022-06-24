// requirements: regex = "1.5.6", lazy_static = "1.4.0"
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read, str::FromStr};

fn main() {
    let mut fabric = [[0; 1001]; 1001];
    let mut file = File::open("inputs/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let claims: Vec<Claim> = content
        .lines()
        .map(|s| s.parse::<Claim>().unwrap())
        .collect();
    // println!("{} {}", claims.iter().map(|c| c.x + c.width).max().unwrap(), claims.iter().map(|c| c.y + c.height).max().unwrap());
    for claim in &claims {
        fabric[claim.x][claim.y] += 1;
        fabric[claim.x + claim.width][claim.y + claim.height] += 1;
        fabric[claim.x + claim.width][claim.y] -= 1;
        fabric[claim.x][claim.y + claim.height] -= 1;
    }
    for col in 0..1000 {
        for row in 1..1000 {
            fabric[col][row] += fabric[col][row - 1];
        }
    }
    for row in 0..1000 {
        for col in 1..1000 {
            fabric[col][row] += fabric[col - 1][row];
        }
    }
    println!(
        "{}",
        claims
            .iter()
            .find(|&c| {
                for col in c.x..c.x + c.width {
                    for row in c.y..c.y + c.height {
                        if fabric[col][row] != 1 {
                            return false;
                        }
                    }
                }
                true
            })
            .unwrap()
            .id
    );
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)")
                    .unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Claim {
            id: caps["id"].parse().unwrap(),
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        })
    }
}
