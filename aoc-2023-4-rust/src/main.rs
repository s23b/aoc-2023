use std::{collections::BTreeSet, fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let mut result = vec![1; input.lines().count()];
    input.lines().enumerate().for_each(|(line_no, line)| {
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
            .enumerate()
            .for_each(|(i, _)| {
                let i = line_no + i + 1;
                result[i] = result[i] + result[line_no];
            });
    });
    let result: usize = result.iter().sum();
    println!("{result:?}");
}
