use std::io;
use std::io::prelude::*;
use std::str::FromStr;

pub trait Solve: FromStr {
    fn solve1(&self) -> anyhow::Result<usize>;
    fn solve2(&self) -> anyhow::Result<usize>;
    fn solve_and_print_all()
    where
        <Self as FromStr>::Err: std::fmt::Debug,
    {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let input = input.parse::<Self>().unwrap();

        println!("Part 1: {:#?}", Self::solve1(&input));
        println!("Part 2: {:#?}", Self::solve2(&input));
    }
}
