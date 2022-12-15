use aoc_2022::*;
use itertools::Itertools;
use std::collections::HashMap;

use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Block {
    Sand,
    Rock,
}

#[derive(Clone, Debug)]
struct Input {
    grid: HashMap<(isize, isize), Block>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = s
            .lines()
            .flat_map(|l| {
                let points = l
                    .split("->")
                    .map(|line| {
                        line.split(',')
                            .map(|coord| coord.trim().parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect::<Vec<(isize, isize)>>();
                points
                    .windows(2)
                    .into_iter()
                    .flat_map(|input| {
                        let (p0, p1) = (input[0], input[1]);
                        let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
                        let (dx, dy) = (dx.clamp(-1, 1), dy.clamp(-1, 1));
                        let mut pos = p0;
                        std::iter::from_fn(move || -> Option<(isize, isize)> {
                            if pos != p1 {
                                pos = (pos.0 + dx, pos.1 + dy);
                                Some(pos)
                            } else {
                                None
                            }
                        })
                        .chain(std::iter::once(p1))
                        .chain(std::iter::once(p0))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<(isize, isize)>>();
        let mut hm = HashMap::new();
        for b in blocks {
            hm.insert(b, Block::Rock);
        }
        Ok(Input { grid: hm })
    }
}

impl Input {
    // die_height prevents infinite recursion. If the y coordinate is higher than die_height,
    // recursion stops
    fn deposit(
        &mut self,
        grain: (isize, isize),
        die_height: isize,
        floor: Option<isize>,
    ) -> Option<(isize, isize)> {
        let below = self.grid.get(&(grain.0, grain.1 + 1));
        match below {
            Some(_) => {
                let left = (grain.0 - 1, grain.1 + 1);
                let right = (grain.0 + 1, grain.1 + 1);
                match (self.grid.get(&left), self.grid.get(&right)) {
                    (None, _) => self.deposit(left, die_height, floor),
                    (Some(_), None) => self.deposit(right, die_height, floor),
                    (Some(_), Some(_)) => {
                        self.grid.insert(grain, Block::Sand);
                        Some(grain)
                    }
                }
            }
            None if grain.1 > die_height => None,
            None if Some(grain.1 + 1) == floor => {
                self.grid.insert(grain, Block::Sand);
                Some(grain)
            }
            None => self.deposit((grain.0, grain.1 + 1), die_height, floor),
        }
    }
    fn calc_bounds(&self) -> ((isize, isize), (isize, isize)) {
        let minx = self.grid.keys().min_by_key(|x| x.0).unwrap().0;
        let maxx = self.grid.keys().max_by_key(|x| x.0).unwrap().0;
        let miny = self.grid.keys().min_by_key(|x| x.1).unwrap().1;
        let maxy = self.grid.keys().max_by_key(|x| x.1).unwrap().1;
        ((minx, maxx), (miny, maxy))
    }
    fn display(&self) {
        let ((minx, maxx), (miny, maxy)) = self.calc_bounds();
        for y in miny..=maxy {
            for x in minx..=maxx {
                match self.grid.get(&(x, y)) {
                    Some(Block::Sand) => print!("o"),
                    Some(Block::Rock) => print!("#"),
                    None => print!(" "),
                }
            }
            println!();
        }
    }
}

impl Solve for Input {
    type Output = isize;
    fn solve1(&self) -> anyhow::Result<isize> {
        self.display();
        let sand = (0..).map(|_| (500, 0));
        let mut input = self.clone();
        let (_, (_, maxy)) = input.calc_bounds();
        let c = sand
            .map(|grain| input.deposit(grain, maxy, None))
            .take_while(|x| x.is_some())
            .count();
        input.display();
        Ok(c as isize)
    }

    fn solve2(&self) -> anyhow::Result<isize> {
        self.display();
        let sand = (0..).map(|_| (500, 0));
        let mut input = self.clone();
        let (_, (_, maxy)) = self.calc_bounds();

        let c = sand
            .map(|grain| input.deposit(grain, isize::MAX, Some(maxy + 2)))
            .take_while(|x| *x != Some((500, 0)))
            .count();
        input.display();
        Ok(c as isize + 1)
    }
}

fn main() {
    Input::solve_and_print_all();
}
