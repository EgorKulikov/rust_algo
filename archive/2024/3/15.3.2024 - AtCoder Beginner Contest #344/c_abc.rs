//{"name":"C - A+B+C","group":"AtCoder - \tToyota Programming Contest 2024#3（AtCoder Beginner Contest 344）","url":"https://atcoder.jp/contests/abc344/tasks/abc344_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 3\n2\n2 4\n6\n1 2 4 8 16 32\n4\n1 5 10 50\n","output":"No\nYes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CABC"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let m = input.read_size();
    let b = input.read_int_vec(m);
    let l = input.read_size();
    let c = input.read_int_vec(l);

    let mut present = HashSet::new();
    for a in a {
        for &b in &b {
            for &c in &c {
                present.insert(a + b + c);
            }
        }
    }

    out.set_bool_output(BoolOutput::YesNo);
    let q = input.read_size();
    for _ in 0..q {
        let x = input.read_int();
        out.print_line(present.contains(&x));
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
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
    //    tester::stress_test();
}
//END MAIN
