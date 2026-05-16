//{"name":"A - Comfortable Temperature","group":"AtCoder - AtCoder Weekday Contest 0068 Beta","url":"https://atcoder.jp/contests/awc0068/tasks/awc0068_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 20 25\n18\n20\n22\n25\n30\n","output":"3\n"},{"input":"4 10 12\n9\n13\n0\n100\n","output":"0\n"},{"input":"10 -5 5\n-10\n-5\n-3\n0\n4\n5\n6\n10\n-1\n-6\n","output":"6\n"},{"input":"20 15 30\n12\n15\n18\n21\n24\n27\n30\n33\n14\n16\n19\n22\n25\n28\n31\n29\n17\n13\n30\n20\n","output":"15\n"},{"input":"1 -100 -100\n-100\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::io::output::BoolOutput;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_int();
    let r = input.read_int();
    let t = input.read_int_vec(n);

    out.print_line(t.copy_filter(|&x| l <= x && x <= r).count());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
