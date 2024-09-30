//{"name":"M. Фокус со стаканчиками","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/M","interactive":false,"timeLimit":3000,"tests":[{"input":"2 1\n2 1\n","output":"2 1\n"},{"input":"3 2\n1 2\n1 1\n","output":"2 1 3\n"},{"input":"3 3\n1 3\n2 3\n1 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MFokusSoStakanchikami"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::treap::{PurePayload, Treap};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let swaps = input.read_size_pair_vec(m).dec();

    let mut not_seen = BitSet::new(n);
    not_seen.fill(true);
    let mut treap = Treap::new();
    for _ in 0..n {
        treap.add(PurePayload(None));
    }
    for &(y, x) in &swaps {
        let (left, mut mid, right) = treap.split_at_single(x);
        match mid.payload().unwrap().0 {
            Some(id) => {
                if id != y {
                    out.print_line(-1);
                    return;
                }
            }
            None => {
                if !not_seen[y] {
                    out.print_line(-1);
                    return;
                }
                not_seen.unset(y);
                mid = Treap::single(PurePayload(Some(y)));
            }
        }
        treap = Treap::merge_three(mid, left, right);
    }
    for (_, x) in swaps.into_iter().rev() {
        let (left, mid_right) = treap.split_at(1);
        let (mid, right) = mid_right.split_at(x);
        treap = Treap::merge_three(mid, left, right);
    }
    let mut iter = not_seen.iter();
    out.print_line_iter(
        treap
            .iter()
            .map(|payload| payload.0.unwrap_or_else(|| iter.next().unwrap()))
            .map(|x| x + 1),
    );
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
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            out.print_line((1000000, 1000000));
            for i in 1..=1000000 {
                out.print_line((i, i));
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./m_fokus_so_stakanchikami";
        let time_limit = 3000;
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
fn m_fokus_so_stakanchikami() {
    assert!(tester::run_tests());
}