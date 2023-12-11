use std::cmp::{max, min};

use hashbrown::HashSet;
use itertools::Itertools;

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    for index in 0..grid[0].len() {
        empty_cols.insert_unique_unchecked(index);
    }
    for row in 0..grid.len() {
        if grid[row].iter().all_equal() {
            empty_rows.insert_unique_unchecked(row);
        }
        for col in 0..grid[0].len() {
            if grid[row][col] == b'#' {
                empty_cols.remove(&col);
            }
        }
    }
    let mut locations = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'#' {
                locations.insert_unique_unchecked((row, col));
            }
        }
    }
    let locations = locations.iter().sorted().collect_vec();
    let mut raw_distance = 0;
    let mut empty_distance = 0;
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let (&a, &b) = (locations[i], locations[j]);
            let (row_min, row_max) = (min(a.0, b.0), max(a.0, b.0));
            let (col_min, col_max) = (min(a.1, b.1), max(a.1, b.1));
            let empty_row_distance = empty_rows
                .iter()
                .filter(|&&x| x > row_min && x < row_max)
                .count();
            let empty_col_distance = empty_cols
                .iter()
                .filter(|&&x| x > col_min && x < col_max)
                .count();
            raw_distance += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            empty_distance += empty_col_distance + empty_row_distance;
        }
    }
    (
        raw_distance + empty_distance,
        raw_distance + empty_distance * 999999,
    )
}
