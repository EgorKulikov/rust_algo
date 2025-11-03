//{"name":"D. Максимум + минимум + количество","group":"Codeforces - Codeforces Round 975 (Div. 1)","url":"https://codeforces.com/contest/2018/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n5 4 5\n3\n4 5 4\n10\n3 3 3 3 4 1 2 3 5 4\n10\n17 89 92 42 29 41 92 14 70 45\n","output":"12\n11\n12\n186\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaksimumMinimumKolichestvo"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::{LegacyTaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long_vec(n);

    #[derive(Copy, Clone, Default)]
    struct Result {
        res: i64,
        max: i64,
        max_but_one: i64,
    }

    const INF: i64 = i64::MIN / 10;
    impl Result {
        fn new() -> Self {
            Self {
                res: INF,
                max: INF,
                max_but_one: INF,
            }
        }

        fn update(&mut self, res: i64, max: i64) {
            if self.res + 1 < res {
                self.res = res;
                self.max = max;
                self.max_but_one = INF;
            } else if self.res + 1 == res {
                self.max_but_one = self.max;
                self.res = res;
                self.max = max;
            } else if self.res == res {
                self.max.maxim(max);
            } else if self.res - 1 == res {
                self.max_but_one.maxim(max);
            }
        }

        fn update_composition(&mut self, a: &Result, b: &Result) {
            self.update(a.res + b.res, a.max.max(b.max));
            self.update(a.res + b.res - 1, a.max_but_one.max(b.max_but_one));
        }

        fn value(&self) -> i64 {
            (self.res + self.max).max(self.res - 1 + self.max_but_one)
        }
    }

    #[derive(Clone)]
    struct Node {
        ans: Arr2d<Result>,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                ans: Arr2d::new(2, 2, Result::default()),
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            for i in 0..2 {
                for j in 0..2 {
                    let mut cur = Result::new();
                    for l in 0..2 {
                        for m in 0..2 - l {
                            cur.update_composition(&left_val.ans[(i, l)], &right_val.ans[(m, j)]);
                        }
                    }
                    self.ans[(i, j)] = cur;
                }
            }
        }
    }

    let mut ans = None;
    let mut order: Vec<_> = (0..n).collect();
    order.sort_by_key(|&i| x[i]);
    order.reverse();
    let mut added = BitSet::new(n);
    let mut st = SegmentTree::with_gen(n, |_| {
        let mut res = Node {
            ans: Arr2d::new(2, 2, Result::new()),
        };
        res.ans[(0, 0)].update(0, INF);
        res
    });
    for i in order {
        added.set(i);
        st.binary_search_mut_with_mid(
            |_, _, _, mid| {
                if i < mid {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            |node, _| {
                node.ans[(1, 1)].update(1, x[i]);
            },
        );
        let prefix = st.query(0..i);
        let suffix = st.query(i..n);
        let mut res = Result::new();
        for i in 0..2 {
            for j in 0..2 {
                res.update_composition(&prefix.ans[(i, 0)], &suffix.ans[(1, j)]);
            }
        }
        ans.maxim(res.value() + x[i]);
    }
    out.print_line(ans);
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
        let path = "./d_maksimum_minimum_kolichestvo";
        let time_limit = 2000;
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
fn d_maksimum_minimum_kolichestvo() {
    assert!(tester::run_tests());
}
