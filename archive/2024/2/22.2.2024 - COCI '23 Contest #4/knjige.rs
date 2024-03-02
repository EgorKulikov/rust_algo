//{"name":"#2 - Knjige","group":"DMOJ - COCI '23 Contest 4","url":"https://dmoj.ca/problem/coci23c4p2","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5 2 1\n2 2 4\n","output":"6\n"},{"input":"2 10 3 1\n3 3\n","output":"6\n"},{"input":"4 10 3 2\n3 4 5 6\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Knjige"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let t = input.read_size();
    let a = input.read_size();
    let b = input.read_size();
    let k = input.read_long_vec(n);

    let s = k.partial_sums();
    let mut ans = 0;
    for i in 1..=n {
        if a * i > t {
            break;
        }
        let can_skip = (n - i).min((t - a * i) / b);
        ans.maxim(s[i + can_skip] - s[can_skip]);
    }
    out.print_line(ans);
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
