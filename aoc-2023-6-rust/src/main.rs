use std::{fs::read_to_string, str::FromStr, time::Instant};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();
    let (time, distance) = input.split_once('\n').unwrap();
    let time: f64 = time
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: f64 = distance
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let discriminant = (time * time - 4.0 * (distance + 1.0)).sqrt();
    let x1 = ((time - discriminant) / 2.0).ceil() as usize;
    let x2 = ((time + discriminant) / 2.0).floor() as usize;
    let result = x2 - x1 + 1;
    println!("{result:?} obtained in {:?}", Instant::now() - start);
}
