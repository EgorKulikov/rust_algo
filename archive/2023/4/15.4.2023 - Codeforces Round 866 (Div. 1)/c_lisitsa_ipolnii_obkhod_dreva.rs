//{"name":"C. Лисица и полный обход древа","group":"Codeforces - Codeforces Round 866 (Div. 1)","url":"https://codeforces.com/contest/1819/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2\n1 3\n3 4\n3 5\n","output":"Yes\n4 5 1 2 3\n"},{"input":"3\n1 2\n1 3\n","output":"Yes\n1 2 3\n"},{"input":"15\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4 8\n4 9\n5 10\n5 11\n6 12\n6 13\n7 14\n7 15\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLisitsaIPolniiObkhodDreva"}}}

use algo_lib::collections::vec_ext::{ConsecutiveIter, IncDec};
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec_by_one();

    if n == 2 {
        out_line!(true);
        out_line!(1, 2);
        return;
    }
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut root = 0;
    for i in 0..n {
        if graph[i].len() > 1 {
            root = i;
            break;
        }
    }
    let mut big = Vec::new();
    let mut small = Vec::new();
    for e in &graph[root] {
        let v = e.to();
        if graph[v].len() == 1 {
            small.push(v);
        } else {
            big.push(v);
        }
    }
    if big.len() > 2 {
        out_line!(false);
        return;
    }
    let mut ans = Graph::new(n);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut last = prev;
        let mut big = None;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let v = e.to();
            if graph[v].len() == 1 {
                ans.add_edge(last, BiEdge::new(v));
                last = v;
            } else {
                if big.is_some() {
                    return false;
                }
                big = Some(v);
            }
        }
        if let Some(v) = big {
            ans.add_edge(last, BiEdge::new(v));
            f.call(v, vert)
        } else {
            ans.add_edge(last, BiEdge::new(vert));
            true
        }
    });
    for &v in &big {
        if !dfs.call(v, root) {
            out_line!(false);
            return;
        }
    }
    let mut connect = Vec::new();
    if big.len() >= 1 {
        connect.push(big[0]);
    } else {
        connect.push(root);
    }
    connect.append(&mut small);
    if big.len() >= 2 {
        connect.push(big[1]);
    } else {
        connect.push(root);
    }
    for (&u, &v) in connect.consecutive_iter() {
        ans.add_edge(u, BiEdge::new(v));
    }
    for i in 0..n {
        assert_eq!(ans[i].len(), 2, "{}", i);
    }
    let mut v = Vec::with_capacity(n);
    let mut cur = 0;
    let mut forb = ans[0][0].to();
    loop {
        v.push(cur + 1);
        for e in &ans[cur] {
            if e.to() != forb {
                forb = cur;
                cur = e.to();
                break;
            }
        }
        if cur == 0 {
            break;
        }
    }
    out_line!(true);
    out_line!(v);
}

pub(crate) fn run(mut input: Input) -> bool {
    set_bool_output(BoolOutput::YesNo);
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
