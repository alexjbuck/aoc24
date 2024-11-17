# Advent of Code 2024 :crab: Rust Edition

Thank you for being interested in my Advent of Code 2024 development tool!

## Prerequisites

`rustup` installed with `cargo` and `rustc` and toolchain for your platform installed.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

### Optional 

Installing `cargo-watch` adds a file-system watcher component to `cargo` so it will recompile your application whenever you update any tracked files.

```shell
cargo install cargo-watch --locked
```

To use `cargo watch` just replace any occurence of `cargo run` with `cargo watch -x run`.

## Getting Started

To get started, fork and clone the repository.

If you have the GitHub cli `gh` you can run: 

```shell
gh repo fork --clone alexjbuck/aoc24
```

Or else you can just `git clone` and change the remote url to your git provider of choice:

```shell
git clone https://github.com/alexjbuck/aoc24.git
```

## Quick overview of Advent of Code

Advent of Code or AoC is a 25 day programming challenge with a new problem coming out each day. Each problem has two parts, part 1 and part 2.

Generally each part takes a small text file as input and computes a number as output. You input the number into [Advent of Code](https://adventofcode.com/) in order to verify your solution as correct or incorrect. Each day you must pass part 1 before being shown the problem for part 2.

## Day to Day Usage

Each day has its own module located at `src/day##` comprised of 3 files:
- `mod.rs`
- `part1.rs`
- `part2.rs`

You do not need to modify `mod.rs`, you will be doing all of your work in either `part1.rs` and `part2.rs`.

Each submodule (`part1.rs` and `part2.rs`) contains a function called `process` that takes as input a `&str` string slice. Implement your solution inside the `process` function.

```rust
/// day01/part1.rs
pub fn process(_input: &str) -> usize {
    todo!() // Replace me with your implementation
}
```

If you're curious, the `mod.rs` for each day is just some glue code that conditions the inputs and runs `part1::process` and `part2::process`. This is the public interface exposed to the main application, defined in `src/main.rs`.

```rust
pub mod part1;
pub mod part2;

/// Run part 1 and part 2 for this module. If either input is the None variant, skip running it.
pub fn run<T>(input1: Option<T>, input2: Option<T>)
where
    T: AsRef<str>,
{
    if let Some(input) = input1 {
        println!("Part 1: {}", part1::process(input.as_ref()));
    }
    if let Some(input) = input2 {
        println!("Part 2: {}", part2::process(input.as_ref()));
    }
}
```

### How to run `aoc24` 

With the repository cloned onto your computer, from within the repository folder, you can use `cargo run -- <command>`.

To view help:

```shell
# Show help
cargo run -- --help
```

To run a day and selected parts:

```shell
# Run a day and selected parts
cargo run -- -d <day> [--part1] [--part2]
```

#### Using cargo watch

To watch a day and recompile it while working:

```shell
# Run a day and recompile/run it when any file changes
cargo watch -x run -- -d <day> [--part1] [--part2]
```

Often times its quicker to not actually _run_ your application when you change a file, you may wish to just check it with `cargo`, to do this you could run instead:

```shell
# Run `cargo check` every time a file changes.
cargo watch
```

Then, when `cargo check` is not returning any errors, you can just run the day/part one time with:

```shell
cargo run -- -d <day> [--part1] [--part2]
```

### Providing inputs

When the program runs, if it doesn't not already have a non-empty input file it will prompt you for input by opening an editor window in the terminal. Copy the problem input text into the file then save and close it. `aoc24` will remember the input for you going foward.

If you need to modify the input file for any reason, they are stored at `inputs/Day##/Part#.txt` in your repository. Feel free to edit/delete this file. If you delete it, the next time you attempt to run that day & part, `aoc24` will prompt you for input again.

The input text will be made available to you via the [AoC](http://adventofcode.com/) website.

### Panics!

If you run a day or part before you have made any changes to the code, the `aoc24` application will `panic!` and crash with a statement of `not yet implemented`. This is because the `process` function in each `part1.rs` and `part2.rs` submodule is initialized with a `todo!()` macro, which allows the application to compile, but will cause a `panic!` if encountered at runtime.

You will need to remove this `todo!()` macro when you begin to edit the 
