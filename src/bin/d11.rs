use aoc_2022::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Expr(String);
impl Expr {
    fn eval(&self, old: isize) -> isize {
        if &self.0 == "old" {
            old
        } else {
            self.0.parse().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(Expr),
    Mul(Expr),
}
#[derive(Debug, Clone, Copy)]
struct MonkeyTest {
    divisor: isize,
    ontrue: isize,
    onfalse: isize,
}
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    operation: Op,
    test: MonkeyTest,
    inspected: usize,
}
impl Monkey {
    fn eval_op(&self, old: isize) -> isize {
        match &self.operation {
            Op::Add(e) => old + e.eval(old),
            Op::Mul(e) => old * e.eval(old),
        }
    }
    fn eval_op_with_mod(&self, old: isize, p: isize) -> isize {
        match &self.operation {
            Op::Add(e) => ((old % p) + (e.eval(old) % p)) % p,
            Op::Mul(e) => ((old % p) * (e.eval(old) % p)) % p,
        }
    }
}

struct Input(Vec<Monkey>);

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (_, _id) = lines
            .next()
            .unwrap()
            .split_whitespace()
            .next_tuple()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(',').parse().unwrap())
            .collect();
        let (_, _, _, _, operation, expr) = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap();
        let operation = match operation {
            "+" => Op::Add(Expr(expr.to_string())),
            "*" => Op::Mul(Expr(expr.to_string())),
            _ => panic!(),
        };
        let (_, _, _, divisor) = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap();
        let divisor = divisor.parse().unwrap();
        let (_, _, _, _, _, i) = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap();
        let (_, _, _, _, _, j) = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap();
        let test = MonkeyTest {
            divisor,
            ontrue: i.parse().unwrap(),
            onfalse: j.parse().unwrap(),
        };
        Ok(Monkey {
            items,
            operation,
            test,
            inspected: 0,
        })
    }
}
impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.split("\n\n")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>(),
        ))
    }
}

fn solve(monkeys: &[Monkey], rounds: usize, divide_by: isize, reduce_with_mod: bool) -> usize {
    let p = monkeys.iter().map(|m| m.test.divisor).product();
    let mut monkeys: Vec<RefCell<_>> = monkeys.iter().map(|x| RefCell::new(x.clone())).collect();
    for _ in 0..rounds {
        for m in &monkeys {
            let mut m = m.borrow_mut();
            for item in &m.items {
                let new_worry_level = if reduce_with_mod {
                    m.eval_op_with_mod(*item, p)
                } else {
                    m.eval_op(*item)
                } / divide_by as isize;
                let throw_to = if new_worry_level % m.test.divisor == 0 {
                    m.test.ontrue
                } else {
                    m.test.onfalse
                };
                let mut recipient = monkeys[throw_to as usize].borrow_mut();
                recipient.items.push(new_worry_level);
            }
            m.inspected += m.items.len();
            m.items.clear();
        }
    }
    monkeys.sort_by_key(|x| x.borrow().inspected);
    let prod: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|x| x.borrow().inspected as usize)
        .product::<usize>();
    prod
}
impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        Ok(solve(&self.0, 20, 3, false))
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        Ok(solve(&self.0, 10_000, 1, true))
    }
}

fn main() {
    Input::solve_and_print_all();
}
