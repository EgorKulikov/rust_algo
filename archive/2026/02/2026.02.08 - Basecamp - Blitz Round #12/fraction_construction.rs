//{"name":"Fraction Construction","group":"Eolymp - Basecamp - Blitz Round #12","url":"https://eolymp.com/en/compete/qjgameovtl0l7dprd0o9efta48/problem/5","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 1\n4 2\n1 3\n","output":"3\n1 1 1\n5\n1 1 1 2 2\n3\n1 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_size();
    let y = input.read_size();

    for n in 2.. {
        if n * (n - 1) / 2 >= x {
            let delta = n * (n - 1) / 2 - x;
            assert!(delta <= n * n);
            if y >= 3 {
                out.print_line(n);
                let mut ans = Vec::with_capacity(n);
                ans.push(1);
                for _ in 0..delta {
                    ans.push(2);
                }
                for _ in delta..(n - 1) {
                    ans.push(3);
                }
                out.print_line(ans);
                return;
            }
            if delta == 0 {
                out.print_line(n);
                out.print_line(vec![1; n]);
                return;
            }
            assert_eq!(y, 2);
            let d = delta.divisors();
            for i in d.copy_iter() {
                let q = delta / i as usize;
                if q + i as usize == n {
                    out.print_line(n);
                    let mut ans = Vec::with_capacity(n);
                    for _ in 0..q {
                        ans.push(1);
                    }
                    for _ in q..n {
                        ans.push(2);
                    }
                    out.print_line(ans);
                    return;
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
