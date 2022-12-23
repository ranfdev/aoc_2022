use aoc_2022::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct SymbolTable {
    symbols: Vec<String>,
}
impl SymbolTable {
    fn insert(&mut self, s: String) -> usize {
        if let Some(i) = self.position(&s) {
            i
        } else {
            let i = self.symbols.len();
            self.symbols.push(s);
            i
        }
    }
    fn position(&self, s: &str) -> Option<usize> {
        self.symbols
            .iter()
            .enumerate()
            .find(|(_, v)| v.as_str() == s)
            .map(|(i, _)| i)
    }
}

#[derive(Clone, Debug)]
struct Graph {
    st: SymbolTable,
    adj: Vec<Vec<usize>>,
    flow: Vec<usize>,
    open: u64,
    cache: HashMap<(usize, usize, usize, u64), usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            st: SymbolTable { symbols: vec![] },
            adj: vec![],
            flow: vec![],
            open: 0,
            cache: HashMap::with_capacity(1000),
        }
    }
}
impl Graph {
    fn insert(&mut self, v: usize, w: usize, flow: usize) {
        if v.max(w) >= self.adj.len() {
            self.adj.resize(v.max(w) + 1, vec![]);
            self.flow.resize(v.max(w) + 1, 0);
        }
        self.adj[v].push(w);
        self.flow[v] = flow;
    }
    fn find_start(&self) -> usize {
        self.st.position("AA").unwrap()
    }
    fn solve(&mut self, time: usize, v: usize, players: usize) -> usize {
        self.solve_rec(time, v, players, &(time, v))
    }
    fn solve_rec(
        &mut self,
        time: usize,
        v: usize,
        players: usize,
        init @ (init_time, init_v): &(usize, usize),
    ) -> usize {
        let k = (time, v, players, self.open);
        if let Some(n) = self.cache.get(&k) {
            return *n;
        }
        if time == 0 {
            return if players > 1 {
                self.solve_rec(*init_time, *init_v, players - 1, init)
            } else {
                0
            };
        }
        let mut max = 0;
        if self.open & (1 << v) < 1 && self.flow[v] > 0 {
            self.open |= 1 << v;
            if time >= 2 {
                for i in 0..self.adj[v].len() {
                    let nv = self.adj[v][i];
                    max = max.max(self.solve_rec(time - 2, nv, players, init));
                }
            }
            max += (time - 1) * self.flow[v];
            self.open ^= 1 << v;
        }
        for i in 0..self.adj[v].len() {
            max = max.max(self.solve_rec(time - 1, self.adj[v][i], players, init));
        }
        self.cache.insert(k, max);

        max
    }
}

struct Input(Graph);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = Graph::new();
        let re =
            regex::Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves?( .+,?)")
                .unwrap();
        re.captures_iter(s).for_each(|c| {
            let src = g.st.insert(c[1].to_string());
            let rate: usize = c[2].parse().unwrap();
            c[3].split(',').map(|x| x.trim_start()).for_each(|x| {
                let n = g.st.insert(x.to_owned());
                g.insert(src, n, rate);
            });
        });
        Ok(Input(g))
    }
}

impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        let mut g = self.0.clone();
        let start = g.find_start();
        Ok(g.solve(30, start, 1))
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        let mut g = self.0.clone();
        let start = g.find_start();
        Ok(g.solve(26, start, 2))
    }
}

fn main() {
    Input::solve_and_print_all();
}
