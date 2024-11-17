pub mod part1;
pub mod part2;

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
