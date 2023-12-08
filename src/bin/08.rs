use std::iter::{repeat, Repeat};

use hashbrown::HashMap;
use itertools::Itertools;

fn solve(
    instructions: &Vec<char>,
    map: &HashMap<&str, (&str, &str)>,
    starting_node: &str,
    p1: bool,
) -> usize {
    let mut counter = 0;
    let mut current = map.get(starting_node).unwrap();
            for instructions in repeat(instructions) {
    for instruction in instructions {
            counter += 1;
            let next = match instruction {
                'L' => current.0,
                'R' => current.1,
                _ => unreachable!(),
            };
            if if p1 {next == "ZZZ"} else {next.chars().last().unwrap() == 'Z'} {
                return counter;
            }
            current = map.get(next).unwrap();
        }
    }
    unreachable!()
}

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let (instructions, node_map) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    let mut part2_starting_points = vec![];
    node_map.trim().lines().for_each(|line| {
        let (node, next) = line.split_once(" = ").unwrap();
        let next = next
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();
        map.insert(node, next);
        if node.chars().last().unwrap() == 'A' {
            part2_starting_points.push(node);
        }
    });
    let instructions = instructions.chars().collect_vec();
    let part1 = solve(&instructions, &map, "AAA", true);
    let part2 = part2_starting_points.iter().map(|starting_point| solve(&instructions, &map, &starting_point, false)).fold(1, |acc, result| {
        lcm(acc, result)
    });
    (part1, part2)
}

fn lcm(a: usize, b: usize) -> usize {
    a*b/gcd(a,b)
}
fn gcd(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
