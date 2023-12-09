use itertools::Itertools;

#[aoc::main(09)]
fn main(input: &str) -> (i32, i32) {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    input
        .iter()
        .map(|sequence| {
            let mut sequence = sequence.clone();
            solve(&mut sequence);
            (sequence[sequence.len() - 1].clone(), sequence[0].clone())
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

fn solve(sequence: &mut Vec<i32>) {
    let mut diff_seq = vec![];
    for index in 0..sequence.len() {
        if let Some(&next_rec) = sequence.get(index + 1) {
            diff_seq.push(next_rec - sequence[index]);
        } else {
            if !diff_seq.iter().all_equal() {
                solve(&mut diff_seq);
            }
            sequence.push(*sequence.last().unwrap() + diff_seq.last().unwrap());
            sequence.push(*sequence.first().unwrap() - diff_seq.first().unwrap());
            sequence.rotate_right(1);
        }
    }
}
