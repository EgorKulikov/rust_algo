//{"name":"Painting","group":"CSES - Datatähti 2026 alku","url":"https://cses.fi/637/task/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4 4 4\nR 1 1\nC 3 4\nR 2 2\nR 1 1\n","output":"4 4 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_size();
    let mut m = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let queries = input.read_vec::<(u8, usize, usize)>(q);

    let mut row = BitSet::new(n);
    let mut col = BitSet::new(m);
    let mut ans = vec![0; k];
    for (t, i, c) in queries.iter_rev() {
        if t == b'R' {
            if !row[i - 1] {
                row.set(i - 1);
                ans[c - 1] += m;
                n -= 1;
            }
        } else {
            if !col[i - 1] {
                col.set(i - 1);
                ans[c - 1] += n;
                m -= 1;
            }
        }
    }
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
