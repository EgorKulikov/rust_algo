//{"name":"M. Do Not Turn Back","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9488","interactive":false,"timeLimit":1000,"tests":[{"input":"6 8 5\n1 2\n1 3\n2 3\n2 4\n3 5\n4 5\n4 6\n5 6\n","output":"2\n"},{"input":"11 11 2023\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n10 11\n1 11\n","output":"1\n"},{"input":"7 21 1000000000\n1 2\n1 3\n1 4\n1 5\n1 6\n1 7\n2 3\n2 4\n2 5\n2 6\n2 7\n3 4\n3 5\n3 6\n3 7\n4 5\n4 6\n4 7\n5 6\n5 7\n6 7\n","output":"405422475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MDoNotTurnBack"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    type Mod = ModIntF;
    let graph = Graph::with_biedges(n, &edges);
    let mat1 = Matrix::zero(2 * n, 2 * n).do_with(|mat| {
        for i in 0..n {
            mat[(i + n, i)] = Mod::one();
            mat[(i, i + n)] = -Mod::from(graph[i].len()) + 1;
            for e in &graph[i] {
                mat[(i, e.to())] = Mod::one();
            }
        }
    });
    let mat2 = Matrix::zero(2 * n, 2 * n).do_with(|mat| {
        for i in 0..n {
            mat[(i + n, i)] = Mod::one();
            mat[(i, i + n)] = -Mod::from(graph[i].len());
            for e in &graph[i] {
                mat[(i, e.to())] = Mod::one();
            }
        }
    });
    let pw = Matrix::power(&mat1, k - 1);
    let res = Matrix::mult(&mat2, &pw);
    out.print_line(res[(0, n - 1)]);
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
        let path = "./m_do_not_turn_back";
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
fn m_do_not_turn_back() {
    assert!(tester::run_tests());
}
