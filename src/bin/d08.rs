use aoc_2022::*;
use itertools::iproduct;
use std::iter::repeat;
use std::str::FromStr;

struct Input(Vec<Vec<usize>>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .lines()
            .map(|l| {
                l.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|d| d as usize)
                    .collect()
            })
            .collect::<Vec<_>>();
        Ok(Input(r))
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let (rows, cols) = (self.0.len(), self.0[0].len());
        let r = iproduct!(1..rows - 1, 1..cols - 1)
            .filter_map(|(i, j)| {
                let item = self.0[i][j];
                let to_check: [&mut dyn Iterator<Item = _>; 4] = [
                    &mut (0..i).zip(repeat(j)),
                    &mut (i + 1..rows).zip(repeat(j)),
                    &mut repeat(i).zip(0..j),
                    &mut repeat(i).zip(j + 1..cols),
                ];
                let valid = to_check
                    .into_iter()
                    .any(move |it| it.map(|(ii, jj)| self.0[ii][jj] < item).all(|x| x));
                valid.then(|| item)
            })
            .count();
        Ok(r + rows * 2 + cols * 2 - 4)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let (rows, cols) = (self.0.len(), self.0[0].len());

        let r: Option<usize> = iproduct!(1..rows - 1, 1..cols - 1)
            .map(|(i, j)| {
                let item = self.0[i][j];
                let to_check: [&mut dyn Iterator<Item = _>; 4] = [
                    &mut (1..i).rev().zip(repeat(j)),
                    &mut (i + 1..rows - 1).zip(repeat(j)),
                    &mut repeat(i).zip((1..j).rev()),
                    &mut repeat(i).zip(j + 1..cols - 1),
                ];
                to_check
                    .into_iter()
                    .map(|it| it.take_while(|(ii, jj)| self.0[*ii][*jj] < item).count() + 1)
                    .product::<usize>()
            })
            .max();
        Ok(r.unwrap())
    }
}

fn main() {
    Input::solve_and_print_all();
}
