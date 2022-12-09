// I should've used some library to do vector operations...

use aoc_2022::*;
use std::collections::HashSet;
use std::str::FromStr;

struct Input(Vec<(char, usize)>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .lines()
            .map(|l| l.split_once(' ').unwrap())
            .map(|(c, n)| (c.chars().next().unwrap(), n.parse().unwrap()));
        Ok(Input(r.collect()))
    }
}

const DIAGONALS: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];

struct GridState {
    rope: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl GridState {
    fn new(n: usize) -> Self {
        let mut this = GridState {
            rope: (0..n).map(|_| (0, 0)).collect(),
            visited: HashSet::new(),
        };
        this.visited.insert((0, 0));
        this
    }

    fn move_head(&mut self, dir: char, n: usize) {
        (0..n).for_each(|_| self.move_head1(dir, 0));
    }
    fn move_head1(&mut self, dir: char, i: usize) {
        if i >= self.rope.len() {
            return;
        }
        let (x, y) = self.rope[i];
        self.rope[i] = match dir {
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            'R' => (x + 1, y),
            'L' => (x - 1, y),
            _ => unreachable!(),
        };

        self.step_node_towards_target(i + 1, i);
    }
    fn node_near(a: (isize, isize), b: (isize, isize)) -> bool {
        let d = Self::distance(a, b);
        d <= 2f32.sqrt()
    }
    fn step_node_towards_target(&mut self, i: usize, target: usize) {
        if (i >= self.rope.len()) || (i == target) {
            return;
        }
        let len = self.rope.len();

        assert!(i > target);
        let split = self.rope.split_at_mut(i);
        let tail = &mut split.1[0];
        let target = &mut split.0.split_at_mut(target).1[0];

        if Self::node_near(*tail, *target) {
            return;
        }
        let d = Self::distance(*target, *tail);
        if d.fract() == 0.0 {
            let s = Self::sub(*target, *tail);
            let d = d as isize;
            let n = (s.0 / d, s.1 / d);
            *tail = Self::add(*tail, n);
        } else {
            let min_point = DIAGONALS
                .into_iter()
                .map(|p| {
                    let d = Self::distance(*target, Self::add(*tail, p));
                    (d, p)
                })
                .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap())
                .unwrap();
            *tail = Self::add(*tail, min_point.1);
        }
        if i == len - 1 {
            self.visited.insert(*tail);
        }
        self.step_node_towards_target(i + 1, i);
    }

    fn sub(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
        (a.0 - b.0, a.1 - b.1)
    }
    fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
        (a.0 + b.0, a.1 + b.1)
    }
    fn distance(a: (isize, isize), b: (isize, isize)) -> f32 {
        let (dx, dy) = Self::sub(a, b);
        let dx = dx as f32;
        let dy = dy as f32;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let r: GridState = self
            .0
            .iter()
            .fold(GridState::new(2), |mut state, (dir, n)| {
                state.move_head(*dir, *n);
                state
            });
        Ok(r.visited.len())
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let r: GridState = self
            .0
            .iter()
            .fold(GridState::new(10), |mut state, (dir, n)| {
                state.move_head(*dir, *n);
                state
            });
        Ok(r.visited.len())
    }
}

fn main() {
    Input::solve_and_print_all();
}
