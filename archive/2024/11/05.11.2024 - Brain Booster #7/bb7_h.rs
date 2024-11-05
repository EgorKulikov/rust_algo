//{"name":"bb7_h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"bb7_h"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::segment_tree::{QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    value_ref!(Vals A: Vec<usize> = a);
    struct Node {
        qty: DefaultHashMap<usize, usize>,
        big: Vec<usize>,
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            let mut qty = DefaultHashMap::new();
            let mut big = Vec::new();
            for i in left..right {
                qty[Vals::val()[i]] += 1;
                if qty[Vals::val()[i]] * 3 > right - left {
                    big.push(Vals::val()[i]);
                }
            }
            big.sort();
            big.dedup();
            Self { qty, big }
        }

        fn join(&mut self, _left_val: &Self, _right_val: &Self) {}

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    impl QueryResult<Vec<usize>, ()> for Node {
        fn empty_result(_args: &()) -> Vec<usize> {
            Vec::new()
        }

        fn result(&self, _args: &()) -> Vec<usize> {
            self.big.clone()
        }

        fn join_results(
            mut left_res: Vec<usize>,
            mut right_res: Vec<usize>,
            _args: &(),
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> Vec<usize> {
            if left_res.len() < right_res.len() {
                swap(&mut left_res, &mut right_res);
            }
            left_res.extend_from_slice(&right_res);
            left_res.sort();
            left_res.dedup();
            left_res
        }
    }

    impl QueryResult<usize, usize> for Node {
        fn empty_result(_args: &usize) -> usize {
            0
        }

        fn result(&self, args: &usize) -> usize {
            self.qty[*args]
        }

        fn join_results(
            left_res: usize,
            right_res: usize,
            _args: &usize,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> usize {
            left_res + right_res
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let cand = st.query(l..r);
        let mut ans = None;
        for i in cand {
            if 3 * st.query_with_args(l..r, &i) > r - l {
                ans = Some(i);
                break;
            }
        }
        out.print_line(ans);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
