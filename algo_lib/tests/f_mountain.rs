//{"name":"F. Mountain","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 2 - Step 4","url":"https://codeforces.com/edu/course/2/lesson/5/4/practice/contest/280801/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nQ 1\nI 1 4 2\nQ 3\nQ 1\nI 2 2 -1\nQ 3\nE\n","output":"4\n1\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    #[derive(Eq, PartialEq)]
    enum Command {
        I(i64, i64, i64),
        Q(i64),
        E,
    }
    impl Readable for Command {
        fn read(input: &mut Input) -> Self {
            let command = input.read_char();
            match command {
                b'I' => Command::I(input.read_long() - 1, input.read(), input.read()),
                b'Q' => Command::Q(input.read()),
                b'E' => Command::E,
                _ => panic!(),
            }
        }
    }
    let commands = input
        .iter::<Command>()
        .take_while(|c| *c != Command::E)
        .collect::<Vec<_>>();

    #[derive(Clone, Default)]
    struct Node {
        max: i64,
        len: i64,
        left_height: i64,
        left_pos: i64,
        delta: Option<i64>,
        delta_left_height: i64,
        delta_height: i64,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.max = left_val.max.max(right_val.max);
            self.len = left_val.len + right_val.len;
            self.left_height = left_val.left_height;
            self.left_pos = left_val.left_pos;
        }

        fn accumulate(&mut self, delta: &Self) {
            if let Some(d) = delta.delta {
                self.left_height = delta.delta_left_height + (self.left_pos - delta.left_pos) * d;
                self.delta_left_height = self.left_height;
                self.max = self.left_height.max(self.left_height + self.len * d);
                self.delta = Some(d);
                self.delta_height = 0;
            }
            self.left_height += delta.delta_height;
            self.max += delta.delta_height;
            self.delta_height += delta.delta_height;
        }

        fn reset_delta(&mut self) {
            self.delta = None;
            self.delta_height = 0;
        }
    }

    let mut poi = vec![0, n];
    for c in &commands {
        if let Command::I(l, r, _) = c {
            poi.push(*l);
            poi.push(*r);
        }
    }
    poi.sort_unstable();
    poi.dedup();

    let mut st = SegmentTree::with_gen(poi.len(), |i| Node {
        left_pos: poi[i],
        len: if i + 1 == poi.len() {
            0
        } else {
            poi[i + 1] - poi[i]
        },
        delta: Some(0),
        ..Default::default()
    });

    for c in commands {
        match c {
            Command::I(l, r, d) => {
                let l = poi.lower_bound(&l);
                let r = poi.lower_bound(&r);
                let left_height = st.point_query(l).left_height;
                let right_height = st.point_query(r).left_height;
                st.update(
                    l..r,
                    &Node {
                        delta_left_height: left_height,
                        left_pos: poi[l],
                        delta: Some(d),
                        ..Default::default()
                    },
                );
                let new_right_height = left_height + (poi[r] - poi[l]) * d;
                st.update(
                    r..,
                    &Node {
                        delta_height: new_right_height - right_height,
                        ..Default::default()
                    },
                )
            }
            Command::Q(h) => out.print_line(st.binary_search(
                |left, _| {
                    if left.max > h {
                        Direction::Left
                    } else {
                        Direction::Right
                    }
                },
                |node, pos| {
                    if pos + 1 == poi.len() {
                        node.left_pos
                    } else {
                        let delta = node.delta.unwrap();
                        assert!(delta > 0);
                        assert!(h >= node.left_height);
                        let res = node.left_pos + (h - node.left_height) / delta;
                        assert!(res < node.left_pos + node.len);
                        res
                    }
                },
            )),
            Command::E => unreachable!(),
        }
    }
}

pub static TEST_TYPE: LegacyTestType = LegacyTestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        LegacyTestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        LegacyTestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        LegacyTestType::MultiEof => {
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
    use algo_lib::misc::random::Random;
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

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

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

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./f_mountain";
        let tl = 3000;
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
fn f_mountain() {
    assert!(tester::run_tests());
}
