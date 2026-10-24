//! Level 3: several emitters at half power. Beams pass through 'O' and 'M'
//! and stop at anything else. A cell crossed by exactly two beams becomes
//! matter; a cell crossed by three or more loses its matter; otherwise it is
//! left as it was.

use crate::board::{direction, parse_boards, render_all, Board};

pub fn beam_counts(board: &Board) -> Vec<Vec<u32>> {
    let mut counts: Vec<Vec<u32>> = board.rows.iter().map(|r| vec![0; r.len()]).collect();
    for (r, c) in board.find(|b| (b'1'..=b'6').contains(&b)) {
        let (dr, dc) = direction(board.get(r, c) - b'0');
        let (mut r, mut c) = (r + dr, c + dc);
        while matches!(board.get(r, c), b'O' | b'M') {
            counts[r as usize][c as usize] += 1;
            r += dr;
            c += dc;
        }
    }
    counts
}

pub fn apply_counts(board: &mut Board, counts: &[Vec<u32>]) {
    for (r, row) in counts.iter().enumerate() {
        for (c, &k) in row.iter().enumerate() {
            if k == 2 {
                board.rows[r][c] = b'M';
            } else if k >= 3 {
                board.rows[r][c] = b'O';
            }
        }
    }
}

pub fn solve(input: &str) -> String {
    let mut boards = parse_boards(input);
    for board in boards.iter_mut() {
        let counts = beam_counts(board);
        apply_counts(board, &counts);
    }
    render_all(&boards)
}
