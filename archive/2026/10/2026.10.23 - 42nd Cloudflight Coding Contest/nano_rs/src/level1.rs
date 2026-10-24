//! Level 1: draw the hexagonal board of radius r.
//! 2r+1 rows; the row at distance d from the middle holds 2r+1-d cells,
//! indented by d dashes, cells separated by dashes. Border cells are '*'.

pub fn board(r: usize) -> String {
    let mut out = String::new();
    for row in 0..=2 * r {
        let d = row.abs_diff(r);
        let cells = 2 * r + 1 - d;
        let mut line = "-".repeat(d);
        for c in 0..cells {
            if c > 0 {
                line.push('-');
            }
            let border = d == r || c == 0 || c == cells - 1;
            line.push(if border { '*' } else { 'O' });
        }
        line.push_str(&"-".repeat(d));
        out.push_str(&line);
        out.push('\n');
    }
    out
}

pub fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let boards: Vec<String> = (0..n).map(|_| board(it.next().unwrap().parse().unwrap())).collect();
    boards.join("\n") + "\n"
}
