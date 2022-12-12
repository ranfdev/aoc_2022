use aoc_2022::*;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

struct Input {
    start: (usize, usize),
    end: (usize, usize),
    m: Vec<Vec<usize>>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let m = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|(j, c)| {
                        if *c == 'S' {
                            start = (i, *j);
                        } else if *c == 'E' {
                            end = (i, *j);
                        }
                    })
                    .map(|(_, c)| {
                        if c == 'S' {
                            'a' as usize
                        } else if c == 'E' {
                            'z' as usize
                        } else {
                            let z = 'z' as usize;
                            z - (z - c as usize)
                        }
                    })
                    .collect()
            })
            .collect::<Vec<_>>();
        Ok(Input { start, end, m })
    }
}

fn solve(m: &Vec<Vec<usize>>, start: &[(usize, usize)], end: (usize, usize)) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for s in start {
        q.push_back((0, *s));
        seen.insert(*s);
    }
    while let Some((d, (i, j))) = q.pop_front() {
        if (i, j) == end {
            return Some(d);
        }
        for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni < 0 || nj < 0 || ni >= m.len() as isize || nj >= m[0].len() as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if m[ni][nj] <= m[i][j] + 1 && !seen.contains(&(ni, nj)) {
                q.push_back((d + 1, (ni, nj)));
                seen.insert((ni, nj));
            }
        }
    }
    None
}
impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        Ok(solve(&self.m, &[self.start], self.end).unwrap())
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let starts = self
            .m
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
            .filter(|(i, j)| self.m[*i][*j] == 'a' as usize)
            .collect::<Vec<_>>();
        Ok(solve(&self.m, &starts, self.end).unwrap())
    }
}

fn main() {
    Input::solve_and_print_all();
}
