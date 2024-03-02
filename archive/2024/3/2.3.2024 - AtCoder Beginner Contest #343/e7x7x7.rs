//{"name":"E - 7x7x7","group":"AtCoder - AtCoder Beginner Contest 343","url":"https://atcoder.jp/contests/abc343/tasks/abc343_e","interactive":false,"timeLimit":2000,"tests":[{"input":"840 84 7\n","output":"Yes\n0 0 0 0 6 0 6 0 0\n"},{"input":"343 34 3\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E7x7x7"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let v1 = input.read_int();
    let v2 = input.read_int();
    let v3 = input.read_int();

    out.set_bool_output(BoolOutput::YesNo);

    if 3 * v3 + 2 * v2 + v1 != 3 * 7 * 7 * 7 {
        out.print_line(false);
        return;
    }

    let x1 = 0;
    let y1 = 0;
    let z1 = 0;
    for x2 in 0..=7 {
        for y2 in 0..=7 {
            for z2 in 0..=7 {
                for x3 in 0..=14 {
                    for y3 in 0..=14 {
                        for z3 in -7..=14 {
                            let cv3 = (x1.min(x2).min(x3) + 7 - x1.max(x2).max(x3)).max(0)
                                * (y1.min(y2).min(y3) + 7 - y1.max(y2).max(y3)).max(0)
                                * (z1.min(z2).min(z3) + 7 - z1.max(z2).max(z3)).max(0);
                            if v3 != cv3 {
                                continue;
                            }
                            let cv2 = (x1.min(x2) + 7 - x1.max(x2)).max(0)
                                * (y1.min(y2) + 7 - y1.max(y2)).max(0)
                                * (z1.min(z2) + 7 - z1.max(z2)).max(0)
                                + (x1.min(x3) + 7 - x1.max(x3)).max(0)
                                    * (y1.min(y3) + 7 - y1.max(y3)).max(0)
                                    * (z1.min(z3) + 7 - z1.max(z3)).max(0)
                                + (x2.min(x3) + 7 - x2.max(x3)).max(0)
                                    * (y2.min(y3) + 7 - y2.max(y3)).max(0)
                                    * (z2.min(z3) + 7 - z2.max(z3)).max(0)
                                - 3 * cv3;
                            if v2 != cv2 {
                                continue;
                            }
                            out.print_line(true);
                            out.print_line((x1, y1, z1, x2, y2, z2, x3, y3, z3));
                            return;
                        }
                    }
                }
            }
        }
    }
    out.print_line(false);
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
