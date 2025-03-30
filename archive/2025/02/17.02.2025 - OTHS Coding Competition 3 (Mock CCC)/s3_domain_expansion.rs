//{"name":"S3 - Domain Expansion","group":"DMOJ - OTHS Coding Competition 3 (Mock CCC)","url":"https://dmoj.ca/problem/othscc3p3","interactive":false,"timeLimit":3000,"tests":[{"input":"2 5\n1 3 1\n3 5 2\n","output":"2 3\n"},{"input":"4 10\n1 5 1\n1 1 100\n3 4 2\n7 10 3\n","output":"2 1 2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _k = input.read_size();
    let lrs = input
        .read_vec::<(usize, usize, i32)>(n)
        .iter_enumerate()
        .map(|(i, (l, r, s))| (l, r, s, i))
        .collect::<Vec<_>>()
        .sorted_by_key(|x| x.2);

    let (l, r, _, id) = lrs.detuple();
    let Compressed {
        order: poi,
        arrs: [l, r],
    } = compress([&l, &r.inc()]);
    #[derive(Default)]
    struct Node {
        color: Option<usize>,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            if let Some(color) = value.color {
                self.color = Some(color);
            }
        }
        fn reset_delta(&mut self) {
            self.color = None;
        }
    }
    let mut st = SegmentTree::new(poi.len() - 1);
    for i in 0..n {
        st.update(l[i]..r[i], &Node { color: Some(id[i]) });
    }
    let mut ans = vec![0; n];
    for i in 0..poi.len() - 1 {
        if let Some(color) = st.point_query(i).color {
            ans[color] += poi[i + 1] - poi[i];
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
