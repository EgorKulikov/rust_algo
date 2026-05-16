//{"name":"l_labyrinth","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l_labyrinth"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let s = input.read_usize() - 1;
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        if v != s {
            graph.add_edge(u, Edge::new(v));
        }
    }
    let mut color = vec![0; n];
    static mut CUR_COLOR: i32 = 1;
    let mut last = vec![0; n];

    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Option<(usize, usize)> {
            if color[vert] != 0 {
                if color[vert] == unsafe { CUR_COLOR } {
                    None
                } else {
                    Some((vert, prev))
                }
            } else {
                color[vert] = unsafe { CUR_COLOR };
                last[vert] = prev;
                for e in &graph[vert] {
                    match f.call(e.to(), vert) {
                        None => {}
                        Some(res) => {
                            return Some(res);
                        }
                    }
                }
                None
            }
        });
    for e in &graph[s] {
        match dfs.call(e.to(), s) {
            None => unsafe {
                CUR_COLOR += 1;
            },
            Some((t, pt)) => {
                let get_path = |path: &mut Vec<usize>, mut vert: usize| {
                    while vert != s {
                        path.push(vert + 1);
                        vert = last[vert];
                    }
                    path.push(s + 1);
                    path.reverse();
                };
                let mut path1 = Vec::new();
                get_path(&mut path1, t);
                let mut path2 = vec![t + 1];
                get_path(&mut path2, pt);
                out_line!("Possible");
                out_line!(path1.len());
                out_line!(path1);
                out_line!(path2.len());
                out_line!(path2);
                return;
            }
        }
    }
    out_line!("Impossible");
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
