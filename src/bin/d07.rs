// This is like, an entire language with parsing and evaluation steps. Overengineered? Probably.
// Do I like it? Yes.

use anyhow::{anyhow, Context};
use aoc_2022::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Dir {
    path: String,
}
#[derive(Debug, Clone)]
struct File {
    size: usize,
}
#[derive(Debug, Clone)]
struct CommandCd {
    path: String,
}
#[derive(Debug, Clone)]
struct CommandLs();

#[derive(Debug, Clone)]
enum Line {
    Dir(Dir),
    File(File),
    CommandCd(CommandCd),
    CommandLs(CommandLs),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(dir) = s.parse() {
            Ok(Line::Dir(dir))
        } else if let Ok(file) = s.parse() {
            Ok(Line::File(file))
        } else if let Ok(cmd) = s.parse() {
            Ok(Line::CommandCd(cmd))
        } else if let Ok(cmd) = s.parse() {
            Ok(Line::CommandLs(cmd))
        } else {
            Err(anyhow!("invalid line"))
        }
    }
}

impl FromStr for CommandCd {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, path) = s
            .strip_prefix("$ ")
            .context("not a cmd")?
            .split_once(' ')
            .context("missing space")?;
        if cmd == "cd" {
            Ok(CommandCd {
                path: path.to_string(),
            })
        } else {
            Err(anyhow!("cmd is not cd"))
        }
    }
}
impl FromStr for CommandLs {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        (s.strip_prefix("$ ").context("not a cmd")? == "ls")
            .then(CommandLs)
            .ok_or_else(|| anyhow!("cmd is not ls"))
    }
}
impl FromStr for File {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, _) = s.split_once(' ').context("missing space")?;
        let size = size.parse::<usize>()?;
        Ok(File { size })
    }
}
impl FromStr for Dir {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, path) = s.split_once(' ').context("missing space")?;
        anyhow::ensure!(dir == "dir", "not a dir");
        Ok(Dir {
            path: path.to_string(),
        })
    }
}
struct Input {
    lines: Vec<Line>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Result<Vec<Line>, anyhow::Error> =
            s.lines().map(|l| l.parse::<Line>()).collect();
        Ok(Input { lines: lines? })
    }
}

#[derive(Debug)]
struct DirValue {
    parent: Option<Weak<RefCell<DirValue>>>,
    path: String,
    size: usize,
    child: Vec<Rc<RefCell<DirValue>>>,
}

impl DirValue {
    fn traverse(&self, mut f: impl FnMut(&DirValue)) {
        Self::traverse_rec(&mut f, self)
    }

    fn traverse_rec(f: &mut impl FnMut(&DirValue), dir: &DirValue) {
        f(dir);
        for d in &dir.child {
            Self::traverse_rec(f, &d.borrow());
        }
    }
}
struct Runtime {
    root: Rc<RefCell<DirValue>>,
    current: Rc<RefCell<DirValue>>,
}
impl Runtime {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(DirValue {
            parent: None,
            path: "/".to_string(),
            size: 0,
            child: vec![],
        }));
        Self {
            root: root.clone(),
            current: root,
        }
    }
    fn exec_cd(&mut self, cmd: CommandCd) {
        let sub_dir = {
            let current = self.current.borrow();
            match cmd.path.as_str() {
                ".." => current.parent.clone().unwrap().upgrade().unwrap(),
                "." => self.current.clone(),
                "/" => self.root.clone(),
                path => current
                    .child
                    .iter()
                    .find(|d| d.borrow().path == *path)
                    .unwrap()
                    .clone(),
            }
        };
        self.current = sub_dir;
    }
    fn exec_ls_line_file(&mut self, file: File) {
        self.bubble_up_size(file.size, self.current.clone());
    }
    fn bubble_up_size(&mut self, size: usize, current: Rc<RefCell<DirValue>>) {
        let mut current = current.borrow_mut();
        current.size += size;
        if let Some(parent) = current.parent.clone() {
            self.bubble_up_size(size, parent.upgrade().unwrap());
        }
    }
    fn exec_ls_line_dir(&mut self, dir: Dir) {
        let mut current = self.current.borrow_mut();
        let new_dir = Rc::new(RefCell::new(DirValue {
            parent: Some(Rc::downgrade(&self.current)),
            path: dir.path,
            size: 0,
            child: vec![],
        }));
        current.child.push(new_dir);
    }
    fn exec_lines(&mut self, lines: impl Iterator<Item = Line>) {
        for line in lines {
            match line {
                Line::Dir(dir) => self.exec_ls_line_dir(dir),
                Line::File(file) => self.exec_ls_line_file(file),
                Line::CommandCd(cmd) => self.exec_cd(cmd),
                Line::CommandLs(_cmd) => { // useless
                }
            }
        }
    }
}
impl Solve for Input {
    type Output = usize;
    fn solve1(&self) -> anyhow::Result<usize> {
        const THRESHOLD: usize = 100000;

        let mut r = Runtime::new();
        r.exec_lines(self.lines.clone().into_iter());

        let mut smaller = 0;
        r.root.borrow().traverse(|d| {
            if d.size <= THRESHOLD {
                smaller += d.size;
            }
        });
        Ok(smaller)
    }

    fn solve2(&self) -> anyhow::Result<usize> {
        const DISK_SIZE: usize = 70000000;
        const NEEDED: usize = 30000000;

        let mut r = Runtime::new();
        r.exec_lines(self.lines.clone().into_iter());

        let consumed = r.root.borrow().size;
        let to_free = NEEDED - (DISK_SIZE - consumed);

        let mut min_found = usize::MAX;
        r.root.borrow().traverse(|d| {
            if d.size >= to_free && d.size < min_found {
                min_found = d.size;
            }
        });
        Ok(min_found)
    }
}

fn main() {
    Input::solve_and_print_all();
}
