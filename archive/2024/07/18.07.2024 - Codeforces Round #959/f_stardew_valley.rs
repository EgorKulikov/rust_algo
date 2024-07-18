//{"name":"F. Stardew Valley","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2\n1 2 1\n2 3 1\n3 3\n1 2 1\n1 3 1\n2 3 0\n5 9\n1 2 0\n5 2 1\n5 4 1\n5 1 1\n2 3 1\n5 2 1\n4 1 0\n4 3 0\n5 2 0\n","output":"NO\nYES\n3\n1 2 3 1\nYES\n7\n1 2 5 4 3 2 5 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FStardewValley"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable, Callable2, RecursiveFunction, RecursiveFunction2,
};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, u8)>(m).dec();

    let mut deg = vec![0; n];
    let mut dsu = DSU::new(n);
    let mut graph = vec![Vec::new(); n];
    let mut extra_graph = Graph::new(n);
    for &(u, v, t) in &edges {
        if t == 0 {
            if dsu.join(u, v) {
                extra_graph.add_edge(BiEdge::new(u, v));
            }
        } else {
            graph[u].push(v);
            graph[v].push(u);
            deg[u] += 1;
            deg[v] += 1;
        }
    }
    for i in 0..n {
        if dsu.get(i) == i {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> bool {
                let mut res = deg[vert] % 2 == 1;
                for e in &extra_graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    if f.call(e.to(), vert) {
                        res ^= true;
                        graph[vert].push(e.to());
                        graph[e.to()].push(vert);
                    }
                }
                res
            });
            if dfs.call(i, i) {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(true);
    let mut ans = Vec::new();
    let mut removed = vec![DefaultHashMap::<_, usize>::new(); n];
    let mut fep = RecursiveFunction::new(|rec, v: usize| {
        while let Some(to) = graph[v].pop() {
            if removed[v][to] != 0 {
                removed[v][to] -= 1;
            } else {
                removed[to][v] += 1;
                rec.call(to);
            }
        }
        ans.push(v + 1);
    });
    fep.call(0);
    out.print_line(ans.len() - 1);
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
}
//END MAIN
