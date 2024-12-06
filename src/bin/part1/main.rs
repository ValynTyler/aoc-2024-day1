use aoc_2024_day1::{pair::Pair, puzzle_input::PuzzleInput};

fn main() {
    println!("solving part 1...");

    let input_string = include_str!("../../../input/example.txt");

    let input = PuzzleInput::from(input_string);
    println!("{}", input);

    let split = input.split();
    println!("{:?}", split);

    let mut lefts = split.0;
    let mut rights = split.1;
    lefts.sort();
    rights.sort();
    println!("{:?} ___ {:?}", lefts, rights);
}
