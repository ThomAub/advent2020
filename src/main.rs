mod day01;
use anyhow::Result;
use day01::{part_one, part_two};
use std::fs;

pub fn read_input(input_path: &str) -> Result<Vec<String>> {
    let raw_input = fs::read_to_string(input_path)?;
    let input: Vec<String> = raw_input
        .trim_end()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    Ok(input)
}

fn main() -> Result<()> {
    let raw_input_one = read_input("inputs/day01/in.txt")?;
    part_one(raw_input_one);

    let raw_input_two = read_input("inputs/day01/in.txt")?;
    part_two(raw_input_two);
    Ok(())
}
