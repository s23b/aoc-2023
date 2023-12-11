use std::{fs::read_to_string, str::FromStr, time::Instant};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();
    let (time, distance) = input.split_once('\n').unwrap();
    let result = time
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(f32::from_str)
        .map(Result::unwrap)
        .zip(
            distance
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(f32::from_str)
                .map(Result::unwrap),
        )
        .map(|(time, distance)| {
            let discriminant = (time * time - 4.0 * (distance + 1.0)).sqrt();
            let x1 = ((time - discriminant) / 2.0).ceil() as usize;
            let x2 = ((time + discriminant) / 2.0).floor() as usize;
            x2 - x1 + 1
        })
        .product::<usize>();
    println!("{result:?} obtained in {:?}", Instant::now() - start);
}
