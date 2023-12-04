use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let result: u32 = input
        .lines()
        .map(|line| {
            let (game_no, game) = line.trim_start_matches("Game ").split_once(':').unwrap();
            let game_no: u32 = game_no.parse().unwrap();
            let lost = game.split(';').any(|round| {
                round.split(',').any(|cube| {
                    let (count, color) = cube.rsplit_once(' ').unwrap();
                    let count: u32 = count.trim().parse().unwrap();
                    match color {
                        "red" => count > 12,
                        "green" => count > 13,
                        "blue" => count > 14,
                        _ => panic!(),
                    }
                })
            });
            if lost {
                0
            } else {
                game_no
            }
        })
        .sum();
    println!("{result:?}");
}
