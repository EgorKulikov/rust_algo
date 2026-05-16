//{"name":"D. Druid","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 2\n2 1\n","output":"1 2\n"},{"input":"3\n3 1 2\n2 1\n3 2\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDruid"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let v = input.read_usize_vec(n);
    let edges = input.read_vec::<(usize, usize)>(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (a, b) in edges {
        graph.add_edge(a, BiEdge::new(b));
    }
    let mut ends = Vec::new();
    let mut dsu = DSU::new(n);
    for (i, &a) in v.iter().enumerate() {
        for e in graph[i].iter() {
            if v[e.to()] == a {
                dsu.join(i, e.to());
            }
        }
        if a == n {
            let mut cur_deg = 0;
            for e in graph[i].iter() {
                if v[e.to()] == n {
                    cur_deg += 1;
                }
            }
            if cur_deg <= 1 {
                ends.push(i);
            }
        }
    }
    let mut c = v.clone();
    c.sort_unstable();
    c.dedup();
    if ends.len() > 2 || c.len() != dsu.count() {
        out_line!(-1);
        return;
    }
    let mut free = BTreeSet::new();
    let mut at = 0;
    for i in 1..=n {
        if at < c.len() && i == c[at] {
            at += 1;
            continue;
        }
        free.insert(i);
    }
    assert_eq!(c.len() + free.len(), n);
    let mut ans = None;
    for i in ends {
        let mut exact = BitSet::new(n);
        let mut dfs = RecursiveFunction2::new(|f, vert, prev| -> bool {
            let mut cur_exact = true;
            for e in &graph[vert] {
                if e.to() != prev {
                    if !f.call(e.to(), vert) {
                        return false;
                    }
                    if v[e.to()] == v[vert] {
                        if !cur_exact {
                            return false;
                        }
                        cur_exact = false;
                    }
                    if v[e.to()] > v[vert] {
                        return false;
                    }
                }
            }
            exact.set(vert, cur_exact);
            true
        });
        if !dfs.call(i, i) {
            continue;
        }
        let mut free = free.clone();
        let mut c_ans = vec![0; n];
        let mut good = true;
        for j in (0..n).rev() {
            if exact[j] {
                c_ans[j] = v[j];
            } else {
                let prefix = free.range(..v[j]);
                match prefix.last() {
                    None => {
                        good = false;
                        break;
                    }
                    Some(&v) => {
                        free.remove(&v);
                        c_ans[j] = v;
                    }
                }
            }
        }
        if good {
            ans.minim(c_ans);
        }
    }
    out_line!(ans);
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
