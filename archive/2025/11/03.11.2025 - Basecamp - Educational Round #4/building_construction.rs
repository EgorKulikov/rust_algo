//{"name":"Building Construction","group":"Eolymp - Basecamp - Educational Round #4","url":"https://eolymp.com/en/compete/btktopvnh51kpfku7ua54hi0bk/problem/6","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 3 1 4\n10 20 30 40\n","output":"110\n"},{"input":"3\n1 2 3\n10 100 1000\n","output":"120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_size_vec(n);
    let c = input.read_long_vec(n);

    #[derive(Default)]
    struct Node {
        k: i64,
        b: i64,
    }

    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.k += value.k;
            self.b += value.b;
        }

        fn reset_delta(&mut self) {
            self.k = 0;
            self.b = 0;
        }
    }

    let mut st = SegmentTree::<Node>::new(10_001);
    for i in 0..n {
        let pos = h[i] as i64;
        st.update(
            h[i]..,
            &Node {
                b: -pos * c[i],
                k: c[i],
            },
        );
        st.update(
            ..h[i],
            &Node {
                b: pos * c[i],
                k: -c[i],
            },
        );
    }
    let mut ans = None;
    for i in 0..=10_000 {
        let node = st.point_query(i);
        ans.minim(node.k * (i as i64) + node.b);
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
