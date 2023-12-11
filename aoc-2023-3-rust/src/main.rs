use std::{collections::BTreeSet, fs::read_to_string};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();
    let mut mask: Box<[Box<[Vec<usize>]>]> = vec![vec![vec![]; col_count].into(); row_count].into();
    let mut gear_id = 0;
    let digits = input
        .lines()
        .enumerate()
        .map(|(row_id, row)| {
            (
                row_id,
                row.chars()
                    .enumerate()
                    .filter_map(|(col_id, cell)| match cell {
                        digit @ '0'..='9' => Some((col_id, digit.to_digit(10).unwrap())),
                        star @ '*' => {
                            let clamp_add = |x, y: isize, max| {
                                (x as isize + y as isize).clamp(0, max as isize - 1) as usize
                            };
                            (clamp_add(row_id, -1, row_count)..=clamp_add(row_id, 1, row_count))
                                .for_each(|x| {
                                    (clamp_add(col_id, -1, col_count)
                                        ..=clamp_add(col_id, 1, col_count))
                                        .for_each(|y| mask[x][y].push(gear_id))
                                });
                            gear_id += 1;
                            None
                        }
                        _ => None,
                    })
                    .collect::<Box<[_]>>(),
            )
        })
        .collect::<Box<[_]>>();
    let mut star_numbers: Vec<Vec<usize>> = vec![vec![]; gear_id];
    let digits = digits
        .iter()
        .map(|(row_id, row)| {
            row.iter().fold(
                (0, BTreeSet::<usize>::new(), 0),
                |(mut num, mut gears, last_col_id), (col_id, digit)| {
                    if col_id - last_col_id > 1 {
                        gears.iter().for_each(|star| star_numbers[*star].push(num));
                        num = 0;
                        gears = BTreeSet::new();
                    }
                    gears.extend(mask[*row_id][*col_id].clone());
                    num = num * 10 + *digit as usize;
                    (num, gears, *col_id)
                },
            )
        })
        .collect::<Box<[_]>>();
    digits.iter().for_each(|(num, gears, _)| {
        gears.iter().for_each(|star| star_numbers[*star].push(*num));
    });
    let result: usize = star_numbers
        .iter()
        .map(|gear| {
            if gear.len() == 2 {
                gear.iter().product()
            } else {
                0
            }
        })
        .sum();
    println!("{result:?}");
}
