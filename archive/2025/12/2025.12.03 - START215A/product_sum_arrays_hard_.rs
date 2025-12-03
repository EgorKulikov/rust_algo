//{"name":"Product Sum Arrays (Hard)","group":"CodeChef - START215A","url":"https://www.codechef.com/START215A/problems/PRSUMARRHD","interactive":false,"timeLimit":2000,"tests":[{"input":"11\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n1000000\n","output":"1\n1\n1\n2\n1\n2\n1\n3\n2\n2\n1043\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();

    let d = n.divisors();
    let mut mem = Memoization2::new(|mem, max: u64, rem: u64| -> i64 {
        if rem == 1 {
            1
        } else {
            let mut res = 0;
            for i in 1..d.len() {
                if d[i] <= max && rem % d[i] == 0 {
                    res += mem.call(d[i], rem / d[i]);
                }
            }
            res
        }
    });
    out.print_line(mem.call(n as u64, n as u64));
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
