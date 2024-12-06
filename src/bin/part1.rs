use std::{fs::File, io::BufReader};

use aoc_2024_day1::puzzle_input::PuzzleInput;

fn main() -> std::io::Result<()> {
    println!("solving part 1...");

    let input_string = include_str!("../../input/numbers.txt");
    let input = PuzzleInput::from(input_string);

    println!("{}", input);

    Ok(())
}
