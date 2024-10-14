//{"name":"F. Выбери свои запросы","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3 4\n1 2\n3 2\n3 1\n1 2\n","output":"y+\nx+\nx-\ny-\n"},{"input":"4 4\n1 2\n2 3\n3 4\n3 2\n","output":"y+\ny+\nx-\ny-\n"},{"input":"4 2\n2 1\n4 3\n","output":"y+\nx+\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FViberiSvoiZaprosi"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(q).dec();

    let graph = Graph::new(n).do_with(|graph| {
        for (i, &(u, v)) in edges.enumerate() {
            graph.add_edge(BiEdge::with_payload(u, v, i));
        }
    });
    let mut selected = vec![0; q];
    let mut done = BitSet::new(n);
    let mut state = BitSet::new(n);
    let mut processed = BitSet::new(q);
    for i in 0..n {
        if done[i] {
            continue;
        }
        let mut dfs = RecursiveFunction::new(|f, vert: usize| -> bool {
            if done[vert] {
                state.flip(vert);
                return false;
            }
            done.set(vert);
            for e in &graph[vert] {
                if processed[*e.payload()] {
                    continue;
                }
                processed.set(*e.payload());
                if f.call(e.to()) {
                    state.flip(vert);
                    selected[*e.payload()] = vert;
                } else {
                    selected[*e.payload()] = e.to();
                }
            }
            if state[vert] {
                state.flip(vert);
                false
            } else {
                true
            }
        });
        dfs.call(i);
    }
    state.fill(false);
    for i in 0..q {
        out.print(if selected[i] == edges[i].0 { "x" } else { "y" });
        if state[selected[i]] {
            out.print_line("-");
        } else {
            out.print_line("+");
        }
        state.flip(selected[i]);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
