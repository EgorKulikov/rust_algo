//! Level 4: six emitters ride on the border ring. Each tick all six
//! instructions apply at once (F clockwise, B counterclockwise, T toggle),
//! then the active emitters fire with the level 3 rules.

use crate::board::{direction, render_all, Board};
use crate::level1;

/// Border cells of a radius-`r` board in clockwise order (screen sense),
/// starting at the top-left corner. Length 6r.
pub fn ring(r: usize) -> Vec<(i32, i32)> {
    let r = r as i32;
    let mut cells = Vec::new();
    // top row, left to right
    for c in (r..=3 * r).step_by(2) {
        cells.push((0, c));
    }
    // right side going down (last cell of each row)
    for row in 1..=2 * r {
        let d = (row - r).abs();
        cells.push((row, 4 * r - d));
    }
    // bottom row, right to left (excluding its last cell, already added)
    for c in (r..3 * r - 1).rev().step_by(2) {
        cells.push((2 * r, c));
    }
    // left side going up (first cell of each row), excluding bottom-left and top-left corners
    for row in (1..2 * r).rev() {
        let d = (row - r).abs();
        cells.push((row, d));
    }
    cells
}

/// Index in `ring` of emitter k's starting corner.
fn start_index(k: usize, r: usize) -> usize {
    match k {
        1 => 5 * r, // left corner
        2 => 0,     // top-left
        3 => r,     // top-right
        4 => 2 * r, // right
        5 => 3 * r, // bottom-right
        6 => 4 * r, // bottom-left
        _ => unreachable!(),
    }
}

#[derive(Clone)]
pub struct Machine {
    pub board: Board,
    pub ring: Vec<(i32, i32)>,
    pub pos: [usize; 6],
    pub active: [bool; 6],
}

impl Machine {
    pub fn new(r: usize) -> Machine {
        let rows = level1::board(r).lines().map(|l| l.as_bytes().to_vec()).collect();
        let mut pos = [0; 6];
        for k in 1..=6 {
            pos[k - 1] = start_index(k, r);
        }
        Machine { board: Board { rows }, ring: ring(r), pos, active: [false; 6] }
    }

    pub fn tick(&mut self, instr: &[u8; 6]) {
        let n = self.ring.len();
        for k in 0..6 {
            match instr[k] {
                b'F' => self.pos[k] = (self.pos[k] + 1) % n,
                b'B' => self.pos[k] = (self.pos[k] + n - 1) % n,
                b'T' => self.active[k] = !self.active[k],
                _ => {}
            }
        }
        let mut counts: Vec<Vec<u32>> = self.board.rows.iter().map(|r| vec![0; r.len()]).collect();
        for k in 0..6 {
            if !self.active[k] {
                continue;
            }
            let (dr, dc) = direction(k as u8 + 1);
            let (mut r, mut c) = self.ring[self.pos[k]];
            r += dr;
            c += dc;
            while matches!(self.board.get(r, c), b'O' | b'M') {
                counts[r as usize][c as usize] += 1;
                r += dr;
                c += dc;
            }
        }
        crate::level3::apply_counts(&mut self.board, &counts);
        if std::env::var("NANO_DEBUG").is_ok() {
            eprintln!("tick {:?} pos={:?} active={:?}", std::str::from_utf8(instr).unwrap(), self.pos.iter().map(|&p| self.ring[p]).collect::<Vec<_>>(), self.active);
            eprint!("{}", self.board.render());
        }
    }
}

pub fn solve(input: &str) -> String {
    let mut lines = input.lines().map(str::trim).filter(|l| !l.is_empty());
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut boards = Vec::with_capacity(n);
    for _ in 0..n {
        let r: usize = lines.next().unwrap().parse().unwrap();
        let l: usize = lines.next().unwrap().parse().unwrap();
        let mut m = Machine::new(r);
        for _ in 0..l {
            let toks: Vec<u8> = lines.next().unwrap().split_whitespace().map(|t| t.as_bytes()[0]).collect();
            m.tick(&[toks[0], toks[1], toks[2], toks[3], toks[4], toks[5]]);
        }
        boards.push(m.board);
    }
    render_all(&boards)
}

#[cfg(test)]
mod tests {
    use super::ring;

    #[test]
    fn ring_has_6r_border_cells_in_order() {
        for r in 1..6usize {
            let cells = ring(r);
            assert_eq!(cells.len(), 6 * r, "r = {}", r);
            let b = crate::level1::board(r);
            let rows: Vec<&[u8]> = b.lines().map(str::as_bytes).collect();
            for (i, &(row, col)) in cells.iter().enumerate() {
                assert_eq!(rows[row as usize][col as usize], b'*', "r={} cell {} = {:?}", r, i, (row, col));
                let (nr, nc) = cells[(i + 1) % cells.len()];
                assert!((nr - row).abs() <= 1 && (nc - col).abs() <= 2, "r={} jump {:?}->{:?}", r, (row, col), (nr, nc));
            }
        }
    }
}
