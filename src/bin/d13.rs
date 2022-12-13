use aoc_2022::*;
use itertools::Itertools;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::List(l) => write!(f, "[{}]", l.iter().map(|v| v.to_string()).join(",")),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let o = match self {
            vn @ Value::Number(n) => match other {
                Value::Number(m) => n.cmp(m),
                vl @ Value::List(_) => Value::List(vec![vn.clone()]).cmp(vl),
            },
            vl @ Value::List(l) => match other {
                vn @ Value::Number(_) => vl.cmp(&Value::List(vn.into_list())),
                Value::List(m) => l.cmp(m),
            },
        };
        Some(o)
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Value {
    fn into_list(&self) -> Vec<Value> {
        match self {
            Value::List(list) => list.to_vec(),
            Value::Number(n) => vec![Value::Number(*n)],
        }
    }
}
#[derive(Clone, Debug)]
struct Input(Vec<Value>);

#[derive(Clone, Debug)]
enum Token {
    Number(usize),
    OpenBracket,
    CloseBracket,
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut res = vec![];
    let iter = s.chars();
    let mut num = String::new();
    for c in iter {
        if c.is_digit(10) {
            num.push(c);
        } else {
            if !num.is_empty() {
                res.push(Token::Number(num.parse().unwrap()));
                num.clear();
            }
            match c {
                '[' => res.push(Token::OpenBracket),
                ']' => res.push(Token::CloseBracket),
                _ => {}
            }
        }
    }
    if !num.is_empty() {
        res.push(Token::Number(num.parse().unwrap()));
        num.clear();
    }
    res
}

fn parse_list(tokens: &[Token]) -> (Vec<Value>, usize) {
    let mut res = vec![];
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::CloseBracket => break,
            _ => {
                let (v, n) = parse(&tokens[i..]);
                res.push(v);
                i += n;
            }
        }
        i += 1;
    }
    (res, i + 1)
}
fn parse(tokens: &[Token]) -> (Value, usize) {
    let mut val = Value::Number(0);
    let mut i = 0;

    match tokens[i] {
        Token::Number(n) => val = Value::Number(n),
        Token::OpenBracket => {
            let (vali, n) = parse_list(&tokens[i + 1..]);
            i += n;
            val = Value::List(vali);
        }
        _ => {}
    }
    (val, i)
}
impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| parse(&tokenize(l)))
            .map(|(v, _)| v)
            .collect();
        Ok(Input(r))
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let r = self
            .0
            .iter()
            .chunks(2)
            .into_iter()
            .map(|pair| {
                let (p0, p1) = pair.collect_tuple().unwrap();
                p0.cmp(&p1)
            })
            .enumerate()
            .filter(|(_, o)| *o == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum();

        Ok(r)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let mut v = self.0.clone();
        let dp1 = parse(&tokenize("[[2]]")).0;
        let dp2 = parse(&tokenize("[[6]]")).0;

        v.push(dp1.clone());
        v.push(dp2.clone());
        v.sort();

        let (i1, _) = v.iter().find_position(|v| *v == &dp1).unwrap();
        let (i2, _) = v.iter().find_position(|v| *v == &dp2).unwrap();

        Ok((i1 + 1) * (i2 + 1))
    }
}

fn main() {
    Input::solve_and_print_all();
}
