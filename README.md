# Advent of Code 20243

Solutions for Advent of Code 2023<br>
Also using codebase of [Axel Lindeberg](https://github.com/AxlLind)'s [AdventOfCode2023](https://github.com/AxlLind/AdventOfCode2023)

## Usage

```shell
# run specific day in XX format
cargo run --release --bin XX
# run all days
cargo run --release
```

### Additional conventions
install `aoc` command via:
```shell
cargo install aoc-ctl
```
get each day's puzzle and inputs
```shell
aoc download \
    --input-file inputs/XX.in \
    --puzzle-file puzzles/XX.md \
    --day X -o
```
