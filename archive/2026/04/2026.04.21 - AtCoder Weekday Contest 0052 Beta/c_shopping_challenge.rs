//{"name":"C - Shopping Challenge","group":"AtCoder - AtCoder Weekday Contest 0052 Beta","url":"https://atcoder.jp/contests/awc0052/tasks/awc0052_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n3 2\n4 3\n1 4\n","output":"7\n"},{"input":"3 10\n1 3\n2 3\n3 3\n","output":"-1\n"},{"input":"8 20\n10 5\n8 7\n6 3\n12 8\n3 4\n7 6\n9 10\n5 2\n","output":"31\n"},{"input":"15 50\n20 8\n15 12\n30 10\n25 15\n10 5\n18 7\n12 9\n22 14\n8 3\n35 20\n14 6\n9 11\n27 13\n19 16\n11 4\n","output":"124\n"},{"input":"1 7\n5 7\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_size();
    let vc = input.read_size_pair_vec(n);

    let mut ans = vec![None; s + 1];
    ans[0] = Some(0);
    for (v, c) in vc {
        if c <= s {
            for i in (c..=s).rev() {
                if let Some(prev) = ans[i - c] {
                    let new = prev + v;
                    ans[i].maxim(new);
                }
            }
        }
    }
    out.print_line(ans[s]);
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
