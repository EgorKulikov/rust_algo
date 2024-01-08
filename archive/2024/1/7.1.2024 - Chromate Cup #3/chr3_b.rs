//{"name":"chr3_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::sieve::primes;

type PreCalc = Vec<usize>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &PreCalc) {
    let n = input.read_size();

    out.print_line(data[n - 1]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::with_capacity(3000);
    let p = primes::<usize>(320000);

    for i in 0..3000 {
        pre_calc.push(p[p[i] - 1]);
    }

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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
