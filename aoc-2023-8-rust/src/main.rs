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
    let sequence = sequence
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Box<[usize]>>();

    let start_id = parse_id("A");
    let goal_id = parse_id("Z");
    let result: usize = map
        .keys()
        .filter(|id| *id & 0xff == start_id)
        .map(|mut current| {
            sequence
                .iter()
                .cycle()
                .take_while(|dir| {
                    current = &map[&current][**dir];
                    current & 0xff != goal_id
                })
                .count()
                + 1
        })
        .fold(1, num::integer::lcm);
    println!("{result:?} obtained in {:?}", Instant::now() - start);
}
