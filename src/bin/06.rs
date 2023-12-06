use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let input = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .collect_vec()
        })
        .collect_vec();
    let part1 = input[0]
        .iter()
        .zip(input[1].iter())
        .map(calculate_ways)
        .fold(1, |acc, x| acc * x);
    let time = input[0].concat();
    let record = input[1].concat();
    let part2 = calculate_ways((&time.as_str(), &record.as_str()));
    (part1, part2)
}

fn calculate_ways(time_record: (&&str, &&str)) -> usize {
    let (time, record) = time_record;
    let (time, record) = (
        time.parse::<usize>().unwrap(),
        record.parse::<usize>().unwrap(),
    );
    (0..time + 1)
        .filter_map(|t| {
            let distance = (time - t) * t;
            if distance > record {
                Some(distance)
            } else {
                None
            }
        })
        .count()
}
