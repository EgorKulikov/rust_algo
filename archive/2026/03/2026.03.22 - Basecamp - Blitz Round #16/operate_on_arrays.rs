//{"name":"Operate on Arrays","group":"Eolymp - Basecamp - Blitz Round #16","url":"https://eolymp.com/en/compete/tjhj0o6ob54rp5sgim74gibrrg/problem/5","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 1\n2 3 5\n3 5\n1 1 1\n2 10\n12 18\n4 10\n8 9 1 7\n3 2\n36 1 30\n5 7\n6 10 15 21 14\n","output":"4\n1\n36\n24\n18\n968\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::primes::sieve::divisor_table;
use std::mem::swap;
use std::ops::Add;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let d = divisor_table(1_000_001);
    let t = input.read_size();

    for _ in 0..t {
        let n = input.read_size();
        let k = input.read_size();
        let a = input.read_size_vec(n);

        type Mod = ModIntF;
        let mut ans = vec![Mod::new(0); k + 1];
        ans[0] = Mod::new(1);
        let mut next = vec![Mod::new(0); k + 1];
        for i in 0..n {
            let mut cur = a[i];
            while cur != 1 {
                let d = d[cur];
                let mut q = 0;
                while cur % d == 0 {
                    cur /= d;
                    q += 1;
                }
                let mut add = Mod::new(0);
                for i in 0..=k {
                    add += ans[i];
                    next[i] = add;
                    if i >= q {
                        add -= ans[i - q];
                    }
                }
                swap(&mut ans, &mut next);
            }
        }
        out.print_line(ans.copy_reduce(Mod::add));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
