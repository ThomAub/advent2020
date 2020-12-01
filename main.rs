mod utils;

fn part_one() -> Result<(usize), Box<dyn std::error::Error>> {
    // Retrieve raw input
    let raw_input = utils::get_raw_input(1)?;
    // Part One
    let total_mass: usize = raw_input
        .trim_end()
        .split('\n')
        .map(|v| v.parse().unwrap())
        .map(|w: usize| w / 3 - 2)
        .sum();
    dbg!(total_mass);
    Ok(total_mass)
}

fn part_two() -> Result<(usize), Box<dyn std::error::Error>> {
    // Retrieve raw input
    let raw_input = utils::get_raw_input(1)?;
    // Part Twolet
    let total_mass: usize = raw_input
        .trim_end()
        .split('\n')
        .map(|v| v.parse().unwrap())
        .map(|w: usize| w / 3 - 2)
        .map(|z|)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one();
    part_two();
}
