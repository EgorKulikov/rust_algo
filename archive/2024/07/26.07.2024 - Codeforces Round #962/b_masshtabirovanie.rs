//{"name":"B. Масштабирование","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n0000\n0000\n0000\n0000\n6 3\n000111\n000111\n000111\n111000\n111000\n111000\n6 2\n001100\n001100\n111111\n111111\n110000\n110000\n8 1\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n","output":"0\n01\n10\n010\n111\n100\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMasshtabirovanie"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_char_table(n, n);

    let ans = Arr2d::generate(n / k, n / k, |i, j| s[(i * k, j * k)]);
    out.print_table(&ans);
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
}
//END MAIN
