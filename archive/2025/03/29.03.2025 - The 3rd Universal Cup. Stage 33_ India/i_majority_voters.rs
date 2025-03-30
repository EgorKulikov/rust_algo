//{"name":"I. Majority Voters","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10273","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 4\nAABA\n1 4\n2 3\n3 3\n3 4\n5 2\nBBBBA\n1 5\n5 5\n","output":"0\n1\n-1\n1\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let next_a = Vec::with_gen_back(n + 1, |i, next_a| {
        if i == n {
            n
        } else {
            if s[i] == b'A' {
                i
            } else {
                next_a[i + 1]
            }
        }
    });

    #[derive(Default, Copy, Clone)]
    struct Node {
        max: i32,
        max_at: usize,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            if left_val.max >= right_val.max {
                *self = *left_val;
            } else {
                *self = *right_val;
            }
        }
    }

    let mut delta = vec![0];
    let mut cur = 0;
    for i in 0..n {
        if s[i] == b'A' {
            cur += 1;
        } else {
            cur -= 1;
        }
        delta.push(cur);
    }
    let mut st = SegmentTree::with_gen(n + 1, |i| Node {
        max: delta[i],
        max_at: i,
    });

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let start_delta = delta[l];
        let end_delta = delta[r];
        if end_delta > start_delta {
            out.print_line(0);
            continue;
        }
        let mut need = start_delta - end_delta + 1;
        let next_a = next_a[l];
        if next_a >= r {
            out.print_line(-1);
            continue;
        }
        let pos = st.query(next_a + 1..=r).max_at;
        let deficit = delta[pos] - start_delta;
        let mut ans = 0;
        if deficit <= 0 {
            ans += -deficit + 1;
            need -= -deficit + 1;
        }
        ans += (need + 1) / 2;
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
