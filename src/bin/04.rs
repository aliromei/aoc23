use hashbrown::HashMap;
use itertools::Itertools;

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let mut map = HashMap::new();
    let result = input
        .lines()
        .map(|line| {
            let (game_number, game) = line.split_once(':').unwrap();
            let game_number: u8 = game_number
                .chars()
                .skip(5)
                .collect::<String>()
                .trim()
                .parse()
                .unwrap();
            let splitted_line = game.trim().split_once("|").unwrap();
            let winning_numbers = splitted_line
                .0
                .split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect_vec();
            let my_numbers = splitted_line
                .1
                .split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect_vec();
            let winners_length = my_numbers
                .iter()
                .filter(|number| winning_numbers.contains(number))
                .collect_vec()
                .len();
            let current_count = map.entry(game_number).or_insert(1).clone();
            for game_number in (game_number + 1u8)..(game_number + 1 + winners_length as u8) {
                let entry = map.entry(game_number).or_insert(1);
                *entry += current_count;
            }
            if winners_length > 0 {
                2usize.pow((winners_length - 1) as u32)
            } else {
                0
            }
        })
        .collect_vec();
    (result.iter().sum(), map.values().sum())
}
