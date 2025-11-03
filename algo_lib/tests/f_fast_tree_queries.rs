//{"name":"F. Fast Tree Queries","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8616","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6\n1 2\n1 3\n3 4\n3 5\n? 2 5\n+ 1 4 1\n? 2 5\n+ 4 5 2\n? 4 5\n? 1 1\n","output":"5\n1\n6\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::simd::{fast_apply, fast_fold};
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let graph = Graph::with_biedges(n, &edges);
    let hld = graph.hl_decomposition();
    let lca = graph.lca();
    let mut aa = Vec::with_gen(hld.paths.len(), |i| {
        Vec::with_gen(hld.paths[i].len(), |j| hld.paths[i][j] as i32 + 1)
    });
    for _ in 0..q {
        let t = input.read_char();
        match t {
            b'+' => {
                let a = input.read_size() - 1;
                let v = input.read_size() - 1;
                let x = input.read_int();
                let l = lca.lca(a, v);
                for part in hld.iter(a..=l).chain(hld.iter(v..l)) {
                    let xx = &mut aa[part.id];
                    fast_apply(&mut xx[part.pos_from..=part.pos_to], |a| a + x);
                }
            }
            b'?' => {
                let a = input.read_size() - 1;
                let v = input.read_size() - 1;
                let l = lca.lca(a, v);
                let mut ans = 0;
                for part in hld.iter(a..=l).chain(hld.iter(v..l)) {
                    let xx = &aa[part.id];
                    ans = fast_fold(&xx[part.pos_from..=part.pos_to], ans, |a, b| a ^ b);
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: LegacyTaskType = LegacyTaskType::Classic;

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
        LegacyTaskType::Classic => input.is_empty(),
        LegacyTaskType::Interactive => true,
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
    use tester::classic::EPS;
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
        let path = "./f_fast_tree_queries";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::LegacyTaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::LegacyTaskType::Classic => {
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
fn f_fast_tree_queries() {
    assert!(tester::run_tests());
}
