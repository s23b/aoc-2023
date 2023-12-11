use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

struct LutItem {
    source: usize,
    dest: usize,
    count: usize,
}

impl FromStr for LutItem {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .split_whitespace()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect::<Box<[_]>>();
        Ok(LutItem {
            source: s[1],
            dest: s[0],
            count: s[2],
        })
    }
}

fn main() {
    let input = read_to_string("src/test_input.txt").unwrap();
    let (seeds, luts) = input.split_once("\n\n").unwrap();
    let luts = luts
        .split("\n\n")
        .map(|lut| {
            let mut lut = lut
                .split_once(":\n")
                .unwrap()
                .1
                .lines()
                .map(|line| line.parse::<LutItem>().unwrap())
                .collect::<Box<_>>();
            lut.sort_unstable_by_key(|item| item.source);
            lut
        })
        .collect::<Box<[_]>>();
    let result = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(usize::from_str)
        .map(Result::unwrap)
        .map(|seed| {
            luts.iter().fold(seed, |seed, lut| {
                match lut
                    .partition_point(|item| item.source <= seed)
                    .checked_sub(1)
                {
                    None => seed,
                    Some(n) => {
                        let count = seed - lut[n].source;
                        if count > lut[n].count {
                            seed
                        } else {
                            lut[n].dest + count
                        }
                    }
                }
            })
        })
        .min()
        .unwrap();
    println!("{result:?}");
}
