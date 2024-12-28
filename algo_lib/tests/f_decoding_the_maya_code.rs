//{"name":"F. Decoding the Maya Code","group":"Yandex - Yandex Cup 2024 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/70295/problems/F/","interactive":false,"timeLimit":3000,"tests":[{"input":"5 4\nA\nB\nC\nD\nE\n1 2\n1 3\n1 4\n2 5\n1 AABCDE\n1 A\n2 BEE\n3 CCCA\n","output":"6\n1\n3\n3\n"},{"input":"6 4\nAA\nAB\nBC\nCD\nAA\nAA\n1 2\n2 3\n3 4\n4 5\n5 6\n1 AAABCD\n2 AABCD\n3 BCD\n1 DCBA\n","output":"9\n5\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDecodingTheMayaCode"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::dfs_order::{DFSOrder, DFSOrderTrait};
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::aho_corasick::{ACPayload, AhoCorasickUppercase};
use algo_lib::string::str::{Str, StrReader};
use std::ops::Deref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();
    let queries = input.read_vec::<(usize, Str)>(q);

    let graph = Graph::from_biedges(n, &edges);
    let DFSOrder { position, end } = graph.dfs_order();
    let mut ans = vec![0; q];
    let mut by_query = DefaultHashMap::new(Vec::new());
    for i in 0..q {
        let v = queries[i].0 - 1;
        let left = position[v];
        let right = end[v];
        let mut rec = RecursiveFunction2::new(|rec, from: usize, to: usize| {
            if from >= right || to <= left {
                return;
            }
            if from >= left && to <= right {
                by_query[(from, to)].push(i);
                return;
            }
            let mid = (from + to) / 2;
            rec.call(from, mid);
            rec.call(mid, to);
        });
        rec.call(0, n);
    }

    let mut s_ordered = vec![Str::new(); n];
    for i in 0..n {
        s_ordered[position[i]] = s[i].clone();
    }
    #[derive(Default, Debug)]
    struct Data(usize);

    impl ACPayload for Data {
        fn add_single(&mut self, _id: usize) {
            self.0 += 1;
        }

        fn add_node(&mut self, other: &Self) {
            self.0 += other.0;
        }
    }

    for ((f, t), id) in by_query {
        let ac = AhoCorasickUppercase::<Data>::new(&s_ordered[f..t]);
        for i in id {
            for x in ac.iterate(&queries[i].1.deref()) {
                ans[i] += x.0;
            }
        }
    }
    out.print_per_line(&ans);
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
    use algo_lib::collections::vec_ext::inc_dec::IncDec;
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::random;
    use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
    use algo_lib::string::str::{Str, StrReader};
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

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let n = random().gen_range(1..=3);
            let q = 1;
            out.print_line((n, q));
            for _ in 0..n {
                let len = random().gen_range(1..=3);
                for _ in 0..len {
                    out.print(random().gen_range(b'A'..=b'B'));
                }
                out.print_line(());
            }
            for i in 2..=n {
                out.print_line((random().gen_range(1..i), i));
            }
            for _ in 0..q {
                let len = random().gen_range(1..=10);
                out.print((random().gen_range(1..=n), ()));
                for _ in 0..len {
                    out.print(random().gen_range(b'A'..=b'B'));
                }
                out.print_line(());
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            let n = input.read_size();
            let q = input.read_size();
            let s = input.read_str_vec(n);
            let edges = input.read_size_pair_vec(n - 1).dec();
            let queries = input.read_vec::<(usize, Str)>(q);

            for (id, t) in queries {
                let id = id - 1;
                let mut ans = 0;
                let mut rec = RecursiveFunction::new(|rec, vert: usize| {
                    for i in 0..=t.len().saturating_sub(s[vert].len()) {
                        if t[i..].starts_with(&s[vert]) {
                            ans += 1;
                        }
                    }
                    for &(u, v) in &edges {
                        if u == vert {
                            rec.call(v);
                        }
                    }
                });
                rec.call(id);
                out.print_line(ans);
            }

            true
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            out.print_line((100000, 1));
            for i in 0..100000 {
                out.print_line('A');
            }
            for i in 1..100000 {
                out.print_line((i, i + 1));
            }
            out.print("1 ");
            for _ in 0..1000000 {
                out.print("A");
            }
            out.print_line("");
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            out.print_line(100_000i64 * 1_000_000);
            true
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./f_decoding_the_maya_code";
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
fn f_decoding_the_maya_code() {
    assert!(tester::run_tests());
}
