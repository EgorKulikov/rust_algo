//{"name":"E. Запросы частот","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 3\n1 1 1\n1 2\n3 1 1\n3 1 2\n3 2 1\n5 5\n1 2 1 1 2\n1 1 2 2\n3 1 1\n2 1 2\n4 1 1\n4 2 1\n4 2 2\n","output":"1 -1 1\n1 1 2 1 -1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZaprosiChastot"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::{out, out_line};
use std::collections::HashSet;
use std::thread;

fn solve(input: &mut Input, _test_case: usize) {
    let t = input.read();
    let mut tests = Vec::new();
    for _ in 0..t {
        let n = input.read();
        let q = input.read();
        let a: Vec<i32> = input.read_vec(n);
        let p = input.read_vec::<usize>(n - 1).dec_by_one();
        let queries: Vec<(usize, usize, usize)> = input.read_vec(q);
        tests.push((n, q, a, p, queries));
    }
    thread::Builder::new()
        .stack_size(400000000)
        .spawn(move || {
            for (n, q, a, p, queries) in tests {
                let mut by_vertex = vec![Vec::new(); n];
                for (i, (v, _, _)) in queries.iter().enumerate() {
                    by_vertex[*v - 1].push(i);
                }
                let mut graph = Graph::new(n);
                for i in 1..n {
                    graph.add_edge(p[i - 1], Edge::new(i));
                }
                let mut ans = vec![0; q];
                let mut set = TreapSet::new();
                let mut qty = vec![0isize; n + 1];
                let mut once = HashSet::new();
                let mut dfs = RecursiveFunction::new(|f, vert: usize| {
                    let was = qty[a[vert] as usize];
                    if was < -1 {
                        set.remove(&(was, a[vert]));
                    } else if was == -1 {
                        once.remove(&a[vert]);
                    }
                    qty[a[vert] as usize] -= 1;
                    if was < 0 {
                        set.insert((was - 1, a[vert]));
                    } else {
                        once.insert(a[vert]);
                    }
                    for i in by_vertex[vert].drain(..) {
                        let (_, l, mut k) = queries[i];
                        if l == 1 {
                            if k <= once.len() {
                                ans[i] = *once.iter().next().unwrap();
                                continue;
                            }
                            k -= once.len();
                        }
                        let qt = set.lower_bound(&(-(l as isize), (n + 1) as i32));
                        if k > qt {
                            ans[i] = -1;
                        } else {
                            let (_, a) = set.get_at(qt - k).unwrap();
                            ans[i] = *a;
                        }
                    }
                    for e in graph[vert].iter() {
                        f.call(e.to());
                    }
                    if was < 0 {
                        set.remove(&(was - 1, a[vert]));
                    } else {
                        once.remove(&a[vert]);
                    }
                    if was < -1 {
                        set.insert((was, a[vert]));
                    } else if was == -1 {
                        once.insert(a[vert]);
                    }
                    qty[a[vert] as usize] += 1;
                });
                dfs.call(0);
                out_line!(ans);
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
