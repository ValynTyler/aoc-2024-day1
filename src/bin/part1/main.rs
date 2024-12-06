use aoc_2024_day1::puzzle_input::PuzzleInput;
use puzzle_solver::PuzzleSolver;

mod puzzle_solver;

fn main() {
    println!("solving part 1...");

    let input_string = include_str!("../../../input/example.txt");

    let input = PuzzleInput::from(input_string);
    println!("{}", input);

    let solver = PuzzleSolver::from(input);
    println!("{}", solver);
}
