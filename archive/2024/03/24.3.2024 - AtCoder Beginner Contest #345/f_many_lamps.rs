//{"name":"F - Many Lamps","group":"AtCoder - Monoxer Programming Contest 2024（AtCoder Beginner Contest 345）","url":"https://atcoder.jp/contests/abc345/tasks/abc345_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 4\n1 2\n1 3\n2 4\n3 5\n1 5\n","output":"Yes\n3\n3 4 5\n"},{"input":"5 5 5\n1 2\n1 3\n2 4\n3 5\n1 5\n","output":"No\n"},{"input":"10 10 6\n2 5\n2 6\n3 5\n3 8\n4 6\n4 8\n5 9\n6 7\n6 10\n7 9\n","output":"Yes\n3\n10 9 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FManyLamps"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut k = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    out.set_bool_output(BoolOutput::YesNo);
    if k % 2 == 1 {
        out.print_line(false);
        return;
    }
    let mut dsu = DSU::new(n);
    let mut graph = Graph::new(n);
    for (i, (u, v)) in edges.into_iter().enumerate() {
        if dsu.join(u, v) {
            graph.add_edge(BiEdge::with_payload(u, v, i));
        }
    }
    let mut edges = BitSet::new(m);
    let mut max_possible = 0;
    let ok = k;
    for i in 0..n {
        if k == 0 {
            break;
        }
        if dsu.get(i) != i {
            continue;
        }
        max_possible += dsu.size(i) / 2 * 2;
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> bool {
            let mut enabled = false;
            let mut to_switch = 0;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                if !f.call(e.to(), vert) {
                    if !enabled {
                        enabled = true;
                        to_switch = *e.payload();
                    } else {
                        enabled = false;
                        edges.set(*e.payload());
                        edges.set(to_switch);
                        k -= 2;
                    }
                }
                if k == 0 {
                    return true;
                }
            }
            if enabled {
                k -= 2;
                edges.set(to_switch);
            }
            enabled
        });
        dfs.call(i, i);
    }
    if k != 0 {
        assert!(max_possible < ok);
        out.print_line(false);
        return;
    }
    out.print_line(true);
    let ans = edges.into_iter().collect_vec().inc();
    out.print_line(ans.len());
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
    //    tester::stress_test();
}
//END MAIN
