use hashbrown::HashMap;
use itertools::Itertools;

const CARD_COUNT: usize = 13;
const PART1_ORDER: [char; CARD_COUNT] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const PART2_ORDER: [char; CARD_COUNT] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn organize(cards: &str) -> String {
    let mut map = HashMap::new();
    cards.chars().for_each(|card| {
        let count = map.entry(card).or_insert(0u8);
        *count += 1;
    });
    let binding = cards
        .chars()
        .map(|card| {
            let position = PART1_ORDER.iter().position(|&chr| chr == card).unwrap();
            (position + 65) as u8 as char
        })
        .join("");
    let char_as_index_in_order = binding.as_str();
    let result = map.values().sorted().map(|v| v.to_string()).join("");
    let order = match result.as_str() {
        "5" => "0",
        "14" => "1",
        "23" => "2",
        "113" => "3",
        "122" => "4",
        "1112" => "5",
        "11111" => "6",
        _ => unreachable!(),
    };
    [order, char_as_index_in_order].concat()
}

fn organize_2(cards: &str) -> String {
    let mut map = HashMap::new();
    cards.chars().for_each(|card| {
        let count = map.entry(card).or_insert(0u8);
        *count += 1;
    });
    let entry = map.remove_entry(&'J');
    if let Some((_, count)) = entry {
        let max = map.values().max();
        if let Some(max) = max {
            let key = map
                .keys()
                .find(|key| map.get(*key).unwrap() == max)
                .unwrap();
            map.entry(*key).and_modify(|value| *value += count);
        } else {
            map.entry('A').or_insert(5);
        }
    }
    let binding = cards
        .chars()
        .map(|card| {
            let position = PART2_ORDER.iter().position(|&chr| chr == card).unwrap();
            (position + 65) as u8 as char
        })
        .join("");
    let char_as_index_in_order = binding.as_str();
    let result = map.values().sorted().map(|v| v.to_string()).join("");
    let order = match result.as_str() {
        "5" => "0",
        "14" => "1",
        "23" => "2",
        "113" => "3",
        "122" => "4",
        "1112" => "5",
        "11111" => "6",
        _ => unreachable!(),
    };
    [order, char_as_index_in_order].concat()
}
#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let input = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u32>().unwrap();
            ((organize(cards), bid), (organize_2(cards), bid))
        })
        .collect_vec();
    let mut input1 = input.iter().map(|(x, _)| x).collect_vec();
    let mut input2 = input.iter().map(|(_, x)| x).collect_vec();
    input1.sort_by(|l, r| r.0.cmp(&l.0));
    input2.sort_by(|l, r| r.0.cmp(&l.0));
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..input.len() {
        part1 += (input1[i].1 * (i + 1) as u32) as usize;
        part2 += (input2[i].1 * (i + 1) as u32) as usize;
    }
    (part1, part2)
}
