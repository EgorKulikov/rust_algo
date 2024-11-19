//{"name":"Buying Order (Hard)","group":"CodeChef - START160A","url":"https://www.codechef.com/START160A/problems/BUYORDERHARD","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n0 1\n2\n1 1\n","output":"1 2\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BuyingOrderHard"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_int_vec(n);

    #[derive(Clone, Default)]
    struct Node {
        delta: usize,
        ans: usize,
    }
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.ans = left_val.ans.min(right_val.ans);
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
            self.ans += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::<Node>::new(n + 1);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if b[i] == 0 {
            st.update(0..i, &Node { delta: 1, ans: 0 });
        } else {
            st.update(i + 1.., &Node { delta: 1, ans: 0 });
        }
        ans.push(st.query(..).ans + i + 1);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
