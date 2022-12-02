use aoc_2022::*;
use std::str::FromStr;

const ABC: &str = "ABC";
const XYZ: &str = "XYZ";
const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

struct Input(Vec<(char, char)>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.lines().map(|l| {
            let mut it = l.chars();
            let a = it.next().unwrap();
            it.next().unwrap();
            let b = it.next().unwrap();
            (a, b)
        });
        Ok(Input(r.collect()))
    }
}

fn beats(n: usize) -> Option<usize> {
    match n {
        0 => Some(2),
        1 => Some(0),
        2 => Some(1),
        _ => None,
    }
}

impl Solve for Input {
    fn solve1(&self) -> anyhow::Result<usize> {
        let r = self
            .0
            .iter()
            .map(|(a, b)| {
                let a = ABC.chars().position(|c| c == *a).unwrap();
                let b = XYZ.chars().position(|c| c == *b).unwrap();
                if a == b {
                    b + DRAW + 1
                } else if beats(b) == Some(a) {
                    b + WIN + 1
                } else {
                    b + LOSE + 1
                }
            })
            .sum();
        Ok(r)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let r = self
            .0
            .iter()
            .filter_map(|(a, b)| {
                let a = ABC.chars().position(|c| c == *a).unwrap();
                match b {
                    'X' => (0..3).find(|x| beats(a) == Some(*x)).map(|n| n + LOSE + 1),
                    'Y' => (0..3)
                        .find(|x| beats(a) != Some(*x) && beats(*x) != Some(a))
                        .map(|n| n + DRAW + 1),
                    'Z' => (0..3).find(|x| beats(*x) == Some(a)).map(|n| n + WIN + 1),
                    _ => None,
                }
            })
            .sum();
        Ok(r)
    }
}

fn main() {
    Input::solve_and_print_all();
}
