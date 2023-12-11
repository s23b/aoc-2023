use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();
    let (sequence, map) = input.split_once("\n\n").unwrap();
    let parse_id = |s: &str| s.chars().map(|c| c as usize).fold(0, |s, c| (s << 8) + c);
    let map: HashMap<usize, [usize; 2]> = map
        .lines()
        .map(|line| {
            let (id, dir) = line.split_once(" = (").unwrap();
            let (left, right) = dir.split_once(')').unwrap().0.split_once(", ").unwrap();
            (parse_id(id), [parse_id(left), parse_id(right)])
        })
        .collect();
    let mut current = parse_id("AAA");
    let goal = parse_id("ZZZ");
    let result = sequence
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .cycle()
        .take_while(|dir| {
            current = map[&current][*dir];
            current != goal
        })
        .count()
        + 1;
    println!("{result:?} obtained in {:?}", Instant::now() - start);
}
