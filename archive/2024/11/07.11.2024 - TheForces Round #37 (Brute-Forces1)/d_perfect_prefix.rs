//{"name":"D. Perfect Prefix","group":"Codeforces - TheForces Round #37 (Brute-Forces1)","url":"https://codeforces.com/gym/105491/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n5 5\n1 1 -1 1 -1\n1 1 4\n1 2 4\n2 3\n1 1 4\n1 4 5\n11 6\n1 1 1 1 -1 -1 -1 1 1 -1 -1\n1 7 9\n1 1 11\n2 5\n2 6\n1 1 7\n1 7 10\n","output":"0\n1\n0\n-1\n1\n0\n0\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPerfectPrefix"}}}

use algo_lib::collections::segment_tree::{QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cell::Cell;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_int_vec(n);

    #[derive(Default)]
    struct State {
        delta: i32,
        low: i32,
    }

    impl State {
        fn new(a: i32) -> Self {
            Self { delta: a, low: a }
        }
    }

    impl SegmentTreeNode for State {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.delta = left_val.delta + right_val.delta;
            self.low = left_val.low.min(right_val.low + left_val.delta);
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    impl QueryResult<(), Cell<(i32, i32)>> for State {
        fn empty_result(_args: &Cell<(i32, i32)>) -> () {}

        fn result(&self, args: &Cell<(i32, i32)>) -> () {
            let (sum, shifts) = args.get();
            let add = (-(sum + shifts * 2 + self.low) + 1).max(0);
            args.set((sum + self.delta, shifts + (add + 1) / 2));
        }

        fn join_results(
            _left_res: (),
            _right_res: (),
            _args: &Cell<(i32, i32)>,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> () {
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| State::new(a[i]));
    for _ in 0..q {
        match input.read_int() {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let ans = Cell::new((0, 0));
                st.query_with_args(l..r, &ans);
                let (sum, shifts) = ans.get();
                if sum <= 0 {
                    out.print_line(-1);
                } else {
                    out.print_line(shifts);
                }
            }
            2 => {
                let x = input.read_size() - 1;
                a[x] *= -1;
                st.point_update(x, State::new(a[x]));
            }
            _ => unreachable!(),
        }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
