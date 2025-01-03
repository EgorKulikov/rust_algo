//{"name":"coderun_10","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_10"}}}

use algo_lib::collections::segment_tree::{QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::value_ref;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut pos = vec![Vec::new(); n];
    for i in 0..n {
        pos[a[i]].push(i);
    }
    value_ref!(Pos: Vec<Vec<usize>> = pos);
    struct Node {
        mode: Option<usize>,
        left: usize,
        right: usize,
    }
    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                mode: None,
                left,
                right,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            for opt in &[left_val.mode, right_val.mode] {
                if let &Some(mode) = opt {
                    let qty = Pos::val()[mode].inside(&self.left..&self.right);
                    if 2 * qty > self.right - self.left {
                        self.mode = Some(mode);
                        return;
                    }
                }
            }
        }
    }
    impl QueryResult<Option<usize>, (usize, usize)> for Node {
        fn empty_result(_args: &(usize, usize)) -> Option<usize> {
            None
        }

        fn result(&self, args: &(usize, usize)) -> Option<usize> {
            if let Some(mode) = self.mode {
                let (left, right) = *args;
                let qty = Pos::val()[mode].inside(&left..&right);
                if 2 * qty > right - left {
                    return Some(mode + 1);
                }
            }
            None
        }

        fn join_results(
            left_res: Option<usize>,
            right_res: Option<usize>,
            _args: &(usize, usize),
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> Option<usize> {
            if left_res.is_some() {
                return left_res;
            }
            right_res
        }
    }
    let mut st = SegmentTree::gen(n, |i| Node {
        mode: Some(a[i]),
        left: i,
        right: i + 1,
    });

    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        out.print_line(st.query_with_args(l..r, &(l, r)).unwrap_or(0));
    }
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
