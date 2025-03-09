//{"name":"F. Mega Polynomial","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1 2 2 4 1\n4 2 3 3 4\n2 4 1 3 3\n2 1 5 2 4\n131296 123463 91609 133724 142208\n172458 127836 190471 141192 190476\n","output":"0\n2\n4\n5\n50599\n190477\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let d = input.read_long();
    let n = input.read_long();

    let right = c * b * n;
    if right % (a * d) == 0 {
        let nk = right / (a * d);
        let k = n - nk;
        if k >= 0 && k < n {
            let da = (a / gcd(a, c)).prime_divisors();
            let mut good = true;
            fn pw(n: i64, p: i64) -> i64 {
                if n == 0 {
                    0
                } else {
                    n / p + pw(n / p, p)
                }
            }
            for (p, q) in da {
                let qq = pw(n, p as i64) - pw(n - k, p as i64);
                if (qq as usize) < q {
                    good = false;
                    break;
                }
            }
            if good {
                out.print_line(k);
                return;
            }
        }
    }
    out.print_line(n + 1);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
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
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
