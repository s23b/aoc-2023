use std::fs::read_to_string;

const DIGITS: [(&'static str, u32); 19] = [
    ("0", 0),
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = DIGITS
                .iter()
                .filter_map(|digit| line.find(digit.0).map(|pos| (digit.1, pos)))
                .min_by_key(|(_, pos)| *pos);
            let last = DIGITS
                .iter()
                .filter_map(|digit| line.rfind(digit.0).map(|pos| (digit.1, pos)))
                .max_by_key(|(_, pos)| *pos);
            first.unwrap().0 * 10 + last.unwrap().0
        })
        .sum();
    println!("{result:?}");
}
