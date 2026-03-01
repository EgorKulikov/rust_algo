//{"name":"E - Count the Types of Flowers","group":"AtCoder - AtCoder Weekday Contest 0015 Beta","url":"https://atcoder.jp/contests/awc0015/tasks/awc0015_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 1 3 2\n1 3\n2 5\n1 5\n","output":"2\n3\n3\n"},{"input":"10 5\n3 1 4 1 5 9 2 6 5 3\n1 5\n3 7\n1 10\n5 5\n2 8\n","output":"4\n5\n7\n1\n6\n"},{"input":"20 8\n5 3 5 2 1 4 3 2 1 5 4 3 2 1 5 4 3 2 1 4\n1 20\n1 1\n10 15\n5 10\n1 10\n11 20\n7 14\n3 8\n","output":"5\n1\n5\n5\n5\n5\n5\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let p = input.read_size_vec(n).dec();
    let lr = input.read_size_pair_vec(q).dec();

    #[derive(Default)]
    struct Node {
        qty: MultiTreapSet<usize>,
    }
    impl SegmentTreeNode for Node {}

    let mut ans = vec![0; q];
    let mut queries = vec![Vec::new(); n];
    for (i, (l, r)) in lr.iter_enumerate() {
        queries[r].push((l, i));
    }
    let mut st = SegmentTree::<Node>::new(n);
    let mut prev = vec![0; n];
    for i in 0..n {
        st.point_through_update(i, |node| {
            node.qty.insert(prev[p[i]]);
        });
        prev[p[i]] = i + 1;
        for (l, j) in queries[i].drain(..) {
            st.for_each_mut(l..=i, |_, node| {
                ans[j] += node.qty.less_or_eq(&l);
            });
        }
    }
    out.print_per_line(&ans);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
