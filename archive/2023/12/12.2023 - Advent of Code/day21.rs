//{"name":"day21","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day21"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let steps1 = input.read_long();
    let steps2 = input.read_long();
    let lines = input.read_lines();
    let n = lines.len();
    let m = lines[0].len();

    let calc_dist = |start: &[(usize, usize, i64)]| -> Arr2d<Option<i64>> {
        let mut dist = Arr2d::new(n, m, None);
        let mut queue = BinaryHeap::new();
        for &(r, c, d) in start {
            dist[r][c] = Some(d);
            queue.push((Reverse(d), r, c));
        }
        while let Some((Reverse(cur), r, c)) = queue.pop() {
            for (r, c) in D4::iter(r, c, n, m) {
                if lines[r][c] != b'#' && dist[r][c].is_none() {
                    dist[r][c] = Some(cur + 1);
                    queue.push((Reverse(cur + 1), r, c));
                }
            }
        }
        dist
    };

    let mut r = 0;
    let mut c = 0;
    for i in lines.indices() {
        for j in lines[0].indices() {
            if lines[i][j] == b'S' {
                r = i;
                c = j;
            }
        }
    }
    let dist = calc_dist(&[(r, c, 0)]);

    {
        // part 1
        let mut ans = 0;
        for &d in &dist {
            if d.is_some() && d.unwrap() % 2 == steps1 % 2 && d.unwrap() <= steps1 {
                ans += 1;
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut ans = 0i64;
        for &d in &dist {
            if d.is_some() && d.unwrap() % 2 == steps2 % 2 {
                ans += 1;
            }
        }
        for (fr, tr, fc, tc, dr, dc) in [
            (0, 0, 0, m - 1, n - 1, 0),
            (n - 1, n - 1, 0, m - 1, 1, 0),
            (0, n - 1, 0, 0, 0, m - 1),
            (0, n - 1, m - 1, m - 1, 0, 1),
        ] {
            let mut start = Vec::new();
            for i in fr..=tr {
                for j in fc..=tc {
                    let d = dist[(i, j)].unwrap();
                    start.push(((i + dr) % n, (j + dc) % m, d + 1));
                }
            }
            let mut d = calc_dist(&start);
            for _ in 0..4 {
                for &d in &d {
                    if d.is_some() && d.unwrap() % 2 == steps2 % 2 {
                        ans += 1;
                    }
                }
                let mut start = Vec::new();
                for i in fr..=tr {
                    for j in fc..=tc {
                        let d = d[(i, j)].unwrap();
                        start.push(((i + dr) % n, (j + dc) % m, d + 1));
                    }
                }
                d = calc_dist(&start);
            }
            let delta = n as i64;
            for &d in &d {
                if let Some(mut d) = d {
                    if delta % 2 == 0 {
                        if d % 2 == steps2 % 2 {
                            ans += (steps2 - d) / delta + 1;
                        }
                    } else {
                        if d % 2 != steps2 % 2 {
                            d += delta;
                        }
                        ans += (steps2 - d) / (2 * delta) + 1;
                    }
                }
            }
        }
        for (sr, sc, tr, tc) in [
            (n - 1, m - 1, 0, 0),
            (0, 0, n - 1, m - 1),
            (n - 1, 0, 0, m - 1),
            (0, m - 1, n - 1, 0),
        ] {
            let base = dist[(sr, sc)].unwrap();
            let d = calc_dist(&[(tr, tc, base + 2)]);
            let delta = n as i64;
            for &d in &d {
                if let Some(mut d) = d {
                    if delta % 2 == 0 {
                        if d % 2 == steps2 % 2 {
                            let x = (steps2 - d) / delta + 1;
                            ans += x * (x + 1) / 2;
                        }
                    } else {
                        if d % 2 != steps2 % 2 {
                            d += delta;
                            let x = (steps2 - d) / (2 * delta) + 1;
                            ans += x * (x + 1);
                        } else {
                            let x = (steps2 - d) / (2 * delta) + 1;
                            ans += x * x;
                        }
                    }
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
