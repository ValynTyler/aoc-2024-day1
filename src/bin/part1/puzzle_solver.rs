use std::fmt::Display;

use aoc_2024_day1::{pair::Pair, puzzle_input::PuzzleInput};

pub struct PuzzleSolver(Vec<Pair>);

impl Display for PuzzleSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pair in &self.0 {
            writeln!(f, "({}, {})", pair.left, pair.right)?;
        }
        Ok(())
    }
}

impl From::<PuzzleInput> for PuzzleSolver {
    fn from(value: PuzzleInput) -> Self {
        let split = value.split();

        let mut lefts = split.0;
        let mut rights = split.1;

        lefts.sort();
        rights.sort();

        assert_eq!(lefts.len(), rights.len());
        let mut output = vec![];
        for i in 0..lefts.len() {
            output.push(Pair { left: lefts[i], right: rights[i] })
        }

        Self(output)
    }
}

impl PuzzleSolver {
    pub fn total_diff(&self) -> i32 {
        let mut sum = 0;
        for pair in &self.0 {
            sum += pair.diff()
        }

        sum
    }
}
