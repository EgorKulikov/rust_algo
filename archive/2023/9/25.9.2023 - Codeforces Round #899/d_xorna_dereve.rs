//{"name":"D. XOR на дереве","group":"Codeforces - Codeforces Round 899 (Div. 2)","url":"https://codeforces.com/contest/1882/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n4\n3 2 1 0\n1 2\n2 3\n2 4\n1\n100\n","output":"8 6 12 10\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DXORNaDereve"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut v = vec![0; n];
    let mut m = vec![0; n];
    let mut global = 0;
    let mut dfs1 = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        v[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            v[vert] += v[e.to()];
            m[e.to()] += (n.into_i64() - 2 * v[e.to()]) * (a[vert] ^ a[e.to()]);
            global += v[e.to()] * (a[vert] ^ a[e.to()]);
        }
    });
    dfs1.call(0, n);
    let mut dfs2 = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            m[e.to()] += m[vert];
            f.call(e.to(), vert);
        }
    });
    dfs2.call(0, n);
    for i in &mut m {
        *i += global;
    }
    out_line!(m);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
