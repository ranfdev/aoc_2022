use aoc_2022::*;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Input(Vec<((isize, isize), (isize, isize))>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let res = s
            .lines()
            .map(|l| {
                let cs = re.captures(l).unwrap();
                let mut ns = cs
                    .iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().parse::<isize>().unwrap());
                (
                    (ns.next().unwrap(), ns.next().unwrap()),
                    (ns.next().unwrap(), ns.next().unwrap()),
                )
            })
            .collect();
        Ok(Input(res))
    }
}

impl Solve for Input {
    type Output = isize;
    fn solve1(&self) -> anyhow::Result<isize> {
        //const Y: isize = 10;
        const Y: isize = 2000000;

        let m = self
            .0
            .iter()
            .flat_map(|((s0, s1), (b0, b1))| {
                let md = (s0 - b0).abs() + (s1 - b1).abs();
                let dy_target = (s1 - Y).abs();
                let delta_x = (md - dy_target).max(0);
                if delta_x == 0 {
                    return None;
                }
                let dx = delta_x / delta_x;
                Some((0..(delta_x.abs() * 2 + 1)).map(move |i| (s0 - delta_x) + dx * i))
            })
            .flatten()
            .collect::<HashSet<_>>();

        Ok(m.len() as isize - 1)
    }

    fn solve2(&self) -> anyhow::Result<isize> {
        //const SEARCH_SPACE: isize = 20;
        const SEARCH_SPACE: isize = 4000000;

        let mut v = vec![];
        let mut m = (0..=SEARCH_SPACE).flat_map(|yy| {
            v.clear();
            v.extend(self.0.iter().flat_map(|((s0, s1), (b0, b1))| {
                let md = (s0 - b0).abs() + (s1 - b1).abs();
                let dy_target = (s1 - yy).abs();
                let delta_x = (md - dy_target).max(0);
                if delta_x > 0 {
                    Some((s0 - delta_x, s0 + delta_x))
                } else {
                    None
                }
            }));
            v.sort();
            let init = v[0];
            let (_, hole) = v
                .drain(1..)
                .fold_while((init, None), |(acc, _), interval| {
                    if acc.1 >= interval.0 && interval.1 >= acc.0 {
                        Continue(((acc.0.max(interval.0), acc.1.max(interval.1)), None))
                    } else if (acc.1 - interval.0).abs() == 2 {
                        Done((acc, Some(acc.1 + 1)))
                    } else if (interval.1 - acc.0).abs() == 2 {
                        Done((acc, Some(interval.1 + 1)))
                    } else {
                        Done((acc, None))
                    }
                })
                .into_inner();
            hole.zip(Some(yy))
        });
        let res = m.next().unwrap();
        dbg!(res);
        Ok(res.0 * 4000000 + res.1)
    }
}

fn main() {
    Input::solve_and_print_all();
}
