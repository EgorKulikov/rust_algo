//{"name":"B - Spread of Rumors","group":"AtCoder - AtCoder Weekday Contest 0024 Beta","url":"https://atcoder.jp/contests/awc0024/tasks/awc0024_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 1\n1 2\n2 3\n4 5\n","output":"3\n"},{"input":"8 5 2\n3 5\n2 4\n4 6\n5 6\n7 8\n","output":"5\n"},{"input":"10 8 3\n1 4\n4 7\n7 10\n5 6\n2 5\n5 6\n8 9\n6 8\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let ab = input.read_size_pair_vec(m).dec();

    let mut know = BitSet::new(n);
    for i in 0..k {
        know.set(i);
    }
    for (a, b) in ab {
        if know[a] {
            know.set(b);
        }
        if know[b] {
            know.set(a);
        }
    }
    out.print_line(know.count_ones());
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
