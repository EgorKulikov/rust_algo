//{"name":"E - Chords","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3\n4 2\n5 6\n","output":"Yes\n"},{"input":"3\n6 1\n4 3\n2 5\n","output":"No\n"},{"input":"4\n2 4\n3 7\n8 6\n5 1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EChords"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut pairs = input.read_size_pair_vec(n).dec();

    out.set_bool_output(BoolOutput::YesNo);
    for (a, b) in pairs.iter_mut() {
        if a > b {
            swap(a, b);
        }
    }
    pairs.sort();
    let mut ends = Vec::new();
    for (a, b) in pairs {
        while let Some(&last) = ends.last() {
            if last > a {
                break;
            }
            ends.pop();
        }
        if let Some(&last) = ends.last() {
            if last < b {
                out.print_line(true);
                return;
            }
        }
        ends.push(b);
    }
    out.print_line(false);
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
    //    tester::stress_test();
}
//END MAIN
