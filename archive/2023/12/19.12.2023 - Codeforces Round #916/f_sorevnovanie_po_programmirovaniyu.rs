//{"name":"F. Соревнование по программированию","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4\n1 2 1\n2\n1\n5\n5 5 5 1\n7\n1 2 1 1 3 3\n7\n1 1 3 2 2 4\n7\n1 2 1 1 1 3\n","output":"1\n0\n1\n3\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSorevnovaniePoProgrammirovaniyu"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();

    let edges = p.into_iter().zip(1..n).collect_vec();
    let graph = Graph::from_biedges(n, &edges);
    let mut sz = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        sz[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            sz[vert] += f.call(e.to(), vert);
        }
        sz[vert]
    });
    dfs.call(0, n);
    let mut ans = 0;
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, mut has: usize| {
        if has > 0 {
            ans += 1;
            has -= 1;
        }
        let mut sizes = vec![(has, prev)];
        let mut total = has;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            sizes.push((sz[e.to()], e.to()));
            total += sz[e.to()];
        }
        sizes.sort();
        sizes.reverse();
        if sizes[0].0 * 2 <= total {
            ans += total / 2;
        } else {
            f.call(sizes[0].1, vert, total - sizes[0].0);
        }
    });
    dfs.call(0, n, 0);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
