use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use aoc24::day::{Day, Part};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Advent of Code 2024 Task Runner")]
struct Cli {
    #[arg(short)]
    pub day: Day,
}

fn main() {
    let args = Cli::parse();
    let input1 = prepare_input(&args.day, Part::Part1);
    let input2 = prepare_input(&args.day, Part::Part2);
    match &args.day {
        Day::Day01 => aoc24::day01::run(input1, input2),
        Day::Day02 => aoc24::day02::run(input1, input2),
        Day::Day03 => aoc24::day03::run(input1, input2),
        Day::Day04 => aoc24::day04::run(input1, input2),
        Day::Day05 => aoc24::day05::run(input1, input2),
        Day::Day06 => aoc24::day06::run(input1, input2),
        Day::Day07 => aoc24::day07::run(input1, input2),
        Day::Day08 => aoc24::day08::run(input1, input2),
        Day::Day09 => aoc24::day09::run(input1, input2),
        Day::Day10 => aoc24::day10::run(input1, input2),
        Day::Day11 => aoc24::day11::run(input1, input2),
        Day::Day12 => aoc24::day12::run(input1, input2),
        Day::Day13 => aoc24::day13::run(input1, input2),
        Day::Day14 => aoc24::day14::run(input1, input2),
        Day::Day15 => aoc24::day15::run(input1, input2),
        Day::Day16 => aoc24::day16::run(input1, input2),
        Day::Day17 => aoc24::day17::run(input1, input2),
        Day::Day18 => aoc24::day18::run(input1, input2),
        Day::Day19 => aoc24::day19::run(input1, input2),
        Day::Day20 => aoc24::day20::run(input1, input2),
        Day::Day21 => aoc24::day21::run(input1, input2),
        Day::Day22 => aoc24::day22::run(input1, input2),
        Day::Day23 => aoc24::day23::run(input1, input2),
        Day::Day24 => aoc24::day24::run(input1, input2),
        Day::Day25 => aoc24::day25::run(input1, input2),
    };
}

fn prepare_input(day: &Day, part: Part) -> Option<String> {
    let part_filename = format!("inputs/{:?}_{:?}.txt", &day, &part);
    if !PathBuf::from(&part_filename).is_file() {
        return prompt_for_input(day, part);
    }
    let input = std::fs::read_to_string(part_filename).expect("Failed to read part file");
    if input.is_empty() {
        return prompt_for_input(day, part);
    }
    Some(input)
}

fn prompt_for_input(day: &Day, part: Part) -> Option<String> {
    println!(
        "No input file found for {:?} {:?}. Please enter the input once the editor opens. Press enter to continue...",
        day, part
    );
    let _ = io::stdin().read(&mut [0u8]).unwrap();
    let result = edit::edit("");

    match result {
        Ok(input) => Some(input),
        Err(e) if e.kind() == ErrorKind::InvalidData => {
            println!("None UTF-8 input, please provide input manually to continue.");
            None
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("No editor found, please provide input manually to continue.");
            None
        }
        _ => {
            println!("Failed to process input, please provide input manually to continue.");
            None
        }
    }
}
