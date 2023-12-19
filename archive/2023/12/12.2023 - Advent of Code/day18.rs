//{"name":"day18","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day18"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::compress::compress;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();

    let solve = |from_col: bool| -> i64 {
        let mut pos_r = vec![0];
        let mut pos_c = vec![0];
        let mut poi_r = vec![1];
        let mut poi_c = vec![1];
        let mut r = 0;
        let mut c = 0;
        let mut ans = 0;
        for s in &lines {
            let mut bytes = s.as_slice();
            let mut inp = Input::new(&mut bytes);
            scan!(&mut inp, "@ @ (#@)", dir: char, len: i64, col: Str<'static>);
            let (len, dir) = if from_col {
                (
                    i64::from_str_radix(&Str::from(&col[..5]).to_string(), 16).unwrap(),
                    col[5] - b'0',
                )
            } else {
                (
                    len,
                    match dir {
                        'R' => 0,
                        'D' => 1,
                        'L' => 2,
                        'U' => 3,
                        _ => unreachable!(),
                    },
                )
            };
            match dir {
                0 => c += len,
                1 => r += len,
                2 => c -= len,
                3 => r -= len,
                _ => unreachable!(),
            }
            ans += len;
            pos_r.push(r);
            pos_c.push(c);
            poi_r.push(r + 1);
            poi_c.push(c + 1);
        }
        let (rr, [pos_r, _]) = compress([&pos_r, &poi_r]);
        let (cc, [pos_c, _]) = compress([&pos_c, &poi_c]);
        let mut grid = Arr2d::new(rr.len() * 2 - 1, cc.len() * 2 - 1, false);
        let pos = pos_r.into_iter().zip(pos_c).collect_vec();
        for (&(r1, c1), &(r2, c2)) in pos.consecutive_iter() {
            for i in 2 * r1.min(r2)..=2 * r1.max(r2) {
                for j in 2 * c1.min(c2)..=2 * c1.max(c2) {
                    grid[(i, j)] = true;
                }
            }
        }
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
                    if r % 2 == 0 && c % 2 == 0 && r + 1 < grid.d1() && c + 1 < grid.d2() {
                        qty += (rr[r / 2 + 1] - rr[r / 2]) * (cc[c / 2 + 1] - cc[c / 2]);
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
        ans
    };

    {
        // part 1
        out.print_line(solve(false));
    }

    {
        // part 2
        out.print_line(solve(true));
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
