//! Level 2: one emitter on the border fires a straight beam; every 'O' on its
//! way becomes 'M'. The beam stops at anything that is not 'O'.

use crate::board::{direction, parse_boards, render_all, Board};

pub fn fire(board: &mut Board, (r, c): (i32, i32), dir: u8) {
    let (dr, dc) = direction(dir);
    let (mut r, mut c) = (r + dr, c + dc);
    while board.get(r, c) == b'O' {
        board.set(r, c, b'M');
        r += dr;
        c += dc;
    }
}

pub fn solve(input: &str) -> String {
    let mut boards = parse_boards(input);
    for board in boards.iter_mut() {
        for (r, c) in board.find(|b| (b'1'..=b'6').contains(&b)) {
            let d = board.get(r, c) - b'0';
            fire(board, (r, c), d);
        }
    }
    render_all(&boards)
}
