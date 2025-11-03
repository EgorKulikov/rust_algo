//{"name":"Triangle Count (Hard)","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/TRICOUNT2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n5 2 4 12\n6\n3 1 6 5 50 17\n3\n100 2 69\n","output":"3 7 15\n1 6 9 20 53\n3 137\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TriangleCountHard"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{LegacyTaskType, TestType};
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Default, Debug)]
    struct Segments {
        segments: BTreeSet<(usize, usize)>,
        len: usize,
    }

    impl Segments {
        fn add(&mut self, mut l: usize, mut r: usize) {
            if let Some(&(a, b)) = self.segments.floor(&(l, usize::MAX)) {
                if b >= l {
                    self.segments.remove(&(a, b));
                    self.len -= b - a + 1;
                    l.minim(a);
                    r.maxim(b);
                }
            }
            while let Some(&(a, b)) = self.segments.ceil(&(l, l)) {
                if a > r {
                    break;
                }
                self.segments.remove(&(a, b));
                self.len -= b - a + 1;
                r.maxim(b);
            }
            self.segments.insert((l, r));
            self.len += r - l + 1;
        }
    }

    let mut segments = Segments::default();
    let mut set = BTreeSet::new();
    let mut ans = Vec::with_capacity(n);
    for x in a {
        if let Some(&y) = set.floor(&x) {
            segments.add(x - y + 1, x + y - 1);
        }
        if let Some(&y) = set.ceil(&x) {
            segments.add(y - x + 1, x + y - 1);
        }
        // eprintln!("{x} {:?}", segments);
        set.insert(x);
        ans.push(segments.len);
    }
    out.print_line_iter(ans.into_iter().skip(1));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        LegacyTaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        LegacyTaskType::Interactive => true,
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
        let path = "./triangle_count_hard";
        let time_limit = 1000;
        let tester = match TASK_TYPE {
            crate::LegacyTaskType::Interactive => {
                Tester::new_interactive(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    std_interactor,
                )
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::LegacyTaskType::Classic => {
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
fn triangle_count_hard() {
    assert!(tester::run_tests());
}
