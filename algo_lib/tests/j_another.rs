//{"name":"J. Another","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/J","interactive":false,"timeLimit":3000,"tests":[{"input":"6 6\n1 2 3 4 5 6\n2 4\n1 1 2 4 5\n2 4\n2 1\n1 1 3 4 6\n2 1\n","output":"5 6 -1\n5 3 1\n2 6 -1\n2 6 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JAnother"}}}

use algo_lib::collections::payload::PurePayload;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::treap::treap::Tree;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTestType;
use algo_lib::misc::test_type::TaskType;
use std::iter::repeat;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let p = input.read_size_vec(n).dec();

    let mut treap = Tree::new();
    let nodes = Vec::with_gen_prefix(n, |i, _| treap.add_back(PurePayload(p[i])));
    let pp = p.inv();

    for _ in 0..q {
        let command = input.read_size();
        match command {
            1 => {
                let l1 = input.read_size() - 1;
                let r1 = input.read_size();
                let l2 = input.read_size() - 1;
                let r2 = input.read_size();
                let right = treap.range_index(l2..r2).detach();
                let pos_left = treap.range_index(l1..r1);
                let left = pos_left.detach();
                pos_left.push_back(right);
                treap
                    .range_index(..l2 - (r1 - l1) + (r2 - l2))
                    .push_back(left);
            }
            2 => {
                let x = input.read_size() - 1;
                let pos = treap.index_ref(&nodes[pp[x]]);
                out.print_line_iter(
                    treap
                        .range_index(pos + 1..)
                        .iter()
                        .map(|node| Some(node.0 + 1))
                        .take(3)
                        .chain(repeat(None))
                        .take(3),
                );
            }
            _ => unreachable!(),
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
        let path = "./j_another";
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
fn j_another() {
    assert!(tester::run_tests());
}
