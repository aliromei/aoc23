#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let (mut part1, mut part2) = (0,0);
    input.lines().for_each(|line| {
        let (game_number, data) = line.trim_start_matches("Game ").split_once(':').unwrap();
        let mut max = (0u32,0u32,0u32);
        data.trim().split([';', ',']).for_each(|entry| {
            let (count, color) = entry.trim().split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match color.as_bytes()[0] {
                b'r' => {
                    if count > max.0 {
                        max.0 = count
                    }
                }
                b'g' => {
                    if count > max.1 {
                        max.1 = count
                    }
                }
                b'b' => {
                    if count > max.2 {
                        max.2 = count
                    }
                }
                _ => unreachable!(),
            }
        });
        if max.0 <= 12 && max.1 <= 13 && max.2 <= 14 {
            part1 = part1 + game_number.parse::<usize>().unwrap();
        }
        part2 = part2 + (max.0*max.1*max.2) as usize;
    });
    (part1, part2)
}
