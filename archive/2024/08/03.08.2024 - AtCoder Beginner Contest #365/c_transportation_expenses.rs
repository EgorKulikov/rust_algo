//{"name":"C - Transportation Expenses","group":"AtCoder - Toyota Programming Contest 2024#8（AtCoder Beginner Contest 365）","url":"https://atcoder.jp/contests/abc365/tasks/abc365_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 8\n1 3 2 4\n","output":"2\n"},{"input":"3 20\n5 3 2\n","output":"infinite\n"},{"input":"10 23\n2 5 6 5 2 1 7 9 7 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTransportationExpenses"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut m = input.read_size();
    let a = input.read_size_vec(n).sorted();

    for i in 0..n {
        if a[i] > m / (n - i) {
            out.print_line(m / (n - i));
            return;
        }
        m -= a[i];
    }
    out.print_line("infinite");
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
}
//END MAIN
