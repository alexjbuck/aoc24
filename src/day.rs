use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum Part {
    Part1,
    Part2,
}

impl From<Part> for &str {
    fn from(value: Part) -> Self {
        match value {
            Part::Part1 => "part1",
            Part::Part2 => "part2",
        }
    }
}

impl std::str::FromStr for Part {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "1" | "part1" => Ok(Part::Part1),
            "2" | "part2" => Ok(Part::Part2),
            _ => Err(InputError),
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

#[derive(Debug, Clone, Copy)]
pub struct InputError;

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InputError {}

impl std::str::FromStr for Day {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "01" => Ok(Day::Day01),
            "2" | "02" => Ok(Day::Day02),
            "3" | "03" => Ok(Day::Day03),
            "4" | "04" => Ok(Day::Day04),
            "5" | "05" => Ok(Day::Day05),
            "6" | "06" => Ok(Day::Day06),
            "7" | "07" => Ok(Day::Day07),
            "8" | "08" => Ok(Day::Day08),
            "9" | "09" => Ok(Day::Day09),
            "10" => Ok(Day::Day10),
            "11" => Ok(Day::Day11),
            "12" => Ok(Day::Day12),
            "13" => Ok(Day::Day13),
            "14" => Ok(Day::Day14),
            "15" => Ok(Day::Day15),
            "16" => Ok(Day::Day16),
            "17" => Ok(Day::Day17),
            "18" => Ok(Day::Day18),
            "19" => Ok(Day::Day19),
            "20" => Ok(Day::Day20),
            "21" => Ok(Day::Day21),
            "22" => Ok(Day::Day22),
            "23" => Ok(Day::Day23),
            "24" => Ok(Day::Day24),
            "25" => Ok(Day::Day25),
            _ => Err(InputError),
        }
    }
}

impl From<Day> for &str {
    fn from(value: Day) -> Self {
        match value {
            Day::Day01 => "01",
            Day::Day02 => "02",
            Day::Day03 => "03",
            Day::Day04 => "04",
            Day::Day05 => "05",
            Day::Day06 => "06",
            Day::Day07 => "07",
            Day::Day08 => "08",
            Day::Day09 => "09",
            Day::Day10 => "10",
            Day::Day11 => "11",
            Day::Day12 => "12",
            Day::Day13 => "13",
            Day::Day14 => "14",
            Day::Day15 => "15",
            Day::Day16 => "16",
            Day::Day17 => "17",
            Day::Day18 => "18",
            Day::Day19 => "19",
            Day::Day20 => "20",
            Day::Day21 => "21",
            Day::Day22 => "22",
            Day::Day23 => "23",
            Day::Day24 => "24",
            Day::Day25 => "25",
        }
    }
}
