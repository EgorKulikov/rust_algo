//{"name":"day9","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day9"}}}

use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = Vec::new();
    while !input.is_exhausted() {
        let l = input.read_line();
        s.push(l.parse_vec::<i64>());
    }

    {
        // part 1
        let mut ans = 0;
        for s in &s {
            let mut s = s.clone();
            let mut deg = 0;
            while s[deg..].iter().any(|&x| x != 0) {
                deg += 1;
                for i in (deg..s.len()).rev() {
                    s[i] -= s[i - 1];
                }
            }
            s.push(0);
            for deg in (1..=deg).rev() {
                for i in deg..s.len() {
                    s[i] += s[i - 1];
                }
            }
            ans += s.pop().unwrap();
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut ans = 0;
        for s in &s {
            let mut s = s.clone();
            s.reverse();
            let mut deg = 0;
            while s[deg..].iter().any(|&x| x != 0) {
                deg += 1;
                for i in (deg..s.len()).rev() {
                    s[i] -= s[i - 1];
                }
            }
            s.push(0);
            for deg in (1..=deg).rev() {
                for i in deg..s.len() {
                    s[i] += s[i - 1];
                }
            }
            ans += s.pop().unwrap();
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
