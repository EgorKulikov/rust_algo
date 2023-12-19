//{"name":"day11","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day11"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let map = input.read_lines();

    let solve = |step: i64| -> i64 {
        let mut r = vec![0];
        let mut cur = 0i64;
        for s in &map {
            if s.iter().count_eq(&b'#') == 0 {
                cur += step;
            } else {
                cur += 1;
            }
            r.push(cur);
        }
        let mut c = vec![0];
        cur = 0;
        for i in 0..map[0].len() {
            if map.iter().map(|s| s[i]).count_eq(&b'#') == 0 {
                cur += step;
            } else {
                cur += 1;
            }
            c.push(cur);
        }
        let mut stars = Vec::new();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == b'#' {
                    stars.push((i, j));
                }
            }
        }
        let mut ans = 0;
        for &(r1, c1) in &stars {
            for &(r2, c2) in &stars {
                if r1 == r2 && c1 == c2 {
                    break;
                }
                ans += (r[r1] - r[r2]).abs() + (c[c1] - c[c2]).abs();
            }
        }
        ans
    };

    {
        // part 1
        out.print_line(solve(2));
    }

    {
        // part 2
        out.print_line(solve(1000000));
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
