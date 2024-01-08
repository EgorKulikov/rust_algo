//{"name":"chr3_g","group":"Manual","url":"","interactive":true,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_g"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut base = 0i64;
    let mut from = 0i64;
    let mut to = 1000000000i64;
    let mut ans_to = to * to;

    loop {
        while from < to {
            let mid = (from + to + 1) / 2;
            out.print_line(('?', base, mid));
            let ans = input.read_char();
            match ans {
                '0' => {
                    out.print_line(('!', base + mid * mid));
                    return;
                }
                '+' => {
                    from = mid;
                }
                '-' => {
                    to = mid - 1;
                }
                _ => unreachable!(),
            }
        }
        let new_base = base + from * from;
        ans_to.minim(base + (from + 1) * (from + 1) - 1);
        if new_base == ans_to {
            out.print_line(('!', new_base));
            return;
        }
        base = new_base;
        from = 0;
        to = (ans_to - base).lower_sqrt();
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
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
