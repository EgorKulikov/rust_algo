//{"name":"F1. Mathematical expectation #1","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/F1","interactive":false,"timeLimit":500,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1MathematicalExpectation1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(_input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    out.print_line("1.359");
}

#[test]
fn test() {
    use algo_lib::misc::random::random;
    let mut sum = 0.;
    let mut rnd = random();
    for i in 0.. {
        for _ in 0..100000000 {
            let mut x = 0.;
            while x < 1. {
                x += (rnd.next(1000000000000000000) as f64) / 1000000000000000000.;
            }
            sum += x;
        }
        println!("{}", sum / (100000000. * (i + 1) as f64));
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
