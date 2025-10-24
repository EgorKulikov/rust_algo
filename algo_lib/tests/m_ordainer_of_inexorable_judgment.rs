//{"name":"M. Ordainer of Inexorable Judgment","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9576","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1 0 1 1\n1 2\n2 1\n2 2\n","output":"1.000000000000\n"},{"input":"3 1 0 1 2\n1 2\n2 1\n2 2\n","output":"1.570796326795\n"},{"input":"3 1 0 1 10000\n1 2\n2 1\n2 2\n","output":"2500.707752257475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MOrdainerOfInexorableJudgment"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::circle::Circle;
use algo_lib::geometry::geometry_utils::{canonize_angle, canonize_angle_base};
use algo_lib::geometry::point::Point;
use algo_lib::geometry::ray::Ray;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let dir = input.read::<Point<Real>>().angle();
    let d = input.read_real();
    let mut t = input.read_real();
    let points = input.read_vec::<Point<Real>>(n);

    let circle = Circle::new(Point::origin(), d);
    let base_dir = points[0].angle();
    let mut max_angle = base_dir;
    let mut min_angle = base_dir;

    for p in points {
        let tangent = circle.tangent_points(p);
        for t in tangent {
            let angle = Ray::new(t, p).angle();
            let delta = canonize_angle(angle - base_dir);
            if delta > 0. {
                max_angle.maxim(base_dir + delta);
            } else {
                min_angle.minim(base_dir + delta);
            }
        }
    }
    let span = max_angle - min_angle;
    let mut dir = canonize_angle_base(dir - min_angle, Real::zero());
    let mut ans = Real::zero();
    if dir < span {
        let rotate = (span - dir).min(t);
        t -= rotate;
        ans += rotate;
        dir = span;
    }
    while t > 0. {
        let rotate = t.min(Real::PI * 2 - dir);
        t -= rotate;
        let rotate = span.min(t);
        t -= rotate;
        ans += rotate;
        dir = span;
    }
    out.print_line(ans);
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
    use tester::classic::default_checker_eps_rel;
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
        let path = "./m_ordainer_of_inexorable_judgment";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(
                    tl,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    default_checker_eps_rel,
                )
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
fn m_ordainer_of_inexorable_judgment() {
    assert!(tester::run_tests());
}
