//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::numbers::number_iterator::iterate;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut mem = Memoization3d::new(18 * 9 + 1, 18, 18 * 9 + 1, |f, sum, digs, mut back| {
        if digs != 0 {
            for i in 0..10 {
                back = f.call(sum + i, digs - 1, back);
            }
            back
        } else if sum > back {
            back + 1
        } else {
            0
        }
    });

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let mut back = 0;
        let v = iterate(1, n);
        for (mut pref, digs, _) in v {
            let mut sum = 0;
            while pref > 0 {
                sum += pref % 10;
                pref /= 10;
            }
            back = mem.call(sum, digs, back);
        }
        if back == 0 {
            out.print_line("Bajtek");
        } else {
            out.print_line("Algosia");
        }
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
