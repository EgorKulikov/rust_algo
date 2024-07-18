//{"name":"ucup20_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup20_e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let r = input.read_long();

    let mut xs = Vec::new();
    let sqrt = r.lower_sqrt();
    for x in 0..=sqrt {
        if (r - x * x).sqrt().is_some() {
            xs.push(x);
        }
    }
    if xs.is_empty() {
        out.print_line("inf");
        return;
    }
    let g = xs.iter().fold(0, |acc, &x| gcd(acc, x));
    let all_odd = xs.into_iter().all(|x| x / g % 2 == 1);
    if all_odd {
        out.print_line(2 * g * g);
    } else {
        out.print_line(g * g);
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
    let test_type = TestType::MultiNumber;
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
    tester::stress_test();
}
//END MAIN
