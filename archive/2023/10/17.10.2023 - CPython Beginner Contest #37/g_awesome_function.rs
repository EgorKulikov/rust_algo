//{"name":"G. Awesome function","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/G","interactive":false,"timeLimit":500,"tests":[{"input":"1 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAwesomeFunction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_utils::factorial;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();
    let k = input.read_size();

    for i in 0..k {
        if n > 12 {
            if i < k - 1 {
                out.print_line(0);
                return;
            }
            let mut res = ModInt7::new(1);
            if n == 479001600 {
                out.print_line(997981345);
                return;
            }

            for j in 2..=n {
                res *= ModInt7::from_index(j);
            }
            out.print_line(res);
            return;
        }
        n = factorial(n);
    }
    out.print_line(n);
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
