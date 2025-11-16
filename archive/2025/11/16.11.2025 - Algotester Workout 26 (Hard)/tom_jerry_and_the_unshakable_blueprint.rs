//{"name":"Tom, Jerry and the Unshakable Blueprint","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/147297","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2\n2 3\n3 1\n","output":"Yes\n"},{"input":"4 4\n1 2\n2 3\n3 1\n1 4\n","output":"No\n"},{"input":"10 11\n1 2\n2 3\n3 1\n4 5\n5 6\n6 4\n1 4\n2 7\n3 8\n5 9\n6 10\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let bridges = graph.bridges().into_iter().collect::<FxHashSet<_>>();
    let mut dsu = DSU::new(n);
    for (u, v) in edges.copy_iter() {
        if !bridges.contains(&(u, v)) && !bridges.contains(&(v, u)) {
            dsu.union(u, v);
        }
    }
    let mut qty = vec![0; n];
    let mut degrees = vec![DefaultHashMap::new(Vec::new()); n];
    for i in 0..n {
        degrees[dsu.find(i)][graph[i].len()].push(i);
    }
    let mut e = vec![Vec::new(); n];
    for (u, v) in edges.copy_iter() {
        if !bridges.contains(&(u, v)) && !bridges.contains(&(v, u)) {
            qty[dsu.find(u)] += 1;
            e[dsu.find(u)].push((u, v));
        }
    }
    let mut color = vec![0; n];
    for i in 0..n {
        if dsu.find(i) == i && dsu.size(i) != 1 {
            if qty[i] != dsu.size(i) {
                out.print_line(false);
                return;
            }
            if degrees[i].len() > 2 {
                out.print_line(false);
                return;
            }
            if degrees[i].len() == 2 {
                let v = degrees[i].values().cloned().collect::<Vec<_>>();
                if v[0].len() != v[1].len() {
                    out.print_line(false);
                    return;
                }
                for i in v[0].copy_iter() {
                    color[i] = 1;
                }
                for (u, v) in e[i].copy_iter() {
                    if color[u] == color[v] {
                        out.print_line(false);
                        return;
                    }
                }
            }
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

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
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
