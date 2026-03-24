//{"name":"C - Observatory with a Mountain View","group":"AtCoder - AtCoder Weekday Contest 0032 Beta","url":"https://atcoder.jp/contests/awc0032/tasks/awc0032_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n100 5\n200 10\n300 15\n150\n100\n","output":"25\n30\n"},{"input":"3 3\n100 3\n200 7\n300 11\n500\n300\n1\n","output":"0\n11\n21\n"},{"input":"8 5\n50 2\n120 8\n300 15\n500 20\n700 25\n1000 30\n1500 50\n2000 100\n600\n1000\n2000\n2001\n1\n","output":"205\n180\n100\n0\n250\n"},{"input":"15 10\n80 12\n150 25\n300 18\n450 30\n600 45\n800 55\n1200 60\n1800 70\n2500 85\n3000 100\n4000 120\n5500 200\n7000 150\n8500 300\n10000 500\n1\n500\n1000\n2000\n3000\n5000\n7000\n9000\n10000\n10001\n","output":"1770\n1685\n1585\n1455\n1370\n1150\n950\n500\n500\n0\n"},{"input":"1 1\n1000000000 1000000000\n1000000000\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let yp = input.read_long_pair_vec(n).sorted();

    let (y, p) = yp.detuple();
    let s = p.partial_sums();

    for _ in 0..q {
        let l = input.read_long();
        let pos = y.lower_bound(&l);
        out.print_line(s[n] - s[pos]);
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
