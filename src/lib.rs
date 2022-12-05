use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt::Debug;

pub trait Solve: FromStr {
    type Output: Debug;
    fn solve1(&self) -> anyhow::Result<Self::Output>;
    fn solve2(&self) -> anyhow::Result<Self::Output>;
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
