//{"name":"day10","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day10"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut t = Vec::new();
    while !input.is_exhausted() {
        t.push(input.read_line());
    }

    let mut r = 0;
    let mut c = 0;
    for i in 0..t.len() {
        for j in 0..t[i].len() {
            if t[i][j] == b'S' {
                r = i;
                c = j;
            }
        }
    }
    let near1 = |r1: usize, c1: usize, r2, c2| -> bool {
        let c = t[r1][c1];
        if c == b'S' {
            return true;
        }
        if r2 == r1 + 1 {
            c == b'|' || c == b'7' || c == b'F'
        } else if r1 == r2 + 1 {
            c == b'|' || c == b'J' || c == b'L'
        } else if c2 == c1 + 1 {
            c == b'-' || c == b'L' || c == b'F'
        } else {
            c == b'-' || c == b'7' || c == b'J'
        }
    };
    let near = |r1, c1, r2, c2| -> bool { near1(r1, c1, r2, c2) && near1(r2, c2, r1, c1) };
    let connected = |r, c| -> Vec<(usize, usize)> {
        let mut ans = Vec::new();
        for (i, j) in D4::iter(r, c, t.len(), t[0].len()) {
            if near(r, c, i, j) {
                ans.push((i, j));
            }
        }
        assert!(ans.len() == 2);
        ans
    };

    let conn = connected(r, c);
    let mut cur = conn[0];
    let mut prev = (r, c);
    let mut l = vec![prev, cur];
    while cur != conn[1] {
        let next = connected(cur.0, cur.1)
            .into_iter()
            .filter(|x| *x != prev)
            .next()
            .unwrap();
        prev = cur;
        cur = next;
        l.push(cur);
    }

    {
        // part 1
        out.print_line(l.len() / 2);
    }

    {
        // part 2
        l.push((r, c));
        let mut grid = Arr2d::new(2 * t.len() - 1, 2 * t[0].len() - 1, false);
        for (&(r1, c1), &(r2, c2)) in l.consecutive_iter() {
            grid[(2 * r1, 2 * c1)] = true;
            grid[(r1 + r2, c1 + c2)] = true;
        }
        let mut ans = 0;
        for i in grid.rows() {
            for j in grid.cols() {
                if grid[(i, j)] {
                    continue;
                }
                let mut q = Vec::new();
                q.push((i, j));
                let mut qty = 0;
                grid[(i, j)] = true;
                let mut outer = i == 0 || j == 0 || i == grid.d1() - 1 || j == grid.d2() - 1;
                while let Some((r, c)) = q.pop() {
                    if r % 2 == 0 && c % 2 == 0 {
                        qty += 1;
                    }
                    if r == 0 || c == 0 || r == grid.d1() - 1 || c == grid.d2() - 1 {
                        outer = true;
                    }
                    for (r1, c1) in D4::iter(r, c, grid.d1(), grid.d2()) {
                        if !grid[(r1, c1)] {
                            grid[(r1, c1)] = true;
                            q.push((r1, c1));
                        }
                    }
                }
                if !outer {
                    ans += qty;
                }
            }
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
