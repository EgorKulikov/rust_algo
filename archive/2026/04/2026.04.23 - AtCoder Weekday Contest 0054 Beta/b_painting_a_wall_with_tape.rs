//{"name":"B - Painting a Wall with Tape","group":"AtCoder - AtCoder Weekday Contest 0054 Beta","url":"https://atcoder.jp/contests/awc0054/tasks/awc0054_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 10\n1 3\n3 2\n6 3\n","output":"7\n"},{"input":"4 12\n0 2\n2 3\n5 1\n9 3\n","output":"9\n"},{"input":"8 30\n0 4\n3 5\n10 6\n14 4\n18 3\n22 5\n25 3\n29 1\n","output":"26\n"},{"input":"15 100\n0 10\n5 20\n18 7\n30 15\n32 8\n40 12\n55 10\n60 5\n62 20\n75 10\n80 15\n10 5\n27 3\n90 10\n50 2\n","output":"95\n"},{"input":"1 1\n0 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _w = input.read_int();
    let lw = input.read_int_pair_vec(n).sorted();

    let mut end = 0;
    let mut ans = 0;
    for (l, w) in lw {
        end.maxim(l);
        ans += (l + w - end).max(0);
        end.maxim(l + w);
    }
    out.print_line(ans);
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
