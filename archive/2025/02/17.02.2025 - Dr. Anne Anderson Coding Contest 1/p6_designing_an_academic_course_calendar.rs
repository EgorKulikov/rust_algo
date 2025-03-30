//{"name":"P6 - Designing An Academic Course Calendar","group":"DMOJ - Dr. Anne Anderson Coding Contest 1","url":"https://dmoj.ca/problem/daacc1p6","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n2\n1 2\n5 4\n3 1\n3 2\n4 2\n","output":"3\n3 5\n1 4\n2\n"},{"input":"9\n9\n3\n2 1 3\n4 6\n3 5\n6 5\n6 7\n5 1\n7 1\n6 2\n1 8\n5 9\n","output":"4\n3 4\n6\n2 5 7\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let need = input.read_size_vec(k).dec();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let order = graph.topological_sort().unwrap();
    let mut will_take = BitSet::new(n);
    for i in need.copy_iter() {
        will_take.set(i);
    }
    for i in order.copy_rev() {
        for e in &graph[i] {
            if will_take[e.to()] {
                will_take.set(i);
                break;
            }
        }
    }
    let mut ans = Vec::new();
    let mut level = vec![0; n];
    for i in order {
        if !will_take[i] {
            continue;
        }
        if ans.len() == level[i] {
            ans.push(Vec::new());
        }
        ans[level[i]].push(i + 1);
        for e in &graph[i] {
            let cand = level[i] + 1;
            level[e.to()].maxim(cand);
        }
    }
    for row in &mut ans {
        row.sort();
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
