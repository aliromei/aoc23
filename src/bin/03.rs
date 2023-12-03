use hashbrown::HashSet;
use itertools::Itertools;

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut part2 = 0;
    let mut tmp_num = String::from("");
    let mut is_pn = false;
    for i in 0..puzzle.len() {
        let line = &puzzle[i];
        for j in 0..line.len() {
            if puzzle[i][j] == '*' {
                if let Some(gear_ratio) = gear_ratio(&puzzle, (i as i32, j as i32)) {
                    part2 = part2 + gear_ratio
                }
            }
            let chr = line[j];
            if chr.is_digit(10) {
                tmp_num.push(chr);
                if sign_adjacent(&puzzle, (i, j)) {
                    is_pn = true
                }
            } else if !tmp_num.is_empty() {
                if is_pn {
                    sum = sum + tmp_num.parse::<i32>().unwrap();
                }
                tmp_num = "".to_string();
                is_pn = false
            }
        }
    }
    (sum as usize, part2 as usize)
}

fn gear_ratio(puzzle: &Vec<Vec<char>>, (x, y): (i32, i32)) -> Option<u32> {
    let result: Vec<u32> = [x - 1, x, x + 1]
        .iter()
        .cartesian_product([y - 1, y, y + 1])
        .filter_map(|(x, y)| {
            if *x < 0 || y < 0 {
                return None;
            } else if let Some(val) = puzzle
                .get(*x as usize)
                .and_then(|line| line.get(y as usize))
            {
                if val.is_digit(10) {
                    let parsed = find_whole_number(puzzle, (*x, y), None).parse::<u32>();
                    return Some(parsed.unwrap());
                }
            }
            None
        })
        .collect();
    let result: HashSet<u32> = result.iter().cloned().collect();
    if result.len() == 2 {
        let mut result = result.iter();
        return Some(result.next().unwrap() * result.next().unwrap());
    }
    None
}

fn find_whole_number(puzzle: &Vec<Vec<char>>, (x, y): (i32, i32), go_left: Option<bool>) -> String {
    let line = puzzle.get(x as usize);
    let left = line.and_then(|line| line.get((y - 1) as usize));
    let current = line.and_then(|line| line.get(y as usize));
    let right = line.and_then(|line| line.get((y + 1) as usize));
    let mut rtn = String::new();
    match go_left {
        Some(go_left) => match go_left {
            true => {
                if left.is_some() && left.unwrap().is_digit(10) {
                    rtn.push_str(find_whole_number(puzzle, (x, y - 1), Some(true)).as_str());
                    rtn.push(*left.unwrap());
                }
            }
            false => {
                if right.is_some() && right.unwrap().is_digit(10) {
                    rtn.push(*right.unwrap());
                    rtn.push_str(find_whole_number(puzzle, (x, y + 1), Some(false)).as_str())
                }
            }
        },
        None => {
            if left.is_some() && left.unwrap().is_digit(10) {
                rtn.push_str(find_whole_number(puzzle, (x, y - 1), Some(true)).as_str());
                rtn.push(*left.unwrap());
            }
            rtn.push(*current.unwrap());
            if right.is_some() && right.unwrap().is_digit(10) {
                rtn.push(*right.unwrap());
                rtn.push_str(find_whole_number(puzzle, (x, y + 1), Some(false)).as_str());
            }
        }
    };
    rtn
}

fn sign_adjacent(puzzle: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    let mut found = false;
    let x = x as i32;
    let y = y as i32;
    for (i, j) in [x - 1, x, x + 1]
        .iter()
        .cartesian_product([y - 1, y, y + 1])
    {
        if *i < 0 || j < 0 {
            continue;
        }
        if let Some(val) = puzzle
            .get(*i as usize)
            .and_then(|line| line.get(j as usize))
        {
            if !val.is_digit(10) && val != &'.' {
                found = true;
                break;
            }
        }
    }
    return found;
}
