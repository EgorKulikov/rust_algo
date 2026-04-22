//{"name":"E - Organizing Books","group":"AtCoder - AtCoder Weekday Contest 0051 Beta","url":"https://atcoder.jp/contests/awc0051/tasks/awc0051_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n3\n1\n4\n2\n","output":"3\n"},{"input":"3 5\n2\n3\n1\n2\n1\n","output":"6\n"},{"input":"10 8\n5\n3\n8\n1\n7\n2\n6\n4\n","output":"15\n"},{"input":"100 12\n10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n1\n1\n","output":"63\n"},{"input":"1 1\n1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
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
    let g = input.read_size_vec(m).dec();

    let mut ft = FenwickTree::new(n);
    let mut ans = 0;
    for i in (0..m).rev() {
        ans += ft.get(..g[i]);
        ft.add(g[i], 1i64);
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
