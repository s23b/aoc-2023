fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let result: u32 = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();
    println!("{result:?}");
}
