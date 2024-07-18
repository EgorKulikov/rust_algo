//{"name":"B. Shake Selection Dilemma","group":"Codeforces - Coding Challenge Alpha V - by AlgoRave","url":"https://codeforces.com/gym/105005/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1000 4\n35 15\n50 25\n30 20\n10 1\n1 2\n10 5\n100000 2\n","output":"2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BShakeSelectionDilemma"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_long();
    let m = input.read_size();
    let shakes = input.read_long_pair_vec(m);

    let mut ans = None;
    for (i, (a, q)) in shakes.into_iter().enumerate() {
        if n >= q {
            ans.maxim(((n / q).min(a), Reverse(i + 1)));
        }
    }
    out.print_line(ans.map(|(_, i)| i.0));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
