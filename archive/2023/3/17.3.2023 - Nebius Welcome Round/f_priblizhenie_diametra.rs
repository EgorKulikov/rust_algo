//{"name":"F. Приближение диаметра","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"9 10 8\n1 2\n2 3\n2 4\n3 5\n4 5\n5 6\n5 7\n6 8\n7 8\n8 9\n3 4\n6 7\n2 8\n1 9\n1 6\n4 9\n3 9\n7 1\n","output":"10 6 5 6 2 4 2 2 1\n"},{"input":"8 7 9\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n1 5\n3 7\n2 4\n4 6\n6 8\n8 2\n5 4\n2 4\n3 3\n1 652997 124613 653029 653029 124613 124613 124613 648901 124613 653029\n","output":"7 5 4 4 4 3 3 3 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPriblizhenieDiametra"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(m + q).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdgeWithId::new(v));
    }
    let mut a = vec![0; q + 1];
    let mut d = vec![0; n];
    let mut queue = VecDeque::with_capacity(n);
    let mut g = |lim: usize| -> usize {
        d.fill(n);
        queue.push_back(0);
        let mut ans = 0;
        d[0] = 0;
        while let Some(u) = queue.pop_front() {
            for e in &graph[u] {
                if e.id() >= lim + m {
                    break;
                }
                let v = e.to();
                if d[v] == n {
                    d[v] = d[u] + 1;
                    queue.push_back(v);
                    ans = d[v];
                }
            }
        }
        ans
    };
    a[0] = g(0);
    a[q] = g(q);
    let a0 = a[0];
    let aq = a[q];
    let mut rec = RecursiveFunction4::new(|f, l, l_res, r, r_res| {
        if r_res * 2 >= l_res || l + 1 == r {
            for i in l..r {
                a[i] = l_res;
            }
            return;
        }
        let m = (l + r) / 2;
        let m_res = g(m);
        a[m] = m_res;
        f.call(l, l_res, m, m_res);
        f.call(m, m_res, r, r_res);
    });
    rec.call(0, a0, q, aq);
    out_line!(a);
}

pub(crate) fn run(mut input: Input) -> bool {
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
}
//END MAIN
