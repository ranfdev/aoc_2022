use aoc_2022::*;
use std::str::FromStr;

struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, moves) = s.split_once("\n\n").unwrap();
        let stacks: Vec<Vec<char>> = stacks
            .rsplit_once('\n')
            .unwrap()
            .0
            .lines()
            .flat_map(|l| l.chars().skip(1).step_by(4).enumerate())
            .filter(|(_, c)| *c != ' ')
            .fold(vec![], |mut v, (i, c)| {
                v.resize(v.len().max(i + 1), vec![]);
                v[i].insert(0, c);
                v
            });

        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        Ok(Input {
            stacks,
            moves: moves
                .lines()
                .map(|l| {
                    let caps = re.captures(l).unwrap();
                    (
                        caps[1].parse().unwrap(),
                        caps[2].parse::<usize>().unwrap() - 1usize,
                        caps[3].parse::<usize>().unwrap() - 1usize,
                    )
                })
                .collect(),
        })
    }
}

impl Solve for Input {
    type Output = String;
    fn solve1(&self) -> anyhow::Result<Self::Output> {
        let res = self
            .moves
            .iter()
            .fold(self.stacks.clone(), |mut stacks, (n, from, to)| {
                let l = stacks[*from].len();
                let mut tail = stacks[*from].split_off(l - *n);
                tail.reverse();
                stacks[*to].append(&mut tail);
                stacks
            })
            .iter()
            .filter_map(|s| s.last())
            .collect::<String>();
        Ok(res)
    }

    fn solve2(&self) -> anyhow::Result<Self::Output> {
        let res = self
            .moves
            .iter()
            .fold(self.stacks.clone(), |mut stacks, (n, from, to)| {
                let l = stacks[*from].len();
                let mut tail = stacks[*from].split_off(l - *n);
                stacks[*to].append(&mut tail);
                stacks
            })
            .iter()
            .filter_map(|s| s.last())
            .collect::<String>();
        Ok(res)
    }
}

fn main() {
    Input::solve_and_print_all();
}
