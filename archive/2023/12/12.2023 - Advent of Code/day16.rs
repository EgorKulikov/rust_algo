//{"name":"day16","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day16"}}}

use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::string::str::StrReader;
use std::collections::{HashSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();

    let n = lines.len();
    let m = lines[0].len();
    let go = |row: usize, col: usize, dir: usize| -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((row, col, dir));
        let mut done = Arr3d::new(n, m, 4, false);
        done[(0, 0, 0)] = true;
        let mut ans = HashSet::new();
        while let Some((r, c, dir)) = queue.pop_front() {
            ans.insert((r, c));
            let next = match lines[r][c] {
                b'.' => {
                    vec![(r, c, dir)]
                }
                b'\\' => {
                    let ndir = dir ^ 1;
                    vec![(r, c, ndir)]
                }
                b'/' => {
                    let ndir = dir ^ 3;
                    vec![(r, c, ndir)]
                }
                b'-' => {
                    if dir % 2 == 0 {
                        vec![(r, c, dir)]
                    } else {
                        vec![(r, c, (dir + 1) % 4), (r, c, (dir + 3) % 4)]
                    }
                }
                b'|' => {
                    if dir % 2 == 1 {
                        vec![(r, c, dir)]
                    } else {
                        vec![(r, c, (dir + 1) % 4), (r, c, (dir + 3) % 4)]
                    }
                }
                _ => unreachable!(),
            };
            for (nr, nc, dir) in next {
                let (nr, nc) = D4::go(nr, nc, dir, n, m);
                if (r, c) == (nr, nc) {
                    continue;
                }
                if !done[(nr, nc, dir)] {
                    done[(nr, nc, dir)] = true;
                    queue.push_back((nr, nc, dir));
                }
            }
        }
        ans.len()
    };

    {
        // part 1
        out.print_line(go(0, 0, 0));
    }

    {
        // part 2
        let mut ans = None;
        for i in 0..n {
            ans.maxim(go(i, 0, 0));
            ans.maxim(go(i, m - 1, 2));
        }
        for i in 0..m {
            ans.maxim(go(0, i, 1));
            ans.maxim(go(n - 1, i, 3));
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
