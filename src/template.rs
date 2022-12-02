use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use aoc_2022::*;

struct Input {}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    }
}

impl Solve for Input {
    fn solve1(&self) -> anyhow::Result<usize> {
    }

    fn solve2(&self) -> anyhow::Result<usize> {
    }
}


fn main() {
    Input::solve_and_print_all();
}
