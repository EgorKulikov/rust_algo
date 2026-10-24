//! Level 5: a pick-up arm. After each tick's matter update, if every cell of
//! the pattern holds matter and no neighbouring cell outside the pattern
//! does, the pattern's matter is removed and counted.

use crate::level4::Machine;
use std::collections::HashSet;

/// The six hex neighbours of a cell in the character grid.
pub fn neighbours((r, c): (i32, i32)) -> [(i32, i32); 6] {
    [(r, c - 2), (r, c + 2), (r - 1, c - 1), (r - 1, c + 1), (r + 1, c - 1), (r + 1, c + 1)]
}

pub struct Pattern {
    pub cells: Vec<(i32, i32)>,
    set: HashSet<(i32, i32)>,
}

impl Pattern {
    pub fn new(cells: Vec<(i32, i32)>) -> Pattern {
        let set = cells.iter().copied().collect();
        Pattern { cells, set }
    }

    /// Present at its exact position with no other matter touching it.
    pub fn is_valid(&self, m: &Machine) -> bool {
        self.cells.iter().all(|&(r, c)| m.board.get(r, c) == b'M')
            && self.cells.iter().all(|&cell| {
                neighbours(cell).iter().all(|&(r, c)| self.set.contains(&(r, c)) || m.board.get(r, c) != b'M')
            })
    }

    pub fn remove(&self, m: &mut Machine) {
        for &(r, c) in &self.cells {
            m.board.set(r, c, b'O');
        }
    }
}

pub struct Program {
    pub radius: usize,
    pub pattern: Pattern,
    pub lines: Vec<[u8; 6]>,
}

pub fn parse_programs(input: &str) -> Vec<Program> {
    let mut lines = input.lines().map(str::trim_end).filter(|l| !l.trim().is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut programs = Vec::with_capacity(n);
    for _ in 0..n {
        let dims: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut cells = Vec::new();
        for r in 0..dims[1] {
            for (c, &b) in lines.next().unwrap().as_bytes().iter().enumerate() {
                if b == b'M' {
                    cells.push((r as i32, c as i32));
                }
            }
        }
        let l: usize = lines.next().unwrap().trim().parse().unwrap();
        let prog = (0..l)
            .map(|_| {
                let t: Vec<u8> = lines.next().unwrap().split_whitespace().map(|t| t.as_bytes()[0]).collect();
                [t[0], t[1], t[2], t[3], t[4], t[5]]
            })
            .collect();
        programs.push(Program { radius: dims[0], pattern: Pattern::new(cells), lines: prog });
    }
    programs
}

pub fn run(p: &Program) -> usize {
    let mut m = Machine::new(p.radius);
    let mut count = 0;
    for line in &p.lines {
        m.tick(line);
        if p.pattern.is_valid(&m) {
            p.pattern.remove(&mut m);
            count += 1;
        }
    }
    count
}

pub fn solve(input: &str) -> String {
    parse_programs(input).iter().map(|p| run(p).to_string()).collect::<Vec<_>>().join("\n") + "\n"
}
