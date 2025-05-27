//{"name":"Bob's Sequence Problem","group":"DMOJ","url":"https://dmoj.ca/problem/oly22practice21","interactive":false,"timeLimit":1000,"tests":[{"input":"4 5\n1 2 3 5\n1 2\n2 3\n2 1\n3 4\n4 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut min = a.clone();
    let mut max = a.clone();

    for _ in 0..m {
        let i = input.read_size() - 1;
        let x = input.read_size() - 1;
        min[i].minim(x);
        max[i].maxim(x);
    }

    #[derive(Default)]
    struct STNode {
        map: BTreeMap<usize, usize>,
    }

    impl SegmentTreeNode for STNode {}

    let mut st = SegmentTree::<STNode>::new(100_000);
    let mut ans = 0;
    for i in 0..n {
        let cur = st.for_each_mut::<usize>(..=min[i], |mut res, node| {
            if let Some((_, &val)) = node.map.floor(&a[i]) {
                res.maxim(val);
            }
            res
        });
        ans.maxim(cur + 1);
        st.point_through_update(a[i], |node| {
            if let Some((_, &val)) = node.map.floor(&max[i]) {
                if val >= cur + 1 {
                    return;
                }
            }
            while let Some((&key, &val)) = node.map.next(&max[i]) {
                if val <= cur + 1 {
                    node.map.remove(&key);
                } else {
                    break;
                }
            }
            node.map.insert(max[i], cur + 1);
        });
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
