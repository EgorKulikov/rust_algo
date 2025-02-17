//{"name":"F. Periodic Substring","group":"Codeforces - ITMO Academy: pilot course - Suffix Array - Step 5","url":"https://codeforces.com/edu/course/2/lesson/2/5/practice/contest/269656/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"aabaaabaaabaaabaab\n","output":"4\n"},{"input":"aabaabb\n","output":"2\n"},{"input":"aaaa\n","output":"4\n"},{"input":"ppppplppp\n","output":"5\n"},{"input":"nn\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::suffix_array::SuffixArray;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::mem::take;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let sa = SuffixArray::new(s.as_slice());
    let mut dsu = DSU::new(s.len() + 1);
    let order = s
        .indices()
        .collect::<Vec<_>>()
        .sorted_by_key(|&i| Reverse(sa.lcp(i, i + 1)));
    let mut ans = 1;
    let mut sets = Vec::with_gen(s.len() + 1, |i| {
        let mut set = BTreeSet::new();
        set.insert(sa[i]);
        set
    });
    for i in order {
        let u = dsu.find(i);
        let v = dsu.find(i + 1);
        if sets[u].len() < sets[v].len() {
            sets.swap(u, v);
        }
        let lcp = sa.lcp(i, i + 1);
        let mut min_diff = usize::MAX;
        let sv = take(&mut sets[v]);
        for j in sv {
            if let Some(&k) = sets[u].prev(&j) {
                min_diff.minim(j - k);
            }
            if let Some(&k) = sets[u].next(&j) {
                min_diff.minim(k - j);
            }
            sets[u].insert(j);
        }
        ans.maxim(lcp / min_diff + 1);
        dsu.union(i, i + 1);
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
        let path = "./f_periodic_substring";
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
fn f_periodic_substring() {
    assert!(tester::run_tests());
}
