use aoc_2022::*;
use std::collections::HashSet;
use std::str::FromStr;

struct Input(String);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.to_string()))
    }
}

fn find_sol(s: &str, n: usize) -> Option<usize> {
    s.chars()
        .enumerate()
        .map(|(i, _)| (i, s.chars().skip(i + 1).take(n)))
        .find_map(|(i, x)| {
            let m = x.collect::<HashSet<_>>();
            (m.len() == n).then_some(i + n + 1)
        })
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        Ok(find_sol(&self.0, 4).unwrap())
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        Ok(find_sol(&self.0, 14).unwrap())
    }
}

fn main() {
    Input::solve_and_print_all();
}
