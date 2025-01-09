//{"name":"Min Max Paths","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/MINMAXPATHS","interactive":false,"timeLimit":7000,"tests":[{"input":"3\n5 3\n1 2\n2 3\n3 4\n4 5\n2 2\n1 2\n4 1\n1 2\n1 3\n2 4\n","output":"3 5 0 7 8\n2 0\n0 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinMaxPaths"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_size() - 1;
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let lca = graph.lca();
    let mut ans = vec![usize::MAX; n];
    let mut max_to = vec![0; n];
    let mut last = 0;
    let mut stack = Vec::new();
    loop {
        let mut min = None;
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            if vert < last {
                return;
            }
            min.minim(vert);
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
            }
        });
        dfs.call(s, s);
        let cur = min.unwrap();
        stack.push(cur);
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            if vert < last {
                return;
            }
            if vert == cur {
                max_to[vert] = vert + 1;
            } else {
                let val = max_to[prev].max(vert + 1);
                max_to[vert] = val;
            }
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
            }
        });
        dfs.call(cur, cur);
        let ends = cur == s || (cur + 1) * (cur + 1) >= 2 * n;
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            if vert < last {
                return;
            }
            if lca.on_path(s, vert, cur) {
                ans[vert].minim((max_to[vert].max(max_to[s])) * (cur + 1));
            } else {
                ans[vert].minim((max_to[vert] + max_to[s]) * (cur + 1));
            }
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
            }
        });
        dfs.call(s, s);
        if ends {
            break;
        }
        last = cur + 1;
    }
    ans[s] = 0;
    out.print_line(ans);
}

#[cfg(test)]
mod test {
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
    use algo_lib::numbers::primes::factorize::Factorize;
    use algo_lib::numbers::primes::sieve::primes;

    #[test]
    fn test() {
        let n = 2000u64;
        let m = n * 1000000000;
        let p = primes(n as usize + 1);
        let mut ans = None;
        let mut rec = RecursiveFunction3::new(|rec, mut cur: u64, step: usize, max: usize| {
            if max == 0 {
                let d = cur.divisors();
                let mut q = 0;
                for i in d {
                    if i <= n {
                        q += 1;
                    }
                }
                ans.maxim((q, cur));
                return;
            }
            for i in 0..=max {
                rec.call(cur, step + 1, i);
                cur *= p[step] as u64;
                if cur > m {
                    break;
                }
            }
        });
        rec.call(1, 0, 100);
        println!("{:?}", ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::collections::vec_ext::inc_dec::IncDec;
    use algo_lib::graph::distances::Distances;
    use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::Graph;
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
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

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            /*let n = random().gen_range(2..=4);
            let s = random().gen_range(1..=n);
            out.print_line(1);
            out.print_line((n, s));
            let mut dsu = DSU::new(n);
            for _ in 1..n {
                loop {
                    let u = random().gen_bound(n);
                    let v = random().gen_bound(n);
                    if dsu.union(u, v) {
                        out.print_line((u + 1, v + 1));
                        break;
                    }
                }
            }*/
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            input.read_size();
            let n = input.read_size();
            let s = input.read_size() - 1;
            let edges = input.read_size_pair_vec(n - 1).dec();

            let graph = Graph::from_biedges(n, &edges);
            let mut moves = Graph::new(n);
            for i in 0..n {
                let mut dfs = RecursiveFunction4::new(
                    |f, vert: usize, prev: usize, mut min: usize, mut max: usize| {
                        min.minim(vert + 1);
                        max.maxim(vert + 1);
                        moves.add_edge(BiWeightedEdge::new(i, vert, min * max));
                        for e in &graph[vert] {
                            if e.to() == prev {
                                continue;
                            }
                            f.call(e.to(), vert, min, max);
                        }
                    },
                );
                dfs.call(i, i, i + 1, i + 1);
            }
            out.print_line_iter(moves.distances_from(s).into_iter().map(|x| x.unwrap().0));
            true
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./min_max_paths";
        let time_limit = 7000;
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
        tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn min_max_paths() {
    assert!(tester::run_tests());
}
