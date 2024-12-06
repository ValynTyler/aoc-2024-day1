use std::fmt::Display;

use aoc_2024_day1::puzzle_input::PuzzleInput;

pub struct PuzzleSolver {
    lefts: Vec<i32>,
    rights_freq: [i32; 100_000],
}

impl Display for PuzzleSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lefts: [")?;
        for i in 0..self.lefts.len() - 1 {
            write!(f, "{}, ", self.lefts[i])?;
        }
        writeln!(f, "{}]", self.lefts[self.lefts.len() - 1])?;

        writeln!(f, "rights frequency: {{")?;
        for i in 0..self.rights_freq.len() {
            if self.rights_freq[i] != 0 { writeln!(f, "  {}: {} times", i, self.rights_freq[i])?; }
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl From::<PuzzleInput> for PuzzleSolver {
    fn from(value: PuzzleInput) -> Self {
        let (lefts, rights) = value.split();
        let mut rights_freq = [0; 100_000];

        for i in 0..rights.len() {
            let index = rights[i] as usize;
            rights_freq[index] += 1;
        }

        Self { lefts, rights_freq }
    }
}

impl PuzzleSolver {
    pub fn similarity(&self) -> i32 {
        let mut sum = 0;
        for item in &self.lefts {
            sum += item * self.rights_freq[*item as usize];
        }

        sum
    }
}
