//{"name":"F1. Сверхскоростной подсчёт (простая версия)","group":"Codeforces - Codeforces Round 975 (Div. 1)","url":"https://codeforces.com/contest/2018/problem/F1","interactive":false,"timeLimit":2000,"tests":[{"input":"11\n1 998244353\n2 998244353\n3 998244353\n4 998244353\n5 998244353\n6 998244353\n7 998244353\n8 998244353\n9 998244353\n10 102275857\n10 999662017\n","output":"0 1\n1 2 1\n14 7 4 2\n183 34 19 16 4\n2624 209 112 120 48 12\n42605 1546 793 992 468 216 36\n785910 13327 6556 9190 4672 2880 864 144\n16382863 130922 61939 94992 50100 36960 14256 4608 576\n382823936 1441729 657784 1086596 583344 488700 216000 96480 23040 2880\n20300780 17572114 7751377 13641280 7376068 6810552 3269700 1785600 576000 144000 14400\n944100756 17572114 7751377 13641280 7376068 6810552 3269700 1785600 576000 144000 14400\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1SverkhskorostnoiPodschyotProstayaVersiya"}}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::{TaskType, LegacyTestType};
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_unsigned();

    dynamic_value!(ModVal: u32 = m);
    type Mod = ModInt<ModVal>;
    let mut ans = Vec::with_capacity(n + 1);
    let mut mem = Memoization2d::new(
        n + 2,
        n + 2,
        |mem, max: usize, len: usize| -> (Mod, Mod, Mod) {
            if len > max || max > n {
                (Mod::zero(), Mod::zero(), Mod::zero())
            } else if len == n {
                (Mod::one(), Mod::one(), Mod::one())
            } else {
                let started_right =
                    mem.call(max, len + 1).2 * (n + 1 - max) + mem.call(max + 1, len).0;
                let started_left = mem.call(max, len + 1).1 * (n + 1 - max)
                    + mem.call(max, len + 1).2
                    + mem.call(max + 1, len).0;
                let not_started =
                    mem.call(max, len + 1).1 + mem.call(max, len + 1).2 + mem.call(max + 1, len).0;
                (not_started, started_left, started_right)
            }
        },
    );
    let mut total = Mod::zero();
    for i in (1..=n).rev() {
        let mut cur = Mod::one();
        for j in 0..i {
            let dist = j.min(i - j - 1);
            cur *= n + 1 - (i - dist);
        }
        cur *= mem.call(i, i).0;
        for j in 0..n - i {
            cur -= ans[j] * (n - i + 1 - j);
        }
        ans.push(cur);
        total += cur;
    }
    ans.push(Mod::from(n).power(n) - total);
    ans.reverse();
    out.print_line(ans);
}

pub static TEST_TYPE: LegacyTestType = LegacyTestType::MultiNumber;
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
        let path = "./f1_sverkhskorostnoi_podschyot_prostaya_versiya";
        let time_limit = 2000;
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
fn f1_sverkhskorostnoi_podschyot_prostaya_versiya() {
    assert!(tester::run_tests());
}
