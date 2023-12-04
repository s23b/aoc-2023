use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let result: u32 = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split(&[';', ','])
                .map(|cube| {
                    let (count, color) = cube.rsplit_once(' ').unwrap();
                    let count: u32 = count.trim().parse().unwrap();

                    match color {
                        "red" => (count, 0),
                        "green" => (count, 1),
                        "blue" => (count, 2),
                        _ => panic!(),
                    }
                })
                .fold([0, 0, 0], |mut rgb, (count, i)| {
                    rgb[i] = rgb[i].max(count);
                    rgb
                })
                .into_iter()
                .product::<u32>()
        })
        .sum();
    println!("{result:?}");
}
