//{"name":"D. Circular Matching","group":"Universal Cup - The 3rd Universal Cup Semifinals","url":"https://contest.ucup.ac/contest/2506/problem/14017","interactive":false,"timeLimit":1000,"tests":[{"input":"10 3\n1101000110\n2 5\n6 9\n1 10\n","output":"2\n2\n7\n"},{"input":"29 10\n11000001110001010001100100001\n16 21\n24 25\n6 11\n7 12\n1 10\n14 21\n10 11\n1 4\n14 17\n8 21\n","output":"5\n1\n5\n5\n13\n6\n1\n2\n2\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    #[derive(Default)]
    struct Node {
        delta: Vec<i64>,
        ps: Vec<i64>,
        min: i64,
        max: i64,
    }
    impl SegmentTreeNode for Node {}
    impl Node {
        fn get(&self, at: i64) -> i64 {
            let pos = self.delta.lower_bound(&at);
            at * (pos as i64) - self.ps[pos] + (self.ps[self.delta.len()] - self.ps[pos])
                - (self.delta.len() as i64 - pos as i64) * at
        }
    }

    let mut delta = Vec::new();
    let mut cur = 0;
    for i in 0..n {
        delta.push(cur);
        if s[i] == b'1' {
            cur += 1;
        } else {
            cur -= 1;
        }
    }

    let mut st = SegmentTree::with_gen_full(n, |l, r| {
        let delta = delta[l..r].to_vec().sorted();
        let ps = delta.partial_sums();
        let min = delta.copy_min();
        let max = delta.copy_max();
        Node {
            delta,
            ps,
            min,
            max,
        }
    });

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let mut left = st.for_each_with_init(l..r, |val, node| val.min(node.min), i64::MAX);
        let mut right = st.for_each_with_init(l..r, |val, node| val.max(node.max), i64::MIN);
        let mut calc = |at: i64| -> i64 { st.for_each(l..r, |val, node| val + node.get(at)) };
        while left < right {
            let mid = left + (right - left) / 2;
            let delta = calc(mid + 1) - calc(mid);
            if delta > 0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        out.print_line(calc(left));
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
