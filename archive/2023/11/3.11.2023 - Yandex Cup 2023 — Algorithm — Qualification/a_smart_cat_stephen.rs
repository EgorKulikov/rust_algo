//{"name":"A. Smart cat Stephen","group":"Yandex - Yandex Cup 2023 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/54452/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2 3 1 4\n","output":"1\n"},{"input":"5\n1 2 3 2 3\n","output":"3\n"},{"input":"5\n3 3 4 5 3\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASmartCatStephen"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut last_pos = HashMap::new();
    for (i, j) in a.into_iter().enumerate() {
        last_pos.insert(j, i);
    }
    let mut at = 0;
    let mut target = 1;
    loop {
        if let Some(&pos) = last_pos.get(&target) {
            if pos >= at {
                at = pos + 1;
                target += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    out.print_line(target - 1);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
