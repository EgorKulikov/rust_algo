//{"name":"E. Euclid's Tree","group":"Yandex - Yandex Cup 2024 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/70295/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n5\n4 6 6 6 5\n1 5\n2 5\n5 4\n3 4\n","output":"3\n3\n3\n2\n0\n"},{"input":"2\n5\n6 4 6 2 4\n5 3\n1 5\n5 2\n2 4\n9\n4 6 6 7 7 3 4 4 2\n2 7\n1 6\n3 4\n4 9\n6 4\n3 2\n5 2\n6 8\n","output":"3\n2\n3\n3\n2\n5\n4\n3\n3\n3\n3\n5\n5\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEuclidsTree"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let divisors = divisor_table::<usize>(10_000_001);

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let a = input.read_size_vec(n);
        let edges = input.read_size_pair_vec(n - 1).dec();

        let graph = Graph::from_biedges(n, &edges);
        let lca = graph.lca();
        let mut poi = DefaultHashMap::<_, Vec<_>>::new();
        for i in 0..n {
            let mut cur = a[i];
            while cur != 1 {
                let d = divisors[cur];
                while cur % d == 0 {
                    cur /= d;
                }
                poi[d].push(i);
            }
        }
        let mut ans = vec![None; n];
        for v in poi.values() {
            let mut far = None;
            for &u in v {
                far.maxim((lca.path_length(u, v[0]), u));
            }
            let v1 = far.unwrap().1;
            far = None;
            for &u in v {
                far.maxim((lca.path_length(u, v1), u));
            }
            let v2 = far.unwrap().1;
            for &u in v {
                ans[u].maxim(lca.path_length(u, v1));
                ans[u].maxim(lca.path_length(u, v2));
            }
        }
        out.print_per_line(&ans);
    }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
