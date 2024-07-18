//{"name":"B - Foreign Exchange","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 7 0 3\n2 2\n4 3\n5 2\n","output":"5\n"},{"input":"10\n32 6 46 9 37 8 33 14 31 5\n5 5\n3 1\n4 3\n2 2\n3 2\n3 2\n4 4\n3 3\n3 1\n","output":"45\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BForeignExchange"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);
    let st = input.read_long_pair_vec(n - 1);

    for i in 0..n - 1 {
        a[i + 1] += (a[i] / st[i].0) * st[i].1;
    }
    out.print_line(a[n - 1]);
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
