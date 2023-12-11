use std::{collections::BTreeSet, fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let result: usize = input
        .lines()
        .map(|line| {
            let (winning, had) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning = winning
                .split_whitespace()
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect::<BTreeSet<_>>();
            had.split_whitespace()
                .map(usize::from_str)
                .map(Result::unwrap)
                .filter(|number| winning.contains(number))
                .count()
                .checked_sub(1)
                .map(|count| 1usize << count)
                .unwrap_or_default()
        })
        .sum();
    println!("{result:?}");
}
