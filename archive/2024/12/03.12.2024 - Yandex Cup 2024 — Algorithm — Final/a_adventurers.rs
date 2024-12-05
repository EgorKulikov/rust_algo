//{"name":"A. Adventurers","group":"Yandex - Yandex Cup 2024 — Algorithm — Final","url":"https://contest.yandex.com/contest/72057/problems/A/","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1 1\n1 2\n2 1\n2 2\n","output":"1\n2 2\n"},{"input":"4\n0 0\n0 0\n0 0\n0 0\n","output":"0\n0 0\n"},{"input":"8\n1 2\n2 1\n2 -1\n1 -2\n-1 -2\n-2 -1\n-2 1\n-1 2\n","output":"2\n1 1\n"},{"input":"7\n1 1\n1 2\n1 3\n1 4\n2 1\n3 1\n4 1\n","output":"0\n4 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAdventurers"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_int_pair_vec(n).sorted();

    let (x, y) = pts.detuple();
    let Compressed { arrs: [y], order } = compress([&y]);
    let len = order.len();

    #[derive(Default, Copy, Clone)]
    struct Node {
        up: usize,
        down: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Node::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.up = left_val.up + right_val.up;
            self.down = left_val.down + right_val.down;
        }
    }

    impl Pushable<()> for Node {
        fn push(&mut self, _delta: ()) {
            self.down -= 1;
            self.up += 1;
        }
    }

    let qty = y.qty_bound(len);

    let mut st = SegmentTree::from_generator(len, |i| Node {
        down: qty[i],
        up: 0,
    });

    let mut last = i32::MIN;
    let mut ans = None;
    for i in 0..n {
        if x[i] != last {
            let mut left_up = 0;
            let mut right_up = 0;
            let mut left_down = 0;
            let mut right_down = 0;
            let pos = st.binary_search(
                |l, r| {
                    let cur_left = (left_up + l.up).min(left_down + l.down);
                    let cur_right = (right_up + r.up).min(right_down + r.down);
                    if cur_left <= cur_right {
                        left_up += l.up;
                        left_down += l.down;
                        Direction::Right
                    } else {
                        right_up += r.up;
                        right_down += r.down;
                        Direction::Left
                    }
                },
                |_, pos| pos,
            );
            let left = st.query(..pos);
            ans.maxim((left.up.min(left.down), x[i], order[pos]));
            let right = st.query(pos + 1..);
            ans.maxim((right.up.min(right.down), x[i], order[pos] + 1));
        }
        st.point_update(y[i], ());
        last = x[i];
    }
    let (ans, x, y) = ans.unwrap();
    out.print_line(ans);
    out.print_line((x, y));
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
