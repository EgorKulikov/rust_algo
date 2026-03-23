//{"name":"A Little to the Right","group":"Kattis","url":"https://open.kattis.com/problems/alittletotheright","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 4\n2 5\n3 4\n","output":"1\n"},{"input":"2 4\n1 3 6 2\n4 2 6 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size();
    let c = input.read_int_table(n, p);

    let mut ans = FxHashSet::default();
    for i in 0..p {
        let vec = (0..n).collect::<Vec<_>>().sorted_by_key(|&j| c[j][i]);
        let mut bad = false;
        for (a, b) in vec.consecutive_iter_copy() {
            if c[(a, i)] == c[(b, i)] {
                bad = true;
                break;
            }
        }
        if !bad {
            ans.insert(vec);
        }
    }
    out.print_line(ans.len());
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
