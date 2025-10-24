//{"name":"Trading","group":"Eolymp - Basecamp - Educational Round #2","url":"https://basecamp.eolymp.com/en/compete/b0f0h1n10h679euo6gn2c7821o/problem/10","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n1 3 2\n2 4 6\n","output":"2 6 7 8 0\n"},{"input":"6 4\n4 4 3\n1 2 5\n5 6 1\n6 6 1\n","output":"5 6 0 3 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let lrx: Vec<(usize, usize, usize)> = input.read_vec(m);

    #[derive(Default)]
    struct Node {
        val: Option<i64>,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            if let Some(v) = value.val {
                self.val.maxim(v);
            }
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    for (l, r, x) in lrx {
        st.update(
            l - 1..r,
            &Node {
                val: Some(x as i64 - (l - 1) as i64),
            },
        );
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(st.point_query(i).val.map(|x| x + i as i64).unwrap_or(0));
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
