use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();
    let mut mask: Box<[Box<[bool]>]> = vec![vec![false; col_count].into(); row_count].into();
    let result: u32 = input
        .lines()
        .enumerate()
        .map(|(row_id, row)| {
            (
                row_id,
                row.chars()
                    .enumerate()
                    .filter_map(|(col_id, cell)| match cell {
                        '.' => None,
                        digit @ '0'..='9' => Some((col_id, digit.to_digit(10).unwrap())),
                        _ => {
                            let clamp_add = |x, y, max| {
                                (x as isize + y as isize).clamp(0, max as isize - 1) as usize
                            };
                            (-1..=1).for_each(|x| {
                                (-1..=1).for_each(|y| {
                                    mask[clamp_add(row_id, x, row_count)]
                                        [clamp_add(col_id, y, col_count)] = true
                                })
                            });
                            None
                        }
                    })
                    .collect::<Box<[_]>>(),
            )
        })
        .collect::<Box<[_]>>()
        .iter()
        .map(|(row_id, row)| {
            row.iter().fold(
                (0, 0, false, 0),
                |(mut sum, mut num, mut is_part_num, last_col_id), (col_id, digit)| {
                    if col_id - last_col_id > 1 {
                        if is_part_num {
                            sum += num;
                        }
                        num = 0;
                        is_part_num = false;
                    }
                    is_part_num |= mask[*row_id][*col_id];
                    num = num * 10 + *digit;
                    (sum, num, is_part_num, *col_id)
                },
            )
        })
        .map(|(sum, num, is_part_num, _)| if is_part_num { sum + num } else { sum })
        .sum();

    println!("{result:?}");
}
