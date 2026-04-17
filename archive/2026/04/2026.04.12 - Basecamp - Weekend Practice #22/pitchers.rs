//{"name":"Pitchers","group":"Eolymp - Basecamp - Weekend Practice #22","url":"https://eolymp.com/en/compete/rast2dle4l13n0vnc70f076fao/problem/5","interactive":false,"timeLimit":6000,"tests":[{"input":"5 3\n2 5 3 10 2\n1 3 20\n0 4 12\n2 3 100\n","output":"26\n34\n101\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    let pos = by_index(&a);
    let divisors = all_divisors(500_001, false);

    for _ in 0..q {
        let l = input.read_size();
        let r = input.read_size() + 1;
        let k = input.read_size();

        let mut res = (r - l) * k;
        for d in divisors[k].copy_iter() {
            if d == 1 {
                continue;
            }
            if pos[d].is_empty() {
                continue;
            }
            let q1 = pos[d].less(&r);
            if q1 == 0 {
                continue;
            }
            let q = q1 - pos[d].less(&l);
            if q == 0 {
                continue;
            }
            let mut delta = k;
            let mut cur = k;
            while cur % d == 0 {
                cur /= d;
            }
            delta -= cur;
            res -= delta * q;
        }
        out.print_line(res);
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
