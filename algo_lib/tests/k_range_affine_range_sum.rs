//{"name":"K - Range Affine Range Sum","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_k","interactive":false,"timeLimit":5000,"tests":[{"input":"5 7\n1 2 3 4 5\n1 0 5\n0 2 4 100 101\n1 0 3\n0 1 3 102 103\n1 2 5\n0 2 5 104 105\n1 0 5\n","output":"15\n404\n41511\n4317767\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KRangeAffineRangeSum"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModIntF;
    #[derive(Clone)]
    struct Node {
        sum: Mod,
        a: Mod,
        b: Mod,
        len: usize,
    }
    impl Node {
        fn new(b: Mod) -> Self {
            Self {
                sum: b,
                a: Mod::one(),
                b: Mod::zero(),
                len: 1,
            }
        }
    }
    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                sum: Mod::zero(),
                a: Mod::one(),
                b: Mod::zero(),
                len: right - left,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum + right_val.sum;
        }

        fn accumulate(&mut self, value: &Self) {
            self.sum = self.sum * value.a + value.b * Mod::from_index(self.len);
            self.b = self.b * value.a + value.b;
            self.a *= value.a;
        }

        fn reset_delta(&mut self) {
            self.a = Mod::one();
            self.b = Mod::zero();
        }
    }

    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_vec::<Mod>(n);

    let mut st = SegmentTree::gen(n, |i| Node::new(a[i]));
    for _ in 0..q {
        match input.read::<usize>() {
            0 => {
                let l = input.read::<usize>();
                let r = input.read::<usize>();
                let b = input.read::<Mod>();
                let c = input.read::<Mod>();
                st.update(
                    l..r,
                    &Node {
                        sum: Mod::zero(),
                        a: b,
                        b: c,
                        len: 0,
                    },
                );
            }
            1 => {
                let l = input.read::<usize>();
                let r = input.read::<usize>();
                out.print_line(st.query(l..r).sum);
            }
            _ => unreachable!(),
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

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./k_range_affine_range_sum";
        let time_limit = 5000;
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
fn k_range_affine_range_sum() {
    assert!(tester::run_tests());
}
