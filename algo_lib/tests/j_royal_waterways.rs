//{"name":"J. Royal Waterways","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 0 0 1 0\n2 0 5 1 5\n4 1 -10 -5 10\n","output":"2\n"},{"input":"3\n1 0 0 5 0\n1 0 5 0 0\n1 5 5 0 5\n","output":"3\n"},{"input":"4\n1 0 0 5 0\n1 0 5 0 0\n1 5 5 0 5\n1 5 0 0 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_ext::Square;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let hw: Vec<(i128, i128, i128, i128, i128)> = input.read_vec(n);

    let s = Vec::with_gen(n, |i| {
        Point::new(Rational::new_int(hw[i].1), Rational::new_int(hw[i].2))
    });
    let t = Vec::with_gen(n, |i| {
        Point::new(Rational::new_int(hw[i].3), Rational::new_int(hw[i].4))
    });
    let r = Vec::with_gen(n, |i| Rational::new_int(hw[i].0));
    let seg = Vec::with_gen(n, |i| Segment::new(s[i], t[i]));
    let ds = Arr2d::with_gen(n, n, |i, j| {
        seg[i].square_dist_point(s[j]) > (r[i] + r[j]).square()
    });
    let dt = Arr2d::with_gen(n, n, |i, j| {
        seg[i].square_dist_point(t[j]) > (r[i] + r[j]).square()
    });
    let can = Vec::with_gen_prefix(1 << n, |i, can| {
        if i == 0 {
            true
        } else {
            for j in 0..n {
                if i.is_set(j) && can[i.without_bit(j)] {
                    let mut good = true;
                    for k in 0..n {
                        if k != j {
                            if i.is_set(k) && !dt[(j, k)] {
                                good = false;
                                break;
                            }
                            if !i.is_set(k) && !ds[(j, k)] {
                                good = false;
                                break;
                            }
                        }
                    }
                    if good {
                        return true;
                    }
                }
            }
            false
        }
    });
    let mut ans = 0;
    for i in can.indices() {
        if can[i] {
            ans.maxim(i.count_ones());
        }
    }
    out.print_line(ans);
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
        let path = "./j_royal_waterways";
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
fn j_royal_waterways() {
    assert!(tester::run_tests());
}
