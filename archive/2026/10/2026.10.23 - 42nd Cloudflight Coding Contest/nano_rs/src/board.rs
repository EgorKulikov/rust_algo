//! Character-grid board shared by the levels: rows of bytes, blank-line
//! separated paragraphs in the input, each preceded by "radius rows cols".

#[derive(Clone)]
pub struct Board {
    pub rows: Vec<Vec<u8>>,
}

impl Board {
    pub fn get(&self, r: i32, c: i32) -> u8 {
        if r < 0 || c < 0 || r as usize >= self.rows.len() || c as usize >= self.rows[r as usize].len() {
            b' '
        } else {
            self.rows[r as usize][c as usize]
        }
    }

    pub fn set(&mut self, r: i32, c: i32, v: u8) {
        self.rows[r as usize][c as usize] = v;
    }

    pub fn find(&self, pred: impl Fn(u8) -> bool) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        for (r, row) in self.rows.iter().enumerate() {
            for (c, &b) in row.iter().enumerate() {
                if pred(b) {
                    res.push((r as i32, c as i32));
                }
            }
        }
        res
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        for row in &self.rows {
            s.push_str(std::str::from_utf8(row).unwrap());
            s.push('\n');
        }
        s
    }
}

/// Step in the character grid for emitter directions 1..=6.
pub fn direction(d: u8) -> (i32, i32) {
    match d {
        1 => (0, 2),
        2 => (1, 1),
        3 => (1, -1),
        4 => (0, -2),
        5 => (-1, -1),
        6 => (-1, 1),
        _ => panic!("bad direction {}", d),
    }
}

/// Parses "N" then N paragraphs of "radius rows cols" + rows.
pub fn parse_boards(input: &str) -> Vec<Board> {
    let mut lines = input.lines().map(str::trim_end).filter(|l| !l.is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut boards = Vec::with_capacity(n);
    for _ in 0..n {
        let dims: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let rows = (0..dims[1]).map(|_| lines.next().unwrap().as_bytes().to_vec()).collect();
        boards.push(Board { rows });
    }
    boards
}

pub fn render_all(boards: &[Board]) -> String {
    boards.iter().map(Board::render).collect::<Vec<_>>().join("\n") + "\n"
}
