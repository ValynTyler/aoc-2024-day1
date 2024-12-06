use std::fmt::Display;

use crate::pair::Pair;

pub struct PuzzleInput(Vec<Pair>);

impl Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pair in &self.0 {
            writeln!(f, "({}, {})", pair.left, pair.right)?;
        }
        Ok(())
    }
}

impl From::<&str> for PuzzleInput {
    fn from(value: &str) -> Self {
        PuzzleInput(value
            .trim()
            .lines()
            .map(|line| {
                let split = line.split_whitespace();
                let tokens: Result<Vec<_>, _> = split
                    .map(|token| token.parse::<i32>())
                    .collect();

                // TODO
                let tokens = tokens.unwrap();
                assert_eq!(tokens.len(), 2);

                Pair {
                    left: tokens[0],
                    right: tokens[1],
                }
            })
            .collect()
        )
    }
}

impl PuzzleInput {
    pub fn split(self) -> (Vec<i32>, Vec<i32>) {
        let mut stuff = self;
        let mut output = (vec![], vec![]);

        for item in stuff.0.drain(..) {
            output.0.push(item.left);
            output.1.push(item.right);
        }
            
        output
    }
}
