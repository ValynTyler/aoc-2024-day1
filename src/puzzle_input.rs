use std::fmt::Display;

pub struct Pair {
    left: i32,
    right: i32,
}

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
