use std::{cmp::min, ops::Range};

use itertools::Itertools;

fn get_location(seed: usize, recipes: &Vec<Vec<(Range<usize>, isize)>>) -> usize {
    let mut current = seed;
    for recipe in recipes {
        for v in recipe.iter() {
            if (v.0).contains(&current) {
                current = (v.1 + current as isize) as usize;
                break;
            }
        }
    }
    current
}
fn get_range_location(seeds: Range<usize>, recipes: &Vec<Vec<(Range<usize>, isize)>>) -> usize {
    let mut current_ranges = vec![seeds]; // current range
    for recipe in recipes {
        let mut new = vec![];
        for range in current_ranges.clone().iter() {
            let start = &range.start;
            let mut current = start;
            let mut result = vec![];
            for item in recipe
                .iter()
                .skip_while(|item| current.clone() > item.0.end)
            {
                if item.0.start > current.clone() {
                    result.push(current.clone()..min(range.end.clone(), item.0.start.clone()));
                    current = &item.0.start;
                }
                if current >= &range.end {
                    break;
                }
                result.push(
                    ((current.clone() as isize + item.1) as usize)
                        ..(min(range.end, item.0.end) as isize + item.1) as usize,
                );
                current = &item.0.end;
                if current < &range.end {
                    break;
                }
            }
            if current.clone() < range.end {
                result.push(current.clone()..range.end);
            }
            new.append(&mut result)
        }
        current_ranges = new;
    }
    let mut min = usize::MAX;
    for range in current_ranges {
        if range.start < min {
            min = range.start;
        }
    }
    println!("{min}");
    min
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let input = input.split('\n').fold(Vec::new(), |mut acc, item| {
        if acc.is_empty() || item == "" {
            acc.push(Vec::new());
            if item == "" {
                return acc;
            }
        }
        if let Some(array) = acc.last_mut() {
            array.push(item);
        }
        acc
    });
    let seeds = input[0]
        .first()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let recipes = &input[1..]
        .iter()
        .map(|recipe| {
            let recipe = &recipe;
            let mut recipe = recipe[1..]
                .iter()
                .map(|line| {
                    let mut line = line.split_whitespace();
                    let destination_start = line.next().unwrap().parse::<usize>().unwrap();
                    let source_start = line.next().unwrap().parse::<usize>().unwrap();
                    let range_length = line.next().unwrap().parse::<usize>().unwrap();
                    (
                        (source_start..source_start + range_length),
                        destination_start as isize - source_start as isize,
                    )
                })
                .collect_vec();
            recipe.sort_by(|a, b| a.0.start.cmp(&b.0.start));
            recipe
        })
        .collect_vec();
    let part1 = seeds
        .clone()
        .map(|seed| get_location(seed, recipes))
        .min()
        .unwrap();
    let part2 = seeds
        .collect_vec()
        .chunks_exact(2)
        .map(|pair| get_range_location(pair[0]..(pair[0] + pair[1]), recipes))
        .min()
        .unwrap()
        .to_owned();
    (part1, part2)
}
