use aoc_2022::*;
use itertools::Itertools;
use std::ops::Range;
use std::str::FromStr;

struct Input(Vec<((usize, usize), (usize, usize))>);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
        let v = s
            .lines()
            .flat_map(|l| {
                r.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1) // the first match is the full str
                    .flat_map(|n| n.unwrap().as_str().parse::<usize>())
                    .tuples() 
                    .collect_tuple()
            })
            .collect::<Vec<_>>();

        Ok(Input(v))
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let n = self
            .0
            .iter()
            .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
            .count();
        Ok(n)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let n = self
            .0
            .iter()
            .filter(|(a, b)| b.1 >= a.0 && a.1 >= b.0)
            .count();
        Ok(n)
    }

}

fn main() {
    Input::solve_and_print_all();
}
