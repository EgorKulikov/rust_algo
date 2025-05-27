//{"name":"C. 23 королевство","group":"Codeforces - Codeforces Round 1024 (Div. 1)","url":"https://codeforces.com/contest/2101/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4\n1 2 1 2\n2\n2 2\n10\n1 2 1 5 1 2 2 1 1 2\n8\n1 5 2 8 4 1 4 2\n","output":"4\n1\n16\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Default)]
    struct Node {
        all_occupied: bool,
        qty: usize,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.all_occupied = left_val.all_occupied && right_val.all_occupied;
            self.qty = left_val.qty + right_val.qty;
        }
    }

    let mut front = SegmentTree::<Node>::new(n);
    let mut back = SegmentTree::<Node>::new(n);
    fn add(st: &mut SegmentTree<Node>, i: usize) -> bool {
        st.binary_search_in_mut(
            i..,
            |node| !node.all_occupied,
            |left, _| {
                if !left.all_occupied {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            |node, _| {
                assert!(!node.all_occupied);
                node.all_occupied = true;
                node.qty = 1;
            },
        )
        .is_some()
    }
    let mut i = 0;
    let mut j = n;
    let mut ans = 0;
    while i < j {
        while i < j {
            if add(&mut front, n - a[i]) {
                break;
            }
            i += 1;
        }
        while i < j {
            if add(&mut back, n - a[j - 1]) {
                break;
            }
            j -= 1;
        }
        if i == j {
            break;
        }
        i += 1;
        ans += j - i;
        j -= 1;
    }
    out.print_line(ans);
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
