//{"name":"Kanna's Friendship","group":"Kattis","url":"https://open.kattis.com/problems/kannafriendship","interactive":false,"timeLimit":1000,"tests":[{"input":"10 12\n1 5 5\n2\n1 6 6\n2\n1 5 6\n2\n1 2 4\n2\n1 3 8\n2\n1 1 10\n2\n","output":"1\n2\n2\n5\n7\n10\n"},{"input":"26 19\n1 8 8\n1 15 15\n1 20 20\n1 19 19\n1 16 16\n1 21 21\n1 18 18\n2\n1 1 1\n1 9 9\n1 9 9\n1 7 7\n2\n1 1 1\n1 20 20\n1 8 8\n1 9 9\n1 14 14\n2\n","output":"7\n10\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KannasFriendship"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::iter::repeat;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let q = input.read_size();
    enum Query {
        Add(usize, usize),
        Query,
    }
    impl Readable for Query {
        fn read(input: &mut Input) -> Self {
            let t = input.read_size();
            match t {
                1 => {
                    let s = input.read_size() - 1;
                    let e = input.read_size();
                    Query::Add(s, e)
                }
                2 => Query::Query,
                _ => unreachable!(),
            }
        }
    }
    let queries = input.read_vec::<Query>(q);

    let mut poi = Vec::new();
    for query in &queries {
        if let Query::Add(s, e) = query {
            poi.push(*s);
            poi.push(*e);
        }
    }
    if poi.is_empty() {
        out.print_line_iter(repeat(0).take(q));
        return;
    }
    poi.sort_unstable();
    poi.dedup();
    #[derive(Clone, Default)]
    struct Node {
        len: usize,
        ans: usize,
        enabled: bool,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.len = left_val.len + right_val.len;
            self.ans = left_val.ans + right_val.ans;
        }

        fn accumulate(&mut self, value: &Self) {
            if value.enabled {
                self.enabled = true;
                self.ans = self.len;
            }
        }

        fn reset_delta(&mut self) {
            self.enabled = false;
        }
    }
    let mut st = SegmentTree::with_gen(poi.len() - 1, |i| Node {
        len: poi[i + 1] - poi[i],
        ans: 0,
        enabled: false,
    });

    for query in queries {
        match query {
            Query::Add(s, e) => {
                let s = poi.lower_bound(&s);
                let e = poi.lower_bound(&e);
                st.update(
                    s..e,
                    &Node {
                        len: 0,
                        ans: 0,
                        enabled: true,
                    },
                );
            }
            Query::Query => {
                out.print_line(st.query(..).ans);
            }
        }
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

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

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

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./kannas_friendship";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn kannas_friendship() {
    assert!(tester::run_tests());
}
