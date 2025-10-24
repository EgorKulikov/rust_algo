//{"name":"H. Shortcut on Tree","group":"Universal Cup - The 3rd Universal Cup Semifinals","url":"https://contest.ucup.ac/contest/2506/problem/14021","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 2\n5\n1 1 2 2\n","output":"Yes\n1\n3 1\nYes\n5\n1 4\n4 1\n3 3\n3 1\n5 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();

    let mut graph = Graph::new(n);
    for i in 1..n {
        graph.add_edge(Edge::new(p[i - 1], i));
    }
    let mut has_down = BitSet::new(n);
    let mut has_up = BitSet::new(n);
    let order = graph.topological_sort().unwrap();
    let mut ans = Vec::new();
    for v in order.copy_skip(1).rev() {
        let mut found_down = false;
        let mut found_not_up = false;
        for e in &graph[v] {
            if has_down[e.to()] {
                found_down = true;
            }
            if !has_up[e.to()] {
                found_not_up = true;
            }
        }
        if !found_down {
            has_down.set(v);
            ans.push((v + 1, 1));
        }
        if found_not_up {
            has_up.set(v);
            ans.push((1, v + 1));
        }
    }
    assert!(ans.len() <= n);
    out.print_line(true);
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
