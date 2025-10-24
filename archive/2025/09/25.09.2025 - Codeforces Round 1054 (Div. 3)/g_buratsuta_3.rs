//{"name":"G. Buratsuta 3","group":"Codeforces - Codeforces Round 1054 (Div. 3)","url":"https://codeforces.com/contest/2149/problem/G","interactive":false,"timeLimit":4500,"tests":[{"input":"5\n1 1\n5\n1 1\n4 2\n1 1 2 3\n1 4\n2 3\n6 3\n7 7 7 8 8 9\n1 6\n2 5\n4 6\n8 2\n4 4 4 5 5 5 6 6\n1 8\n3 6\n10 5\n1 2 3 3 3 4 4 4 4 5\n1 10\n1 5\n4 9\n6 9\n7 10\n","output":"5\n1\n1 2\n7\n7 8\n8\n4 5\n5\n4\n3\n4\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    #[derive(Default)]
    struct Node {
        len: usize,
        candidates: Vec<i32>,
        count: DefaultHashMap<i32, usize>,
    }

    impl SegmentTreeNode for Node {
        fn join(left: &Self, right: &Self) -> Self {
            let mut candidates = left.candidates.clone();
            candidates.extend_from_slice(&right.candidates);
            candidates.sort();
            candidates.dedup();
            let mut count = left.count.clone();
            for (&k, &v) in right.count.iter() {
                count[k] += v;
            }
            let len = left.len + right.len;
            let threshold = len / 3;
            let mut good = Vec::new();
            for c in candidates {
                if count[c] > threshold {
                    good.push(c);
                }
            }
            Self {
                len,
                candidates: good,
                count,
            }
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| {
        let mut count = DefaultHashMap::new(0);
        count[a[i]] = 1;
        Node {
            len: 1,
            candidates: vec![a[i]],
            count,
        }
    });

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let mut candidates = Vec::new();
        st.for_each(l..r, |_, node| {
            candidates.extend_from_slice(&node.candidates);
        });
        candidates.sort();
        candidates.dedup();
        let threshold = (r - l) / 3;
        let mut ans = Vec::new();
        for c in candidates {
            let mut count = 0;
            st.for_each(l..r, |_, node| {
                count += node.count[c];
            });
            if count > threshold {
                ans.push(c);
            }
        }
        if ans.is_empty() {
            ans.push(-1);
        }
        out.print_line(ans);
    }
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
