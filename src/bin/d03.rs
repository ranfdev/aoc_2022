use aoc_2022::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Input(Vec<String>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.lines().map(String::from).collect()))
    }
}

fn priority(c: char) -> usize {
    let n = match c {
        'a'..='z' => c as u32 - 'a' as u32,
        'A'..='Z' => 26 + c as u32 - 'A' as u32,
        _ => panic!("invalid char"),
    };
    n as usize + 1
}
impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let r = self
            .0
            .iter()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| {
                let (ha, hb) = (
                    a.chars().collect::<HashSet<_>>(),
                    b.chars().collect::<HashSet<_>>(),
                );
                *ha.intersection(&hb).into_iter().next().unwrap()
            })
            .map(priority)
            .sum();
        Ok(r)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let r = self
            .0
            .iter()
            .chunks(3)
            .into_iter()
            .map(|group| {
                let int = group
                    .map(|l| l.chars().collect::<HashSet<_>>())
                    .reduce(|a, b| a.intersection(&b).cloned().collect())
                    .unwrap();
                int.into_iter().next().unwrap()
            })
            .map(priority)
            .sum();
        Ok(r)
    }
}

fn main() {
    Input::solve_and_print_all();
}
