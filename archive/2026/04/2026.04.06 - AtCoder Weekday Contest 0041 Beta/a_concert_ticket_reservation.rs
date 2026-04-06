//{"name":"A - Concert Ticket Reservation","group":"AtCoder - AtCoder Weekday Contest 0041 Beta","url":"https://atcoder.jp/contests/awc0041/tasks/awc0041_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2 5\n500 2\n1000 1\n1\n1\n1\n2\n2\n","output":"2000\n"},{"input":"3 8\n300 3\n700 2\n1500 1\n1\n2\n3\n1\n2\n1\n3\n2\n","output":"3800\n"},{"input":"5 12\n1000 2\n2500 3\n800 1\n5000 2\n1200 4\n1\n2\n3\n4\n5\n1\n2\n2\n3\n4\n5\n5\n","output":"23900\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::qty::Qty;
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
    let ck = input.read_size_pair_vec(n);
    let p = input.read_size_vec(m).dec();

    let mut ans = 0;
    let q = p.qty_bound(n);
    for i in 0..n {
        let (c, k) = ck[i];
        ans += k.min(q[i]) * c;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
