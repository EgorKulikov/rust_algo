//{"name":"Magic Matrix","group":"DMOJ - Waterloo 2025 Fall C","url":"https://dmoj.ca/problem/waterloo2025fc","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2 4\n3 2\n","output":"YES\n2 2 1 1 2\n2 1 1 1 1\n1 1 2 1 1\n2 1 2 1 2\n2 2 1 2 2\n2 1 1 2 2\n"},{"input":"3\n2 2 2\n2 2 2\n","output":"YES\n3 2 2 1 2 2\n3 1 2 1 1 1\n2 1 2 2 3 2\n1 1 1 1 3 2\n3 3 2 1 3 3\n3 1 3 3 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let r = input.read_size_vec(k);
    let c = input.read_size_vec(k);

    let n = r.copy_sum();
    let m = c.copy_sum();
    let mut id_row = Vec::with_capacity(n);
    for (i, j) in r.iter_enumerate() {
        for _ in 0..j {
            id_row.push(i + 1);
        }
    }
    let mut id_col = Vec::with_capacity(m);
    for (i, j) in c.iter_enumerate() {
        for _ in 0..j {
            id_col.push(i + 1);
        }
    }
    let ans = Arr2d::with_gen(n, m, |i, j| {
        if (i + j) % 2 == 0 {
            id_row[i]
        } else {
            id_col[j]
        }
    });
    out.print_line(true);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
