//{"name":"A - Communicate Topological Order","group":"AtCoder - AtCoder Grand Contest 074","url":"https://atcoder.jp/contests/agc074/tasks/agc074_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 4\n3 4\n4 1\n3 1\n5 2\n5 4 1 2 3\n4 0\n4 2 1 3\n10 15\n6 2\n8 5\n4 9\n7 10\n3 7\n5 6\n8 9\n3 5\n5 2\n8 2\n3 9\n5 9\n10 2\n3 2\n7 4\n8 9 2 6 4 5 3 1 10 7\n","output":"2\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut edges = input.read_size_pair_vec(m).dec();
    let p = input.read_size_vec(n).dec();

    for (u, v) in edges.iter_mut() {
        *u = p[*u];
        *v = p[*v];
    }
    let mut max = vec![None; n];
    for (u, v) in edges {
        max[v].maxim(u);
    }
    let mut ans = 0;
    let mut start = 0;
    for i in 0..n {
        if let Some(mx) = max[i] {
            if mx >= start {
                ans += i - start - 1;
                start = i;
            }
        }
    }
    ans += n - start - 1;
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
