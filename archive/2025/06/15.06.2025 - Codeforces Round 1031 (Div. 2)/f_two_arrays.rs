//{"name":"F. Two Arrays","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 2 4 4 4\n1 3 3 5 2\n7\n2 2 4 4 5 5 5\n1 3 3 2 1 6 6\n7\n12 3 3 4 5 6 4\n1 2 13 8 10 13 7\n","output":"9\n1 3 4 5 2\n1 2 3 4 4\n12\n2 3 4 2 1 5 6\n1 2 3 4 5 6 5\n14\n12 3 13 8 10 6 4\n1 2 3 4 5 13 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n).dec();
    let mut b = input.read_size_vec(n).dec();

    let mut graph = Graph::new(2 * n);
    for i in 0..n {
        graph.add_edge(BiEdgeWithId::new(a[i], b[i]));
    }
    let mut visited = BitSet::new(2 * n);
    let mut visited_edge = BitSet::new(n);
    for i in 0..2 * n {
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, mut rev: bool| {
            if visited[vert] {
                return;
            }
            visited.set(vert);
            for e in &graph[vert] {
                let id = e.id();
                if visited_edge[id] {
                    continue;
                }
                visited_edge.set(id);
                if (b[id] == vert) ^ rev {
                    swap(&mut a[id], &mut b[id]);
                }
                f.call(e.to(), vert, rev);
                if prev == 2 * n {
                    rev = !rev;
                }
            }
        });
        dfs.call(i, 2 * n, false);
    }
    fn num_unique(a: &[usize]) -> usize {
        let mut set = BitSet::new(2 * a.len());
        for &x in a {
            set.set(x);
        }
        set.count_ones()
    }
    out.print_line(num_unique(&a) + num_unique(&b));
    out.print_line(a.inc());
    out.print_line(b.inc());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
