//{"name":"L. Not-Incomplete","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/L","interactive":false,"timeLimit":1000,"tests":[{"input":"14 3 10 15\n","output":"Yes\n2 3 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LNotIncomplete"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x = input.read_int();
    let y = input.read_int();
    let z = input.read_int();
    let n = input.read_int();

    out.set_bool_output(BoolOutput::YesNo);
    let mut need = ((y * x * 6).upper_div(10) - n).max(0);
    if need > (x - z) * y {
        out.print_line(false);
    } else {
        out.print_line(true);
        let mut ans = Vec::with_capacity((x - z) as usize);
        for _ in 0..(x - z) {
            let cur = need.min(y);
            ans.push(cur);
            need -= cur;
        }
        ans.reverse();
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
