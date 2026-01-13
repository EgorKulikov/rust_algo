//{"name":"M. Cup Trick","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/M","interactive":false,"timeLimit":3000,"tests":[{"input":"2 1\n2 1\n","output":"2 1\n"},{"input":"3 2\n1 2\n1 1\n","output":"2 1 3\n"},{"input":"3 3\n1 3\n2 3\n1 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MCupTrick"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::payload::PurePayload;
use algo_lib::collections::treap::treap::Tree;
// use algo_lib::collections::tree::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;
use algo_lib::misc::time_tracker::TimeTracker;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut timer = TimeTracker::new();
    let n = input.read_size();
    let m = input.read_size();
    let swaps = input.read_size_pair_vec(m).dec();

    timer.milestone("read");
    let mut not_seen = BitSet::new(n);
    not_seen.fill(true);
    let mut treap = Tree::with_gen(n, |i| PurePayload((None, i)));
    // let mut treap = Treap::gen_sized(n, |i| PurePayload((None, i)));
    timer.milestone("init");
    for &(y, x) in &swaps {
        let node = treap.range_index(x..=x);
        match node.payload().unwrap().0 {
            (Some(id), _) => {
                if id != y {
                    out.print_line(-1);
                    return;
                }
            }
            (None, id) => {
                if !not_seen[y] {
                    out.print_line(-1);
                    return;
                }
                not_seen.unset(y);
                node.replace(PurePayload((Some(y), id)));
            }
        }
        let mid = node.detach();
        treap.push_front(mid);
    }
    timer.milestone("solve");
    let mut iter = not_seen.iter();
    let mut ans = vec![None; n];
    for &PurePayload((id, index)) in treap.iter() {
        ans[index] = id;
    }
    for i in 0..n {
        if ans[i].is_none() {
            ans[i] = Some(iter.next().unwrap());
        }
    }
    timer.milestone("ans");
    out.print_line_iter(ans.into_iter().map(|x| x.unwrap() + 1));
    timer.milestone("print");
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
    // use algo_lib::misc::random::random;
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

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            /*out.print_line((1000000, 1000000));
            for i in 1..=1000000 {
                out.print_line((i, random().gen_range(i..=1000000)));
            }*/
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./m_cup_trick";
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
        tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn m_cup_trick() {
    assert!(tester::run_tests());
}
