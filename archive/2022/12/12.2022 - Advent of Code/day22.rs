//{"name":"day22","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day22"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out_line, scan};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    scan!(input, "side=@", side: usize);
    let mut wrap = HashMap::new();
    for _ in 0..24 {
        scan!(
            input,
            "@,@,@->@,@,@,@",
            r0: usize,
            c0: usize,
            d0: usize,
            r1: usize,
            c1: usize,
            d1: usize,
            s: char
        );
        wrap.insert((r0, c0, d0), ((r1, c1, d1), s == '+'));
        // if wrap.contains_key(&(r1, c1, d1)) {
        //     assert_eq!(wrap[&(r1, c1, d1)], ((r0, c0, d0), s == '+'));
        // }
    }

    for (&(r0, c0, d0), &((r1, c1, d1), s)) in wrap.iter() {
        assert_eq!(wrap[&(r1, c1, d1)], ((r0, c0, d0), s));
    }

    let mut table: Vec<Str> = Vec::new();

    loop {
        let s = input.read_line();
        if s.is_empty() {
            break;
        }
        table.push(s.into());
    }

    let s: Str = input.read();

    let mut row = 0;
    let mut col = table[0].iter().position(|c| c != b' ').unwrap();
    let mut dir = 0;
    let mut len = 0;

    let next_pos =
        |nsr: usize, nsc: usize, nd: usize, d: bool, x: usize| -> (usize, usize, usize) {
            match nd {
                0 => (
                    nsr * side + if !d { side - 1 - x } else { x },
                    nsc * side + side - 1,
                    2,
                ),
                1 => (
                    nsr * side + side - 1,
                    nsc * side + if !d { side - 1 - x } else { x },
                    3,
                ),
                2 => (
                    nsr * side + if !d { side - 1 - x } else { x },
                    nsc * side,
                    0,
                ),
                3 => (
                    nsr * side,
                    nsc * side + if !d { side - 1 - x } else { x },
                    1,
                ),
                _ => unreachable!(),
            }
        };

    for c in s {
        let mut go = || {
            for _ in 0..len {
                let (mut r, mut c) = (row, col);
                let (n_row, n_col, n_dir) = loop {
                    let sr = r / side;
                    let sc = c / side;
                    let mut n_dir = dir;
                    match dir {
                        0 => {
                            c += 1;
                            if c % side == 0 {
                                let x = r % side;
                                let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                                (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                            }
                        }
                        1 => {
                            r += 1;
                            if r % side == 0 {
                                let x = c % side;
                                let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                                (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                            }
                        }
                        2 => {
                            if c % side == 0 {
                                let x = r % side;
                                let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                                (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                            } else {
                                c -= 1;
                            }
                        }
                        3 => {
                            if r % side == 0 {
                                let x = c % side;
                                let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                                (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                            } else {
                                r -= 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                    assert!(c < table[r].len() && table[r][c] != b' ');
                    if c < table[r].len() && table[r][c] != b' ' {
                        break (r, c, n_dir);
                    }
                };
                if table[n_row][n_col] == b'#' {
                    break;
                }
                (row, col, dir) = (n_row, n_col, n_dir);
            }
            len = 0;
        };
        match c {
            b'L' => {
                go();
                dir = (dir + 3) % 4
            }
            b'R' => {
                go();
                dir = (dir + 1) % 4
            }
            d => {
                len *= 10;
                len += (d - b'0') as usize;
            }
        }
    }
    let mut go = || {
        for _ in 0..len {
            let (mut r, mut c) = (row, col);
            let (n_row, n_col, n_dir) = loop {
                let sr = r / side;
                let sc = c / side;
                let mut n_dir = dir;
                match dir {
                    0 => {
                        c += 1;
                        if c % side == 0 {
                            let x = r % side;
                            let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                            (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                        }
                    }
                    1 => {
                        r += 1;
                        if r % side == 0 {
                            let x = c % side;
                            let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                            (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                        }
                    }
                    2 => {
                        if c % side == 0 {
                            let x = r % side;
                            let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                            (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                        } else {
                            c -= 1;
                        }
                    }
                    3 => {
                        if r % side == 0 {
                            let x = c % side;
                            let ((nsr, nsc, nd), d) = wrap[&(sr, sc, dir)];
                            (r, c, n_dir) = next_pos(nsr, nsc, nd, d, x);
                        } else {
                            r -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
                assert!(c < table[r].len() && table[r][c] != b' ');
                if c < table[r].len() && table[r][c] != b' ' {
                    break (r, c, n_dir);
                }
            };
            if table[n_row][n_col] == b'#' {
                break;
            }
            (row, col, dir) = (n_row, n_col, n_dir);
        }
        len = 0;
    };
    go();

    out_line!(1000 * (row + 1) + 4 * (col + 1) + dir);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
