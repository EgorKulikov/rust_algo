//{"name":"Efficient Room Scheduling","group":"DMOJ - Waterloo 2025 Fall E","url":"https://dmoj.ca/problem/waterloo2025fe","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n5\n4\n7\n8\n4\n2 1\n5\n5\n10\n0 0\n","output":"11\nImpossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size_vec(n).sorted();
    let c = input.read_size_vec(m).sorted();

    if n == 0 && m == 0 {
        return;
    }

    let mut at = 0;
    let mut ans = 0;
    let mut need = 0;
    for i in c {
        while at < n && q[at] <= i {
            at += 1;
            need += 1;
        }
        if need > 0 {
            need -= 1;
            ans += i;
        }
    }
    if need > 0 {
        out.print_line("Impossible");
    } else {
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
