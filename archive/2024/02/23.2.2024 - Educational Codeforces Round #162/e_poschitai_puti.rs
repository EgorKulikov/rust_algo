//{"name":"E. Посчитай пути","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 2 1\n1 2\n2 3\n5\n2 1 2 1 2\n1 2\n1 3\n3 4\n4 5\n5\n1 2 3 4 5\n1 2\n1 3\n3 4\n4 5\n4\n2 2 2 2\n3 1\n3 2\n3 4\n","output":"1\n3\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPoschitaiPuti"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(
        |f, vert: usize, prev: usize| -> DefaultHashMap<usize, usize> {
            let mut res = DefaultHashMap::new();
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let mut call = f.call(e.to(), vert);
                ans += call[c[vert]];
                call.remove(&c[vert]);
                if call.len() > res.len() {
                    swap(&mut call, &mut res);
                }
                for (k, v) in call {
                    ans += res[k] * v;
                    res[k] += v;
                }
            }
            res[c[vert]] = 1;
            res
        },
    );
    dfs.call(0, n);
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
