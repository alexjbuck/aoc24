# Advent of Code 2024 :crab: Rust Edition

Thank you for being interested in my Advent of Code 2024 development tool!

## Prerequisites

`rustup` installed with `cargo` and `rustc` and toolchain for your platform installed.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

## :rocket: Getting Started

```shell
$ cargo install aocr
$ aocr init aoc24 
$ cd aoc24
$ aocr watch
```

## :star: Quick overview of Advent of Code

Advent of Code or AoC is a 25 day programming challenge with a new problem coming out each day. Each problem has two parts, part 1 and part 2.

Generally each part takes a small text file as input and computes a number as output. You input the number into [Advent of Code](https://adventofcode.com/) in order to verify your solution as correct or incorrect. Each day you must pass part 1 before being shown the problem for part 2.

## :book: Day to Day Usage

Each day has its own library crate located at `day##` comprised of 2 files:
- `Cargo.toml`
- `src/lib.rs`

Each day's library crate contains two functions called `part1` and `part2` that takes as input a `&str` string slice and return a `usize` unsized integer. Implement your solution for parts 1 and 2 in these functions.

```rust
/// day01/src/lib.rs
pub fn part1(input: &str) -> usize {
    // TODO: Implement part 1 solution
    0
}

pub fn part2(input: &str) -> usize {
    // TODO: Implement part 2 solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
```
### Initialize your working folder

Use `aocr` to initialize a folder:

```shell
aocr init <folder>
```

This folder will be populated with a `cargo` workspace as well as a fresh `git` repository. You need to be in this workspace when executing `aocr` commands for them to register correctly.

### How to run `aocr` 

To start the interactive terminal user interface (tui), from within your initialized workspace:

```shell
aocr watch
```

To view help:

```shell
# Show help
aocr --help
```

To run a day and selected part without the terminal user interface:

```shell
# Run a day and selected parts
aocr <day> <part>
```

### Navigating the TUI and providing input data

When `aocr watch` runs, you can use the direction arrows or `h/j/k/l` keys (vim-bindings) to move the day/part selector left/up/down/right. 

Press the `i` key to set the input for the selected day.

```
┌Days───┐┌Input (Ctrl+S or Ctrl+Enter to save, Ctrl+V ┐
│01 1 2 ││// input goes here_                         │
│02 1 2 ││                                            │
│03 1 2 ││                                            │
│04 1 2 ││                                            │
│05 1 2 ││                                            │
│06 1 2 ││                                            │
│07 1 2 ││                                            │
│08 1 2 ││                                            │
│09 1 2 ││                                            │
│10 1 2 ││                                            │
│11 1 2 ││                                            │
│12 1 2 ││                                            │
│13 1 2 ││                                            │
│14 1 2 ││                                            │
│15 1 2 ││                                            │
│16 1 2 ││                                            │
│17 1 2 ││                                            │
│18 1 2 ││                                            │
│19 1 2 ││                                            │
│20 1 2 ││                                            │
└───────┘└────────────────────────────────────────────┘
```

Press `enter` or `w` (watch) on a selected day to start running `cargo check` on the library crate for the selected day:

```
┌Days───┐┌Cargo output day 1 part 1───────────────────┐
│01 1 2 ││Checking day 1 part 1...                    │
│02 1 2 ││warning: unused variable: `input`           │
│03 1 2 ││ --> day01/src/lib.rs:3:14                  │
│04 1 2 ││  |                                         │
│05 1 2 ││3 | pub fn part1(input: &str) -> usize {    │
│06 1 2 ││  |              ^^^^^ help: if this is inte│
│07 1 2 ││  |                                         │
│08 1 2 ││  = note: `#[warn(unused_variables)]` on by │
│09 1 2 ││                                            │
│10 1 2 ││warning: unused variable: `input`           │
│11 1 2 ││ --> day01/src/lib.rs:8:14                  │
│12 1 2 ││  |                                         │
│13 1 2 ││8 | pub fn part2(input: &str) -> usize {    │
│14 1 2 ││  |              ^^^^^ help: if this is inte│
│15 1 2 ││                                            │
│16 1 2 ││warning: `day01` (lib) generated 2 warnings │
│17 1 2 ││    Finished `dev` profile [unoptimized + de│
│18 1 2 ││                                            │
│19 1 2 ││                                            │
│20 1 2 ││                                            │
└───────┘└────────────────────────────────────────────┘
```

Press `t` on a selected day to run `cargo test` on the library crate for the selected day:

```
┌Days───┐┌Cargo output day 1 part 1───────────────────┐
│01 1 2 ││warning: unused variable: `input`           │
│02 1 2 ││ --> day01/src/lib.rs:3:14                  │
│03 1 2 ││  |                                         │
│04 1 2 ││3 | pub fn part1(input: &str) -> usize {    │
│05 1 2 ││  |              ^^^^^ help: if this is inte│
│06 1 2 ││  |                                         │
│07 1 2 ││  = note: `#[warn(unused_variables)]` on by │
│08 1 2 ││                                            │
│09 1 2 ││warning: unused variable: `input`           │
│10 1 2 ││ --> day01/src/lib.rs:8:14                  │
│11 1 2 ││  |                                         │
│12 1 2 ││8 | pub fn part2(input: &str) -> usize {    │
│13 1 2 ││  |              ^^^^^ help: if this is inte│
│14 1 2 ││                                            │
│15 1 2 ││warning: `day01` (lib) generated 2 warnings │
│16 1 2 ││warning: `day01` (lib test) generated 2 warn│
│17 1 2 ││    Finished `test` profile [unoptimized + d│
│18 1 2 ││     Running unittests src/lib.rs (target/de│
│19 1 2 ││error: test failed, to rerun pass `-p day01 │
│20 1 2 ││                                            │
└───────┘└────────────────────────────────────────────┘
```

Press `r` on the selected day to run the selected part function and generate a result:

```
┌Days───┐┌Cargo output day 1 part 1───────────────────┐
│01 1 2 ││Result: 0                                   │
│02 1 2 ││                                            │
│03 1 2 ││                                            │
│04 1 2 ││                                            │
│05 1 2 ││                                            │
│06 1 2 ││                                            │
│07 1 2 ││                                            │
│08 1 2 ││                                            │
│09 1 2 ││                                            │
│10 1 2 ││                                            │
│11 1 2 ││                                            │
│12 1 2 ││                                            │
│13 1 2 ││                                            │
│14 1 2 ││                                            │
│15 1 2 ││                                            │
│16 1 2 ││                                            │
│17 1 2 ││                                            │
│18 1 2 ││                                            │
│19 1 2 ││                                            │
│20 1 2 ││                                            │
└───────┘└────────────────────────────────────────────┘
```

If you need to modify the input file for any reason, they are stored at `inputs/day##_part#.txt` in your repository. Feel free to edit/delete this file. If you delete it, the next time you attempt to run that day & part, `aocr` will prompt you for input again.

The input text will be made available to you via the [AoC](http://adventofcode.com/) website.
