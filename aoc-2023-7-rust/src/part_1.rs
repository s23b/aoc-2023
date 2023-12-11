use phf::{phf_map, Map};
use std::{fs::read_to_string, time::Instant};

static CARDS: Map<char, usize> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'J' => 9,
    'T' => 8,
    '9' => 7,
    '8' => 6,
    '7' => 5,
    '6' => 4,
    '5' => 3,
    '4' => 2,
    '3' => 1,
    '2' => 0,
};

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let start = Instant::now();
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let hand = hand.chars().map(|char| CARDS[&char]);
            let mut cards = vec![0; 13];
            hand.clone().for_each(|card| cards[card] += 1);
            cards.sort_unstable_by(|card1, card2| card2.cmp(card1));
            let strength = hand.fold(0, |s, c| (s << 4) + c)
                + match cards[0..2] {
                    [5, _] => 6 << 20,
                    [4, _] => 5 << 20,
                    [3, 2] => 4 << 20,
                    [3, 1] => 3 << 20,
                    [2, 2] => 2 << 20,
                    [2, 1] => 1 << 20,
                    _ => 0,
                };
            (strength, bid)
        })
        .collect::<Box<[_]>>();
    hands.sort_unstable_by(|card1, card2| card1.0.cmp(&card2.0));
    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(count, (_, bid))| (count + 1) * bid)
        .sum();
    println!("{result:?} obtained in {:?}", Instant::now() - start);
}
