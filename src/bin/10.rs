use itertools::Itertools;

const TOP: [u8; 3] = [b'F', b'7', b'|'];
const RIGHT: [u8; 3] = [b'7', b'J', b'-'];
const BOTTOM: [u8; 3] = [b'|', b'J', b'L'];
const LEFT: [u8; 3] = [b'-', b'L', b'F'];

fn solve(
    current: (usize, usize),
    previous: (usize, usize),
    grid: &Vec<Vec<u8>>,
) -> Vec<(usize, usize)> {
    let next = match grid[current.0][current.1] {
        b'7' | b'J' | b'-' if previous.1 != current.1 - 1 => (current.0, current.1 - 1),
        b'-' | b'L' | b'F' if previous.1 != current.1 + 1 => (current.0, current.1 + 1),
        b'F' | b'7' | b'|' if previous.0 != current.0 + 1 => (current.0 + 1, current.1),
        b'|' | b'J' | b'L' if previous.0 != current.0 - 1 => (current.0 - 1, current.1),
        _ => unreachable!(),
    };
    if grid[next.0][next.1] == b'S' {
        vec![next, current]
    } else {
        let mut v = solve(next, current, &grid);
        v.push(current);
        v
    }
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let mut start: Option<(usize, usize)> = None;
    let mut grid = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c == b'S' {
                start = Some((row, col));
            }
        }
    }
    let start = start.unwrap();
    let next = if TOP.contains(&grid[start.0 + 1][start.1]) {
        (start.0 + 1, start.1)
    } else if BOTTOM.contains(&grid[start.0 - 1][start.1]) {
        (start.0 - 1, start.1)
    } else if RIGHT.contains(&grid[start.0][start.1 + 1]) {
        (start.0, start.1 + 1)
    } else {
        (start.0, start.1 - 1)
    };
    let result = solve(next, start, &grid);
    let top = TOP.contains(&grid[start.0 - 1][start.1]);
    let bottom = BOTTOM.contains(&grid[start.0 + 1][start.1]);
    let left = LEFT.contains(&grid[start.0][start.1 - 1]);
    let right = RIGHT.contains(&grid[start.0][start.1 + 1]);
    grid[start.0][start.1] = match true {
        _ if bottom && top => b'|',
        _ if bottom && right => b'F',
        _ if bottom && left => b'7',
        _ if top && right => b'L',
        _ if top && left => b'J',
        _ if left && right => b'-',
        _ => unreachable!(),
    };
    let mut part2 = 0;
    for row in 0..grid.len() {
        let mut should_include = false;
        for col in 0..grid[row].len() {
            if !result.contains(&(row, col)) {
                if should_include {
                    part2 += 1;
                    print!("•"); // visualization
                } else {
                    print!(" "); // visualization
                }
            } else {
                if BOTTOM.contains(&grid[row][col]) {
                    should_include = !should_include;
                }
                // visualization BEGIN
                let c = match grid[row][col] {
                    b'L' => '╰',
                    b'F' => '╭',
                    b'J' => '╯',
                    b'7' => '╮',
                    b'|' => '│',
                    b'-' => '─',
                    x => x as char,
                };
                print!("{}", c);
                // visualization END
            }
        }
        println!(); // visualization
    }
    (result.len() / 2, part2)
}
