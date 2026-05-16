//{"name":"Power Tree","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/POWTREE","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 87\n18 16 14 16\n3 1\n3 4\n4 2\n5 34\n12 13 15 18 16\n1 4\n1 3\n5 4\n3 2\n6 98\n1 18 9 10 10 14\n2 1\n5 3\n6 4\n6 3\n1 6\n","output":"6\n0\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PowerTree"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::{Bounds, IncDec};
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use std::cmp::Reverse;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut x = input.read_long();
    let mut a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> i64 {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            a[vert].maxim(f.call(e.to(), vert));
        }
        a[vert]
    });
    dfs.call(0, n);
    let base: i64 = a.iter().sum();
    if base >= x {
        out_line!(0);
        return;
    }
    x -= base;
    let mut v = Vec::new();
    let mut ft = FenwickTree::new(n);
    let mut ans = None;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        ft.add(v.len(), a[vert]);
        v.push(Reverse(a[vert]));
        let mut left = 0;
        let mut right = x;
        while left < right {
            let mid = (left + right) / 2;
            let pos = v.lower_bound(&Reverse(a[vert] + mid));
            let cur = (a[vert] + mid) * (v.len() - pos) as i64 - ft.get(pos..);
            if cur >= x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        ans.minim(left);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
        }
        v.pop();
        ft.add(v.len(), -a[vert]);
    });
    dfs.call(0, n);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
