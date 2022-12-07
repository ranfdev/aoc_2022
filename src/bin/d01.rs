use aoc_2022::*;
use std::str::FromStr;

struct Input(Vec<usize>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r: Vec<usize> = s
            .split("\n\n")
            .map(|group| group.lines().filter_map(|l| l.parse::<usize>().ok()).sum())
            .collect();
        r.sort();
        Ok(Input(r))
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        Ok(*self.0.iter().rev().next().unwrap())
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        Ok(self.0.iter().rev().take(3).sum())
    }
}

fn main() {
    Input::solve_and_print_all();
}
