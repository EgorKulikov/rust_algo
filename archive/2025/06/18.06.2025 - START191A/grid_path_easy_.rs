//{"name":"Grid Path (Easy)","group":"CodeChef - START191A","url":"https://www.codechef.com/START191A/problems/GRIDPATHEZ","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3 0\n011\n110\n3 0\n011\n010\n6 0\n110010\n010001\n6 0\n110010\n111101\n6 0\n110010\n110101\n","output":"4\n-1\n-1\n4\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    input.read_size();
    let grid = input.read_char_table(2, n);

    let mut pos_top = Vec::new();
    for i in 0..n {
        if grid[(0, i)] == b'1' {
            pos_top.push(i - pos_top.len());
        }
    }
    let mut pos_bottom = Vec::new();
    for i in 0..n {
        if grid[(1, n - 1 - i)] == b'1' {
            pos_bottom.push(i - pos_bottom.len());
        }
    }
    let x = pos_top.partial_sums();
    let y = pos_bottom.partial_sums();
    let mut ans = None;
    for i in 1..x.len() {
        let id = n + 1 - i;
        if id < y.len() {
            ans.minim(x[i] + y[id]);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
