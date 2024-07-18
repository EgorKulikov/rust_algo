//{"name":"ucup21f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup21f"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut visited = vec![0; n];
    let mut dsu = DSU::new(n);
    let mut cyc_len = vec![0; n];
    for i in 0..n {
        if visited[i] != 0 {
            continue;
        }
        let mut j = i;
        loop {
            if visited[j] != 0 {
                break;
            }
            visited[j] = 1;
            j = a[j];
        }
        if visited[j] == 1 {
            let mut k = j;
            let mut len = 0;
            loop {
                visited[k] = 2;
                dsu.join(k, a[k]);
                k = a[k];
                len += 1;
                if k == j {
                    break;
                }
            }
            cyc_len[dsu.get(j)] = len;
        }
        j = i;
        loop {
            if visited[j] != 2 {
                visited[j] = 2;
            } else {
                break;
            }
            j = a[j];
        }
    }
    let mut graph = Graph::new(n);
    for i in 0..n {
        if dsu.get(i) != dsu.get(a[i]) {
            graph.add_edge(Edge::new(dsu.get(a[i]), i));
        }
    }
    let mut paths = Vec::new();
    let mut cycles = Vec::new();
    for i in 0..n {
        if cyc_len[i] != 0 {
            let mut dfs = RecursiveFunction::new(|dfs, vert: usize| -> usize {
                if graph[vert].len() == 0 {
                    return 0;
                }
                let mut cur_paths = Vec::with_capacity(graph[vert].len());
                for edge in &graph[vert] {
                    cur_paths.push(dfs.call(edge.to()) + 1);
                }
                cur_paths.sort();
                cur_paths.reverse();
                paths.extend_from_slice(&cur_paths[1..]);
                cur_paths[0]
            });
            let len = dfs.call(i);
            cycles.push((cyc_len[i], len));
        }
    }
    let mut ans = Vec::with_capacity(n);
    cycles.sort();
    let mut at = 0;
    for i in 1.. {
        while at < cycles.len() && cycles[at].0 + cycles[at].1 < i {
            at += 1;
        }
        if at == cycles.len() {
            break;
        }
        if i >= cycles[at].0 {
            ans.push(0);
        } else {
            ans.push(1);
        }
    }
    let mut cycles = cycles.into_iter().map(|(a, b)| a + b).collect_vec();
    cycles.sort();
    cycles.reverse();
    paths.extend_from_slice(&cycles[1..]);
    assert_eq!(ans.len(), cycles[0]);
    let mut cur = cycles[0];
    paths.sort();
    let mut cur_ans = 0;
    for i in ans.len() + 1..=n {
        if cur < i {
            cur += paths.pop().unwrap();
            cur_ans += 1;
        }
        ans.push(cur_ans);
    }
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
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
