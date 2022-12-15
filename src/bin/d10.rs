use aoc_2022::*;
use std::str::FromStr;

#[derive(Clone, Debug, Copy)]
enum Ops {
    Noop(),
    AddX(isize),
}

impl Ops {
    fn duration(&self) -> usize {
        match self {
            Ops::Noop() => 1,
            Ops::AddX(_) => 2,
        }
    }
}

struct Input(Vec<Ops>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|l| {
                let (op, val) = l.split_once(' ').unwrap_or((l, ""));
                match op {
                    "noop" => Ok(Ops::Noop()),
                    "addx" => Ok(Ops::AddX(val.parse().unwrap())),
                    _ => Err(()),
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Input)
    }
}

struct Cpu {
    x: isize,
    clock: usize,
}

impl Cpu {
    fn exec(&mut self, op: Ops, mut inspect: impl FnMut(&Cpu)) {
        let cycles = op.duration();
        for _ in 0..cycles {
            inspect(self);
            self.clock += 1;
        }
        match op {
            Ops::Noop() => {}
            Ops::AddX(v) => self.x += v,
        }
    }
    fn mainloop(&mut self, ops: &Vec<Ops>, mut inspect: impl FnMut(&Cpu)) {
        let mut pc = 0;
        while pc < ops.len() {
            // fetch
            let op = ops[pc];
            // execute
            self.exec(op, &mut inspect);
            pc += 1;
        }
    }
}

impl Solve for Input {
    type Output = isize;
    fn solve1(&self) -> anyhow::Result<isize> {
        let mut cpu = Cpu { x: 1, clock: 1 };
        let mut tot = 0isize;
        cpu.mainloop(&self.0, |c| {
            if c.clock == 20 || (c.clock > 20 && (c.clock - 20) % 40 == 0) {
                println!("{}: {}", c.clock, c.x);
                tot += c.clock as isize * c.x;
            }
        });
        Ok(tot)
    }

    fn solve2(&self) -> anyhow::Result<isize> {
        let mut cpu = Cpu { x: 1, clock: 1 };
        let mut crt = vec![];
        cpu.mainloop(&self.0, |c| {
            let pixel_being_drawn = (c.clock as isize - 1) % 40;
            if [c.x - 1, c.x, c.x + 1].contains(&pixel_being_drawn) {
                crt.push('#')
            } else {
                crt.push('.')
            }
        });
        let lines = crt.chunks(40).collect::<Vec<_>>();
        for line in lines {
            for x in line {
                print!("{}", x);
            }
            println!();
        }
        Ok(0) // irrelevant, the output is the CRT display
    }
}

fn main() {
    Input::solve_and_print_all();
}
