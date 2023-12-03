const ALPHANUMERIC: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn part1(line: &str) -> usize {
    let digits = line
        .chars()
        .filter(|chr| chr.is_digit(10))
        .collect::<Vec<_>>();
    (digits.first().unwrap().to_digit(10).unwrap() * 10
        + digits.last().unwrap().to_digit(10).unwrap()) as usize
}
fn part2(line: &str) -> usize {
    let mut line = line.to_owned();
    for (alpha, numeric) in ALPHANUMERIC {
        line = line.replace(alpha, numeric);
    }
    part1(&line)
}

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let lines = input.lines();
    let part1: usize = lines.clone().map(part1).sum();
    let part2: usize = lines.clone().map(part2).sum();
    (part1, part2)
}
