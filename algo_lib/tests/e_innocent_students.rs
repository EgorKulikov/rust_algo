//{"name":"E. Innocent Students","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 5\n1 -2 3\n1 1 3 0\n2 3 -2\n1 2 3 1\n2 1 -4\n1 1 1 -10\n7 8\n1 2 4 2 4 6 7\n1 3 5 5\n1 1 7 5\n2 1 6\n2 4 4\n1 1 7 5\n1 1 3 2\n2 7 1\n1 1 7 -1\n","output":"1\n2\n1\n2\n3\n5\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EInnocentStudents"}}}

use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::segment_tree::{Pushable, QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_int_vec(n);

    struct Node {
        values: MultiTreeSet<i32>,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                values: MultiTreeSet::new(),
            }
        }

        fn join(&mut self, _left_val: &Self, _right_val: &Self) {}

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    impl QueryResult<(i32, usize), i32> for Node {
        fn empty_result(_args: &i32) -> (i32, usize) {
            (i32::MAX, 0)
        }

        fn result(&self, args: &i32) -> (i32, usize) {
            let left = self
                .values
                .range_rev(..args)
                .next()
                .copied()
                .map(|t| ((t - *args).abs(), *self.values.get(&t).unwrap()))
                .unwrap_or(Self::empty_result(args));
            let right = self
                .values
                .range(args..)
                .next()
                .copied()
                .map(|t| ((t - *args).abs(), *self.values.get(&t).unwrap()))
                .unwrap_or(Self::empty_result(args));
            Self::join_results(left, right, args, 0, 0, 0)
        }

        fn join_results(
            left_res: (i32, usize),
            right_res: (i32, usize),
            _args: &i32,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> (i32, usize) {
            match left_res.0.cmp(&right_res.0) {
                Ordering::Less => left_res,
                Ordering::Equal => (left_res.0, left_res.1 + right_res.1),
                Ordering::Greater => right_res,
            }
        }
    }

    impl Pushable<&(i32, i32)> for Node {
        fn push(&mut self, delta: &(i32, i32)) {
            self.values.remove(&delta.0);
            self.values.insert(delta.1);
        }
    }

    impl Pushable<&i32> for Node {
        fn push(&mut self, delta: &i32) {
            self.values.insert(*delta);
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    for i in 0..n {
        st.point_through_update(i, &a[i]);
    }

    for _ in 0..q {
        match input.read_int() {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_int();
                out.print_line(st.query_with_args(l..r, &x).1);
            }
            2 => {
                let at = input.read_size() - 1;
                let n_val = input.read_int();
                st.point_through_update(at, &(a[at], n_val));
                a[at] = n_val;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./e_innocent_students";
        let time_limit = 4000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    std_interactor,
                )
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    default_checker,
                )
                //Tester::new_classic(time_limit, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn e_innocent_students() {
    assert!(tester::run_tests());
}
