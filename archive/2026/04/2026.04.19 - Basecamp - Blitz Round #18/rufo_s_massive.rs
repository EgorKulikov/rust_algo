//{"name":"Rufo's massive","group":"Eolymp - Basecamp - Blitz Round #18","url":"https://eolymp.com/en/compete/r3hr8hsui169n3libfqfgmb0no/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10 2 15\n40 5 60\n4\n1 2 3 4\n2 2 3 4\n2\n5 5\n10 10\n1\n5\n7\n","output":"5\n2\n10\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut options = vec![b[0]];
    if b[0] % a[0] == 0 {
        options.push(b[0] / a[0] + 1);
    }
    for i in 0..n {
        let mut next = Vec::new();
        for x in options {
            if a[i] > x && a[i] * (x - 1) > x {
                if b[i] == a[i] * (x - 1) {
                    next.push(x);
                }
            } else if b[i] == x {
                next.push(x);
            }
        }
        options = next;
    }
    out.print_line(options.get(0));
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
