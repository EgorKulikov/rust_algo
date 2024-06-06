//{"name":"Non-divisor chain length","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/non-divisor-chain-length","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1\n2\n3\n12\n121\n","output":"1\n1\n2\n5\n61\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NonDivisorChainLength"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};

type PreCalc = Vec<Option<usize>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();

    let mut rec = RecursiveFunction::new(|rec, n: usize| -> usize {
        if let Some(ans) = data[n] {
            ans
        } else {
            for i in 2.. {
                if n % i != 0 {
                    data[n] = Some(rec.call(n - i) + 1);
                    return data[n].unwrap();
                }
            }
            unreachable!();
        }
    });
    out.print_line(rec.call(n));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = vec![None; 100001];
    pre_calc[1] = Some(1);
    pre_calc[2] = Some(1);

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
