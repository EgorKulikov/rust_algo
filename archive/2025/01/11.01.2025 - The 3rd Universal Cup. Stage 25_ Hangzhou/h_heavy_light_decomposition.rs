//{"name":"H. Heavy-light Decomposition","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9733","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n12 5\n1 5\n9 11\n7 8\n6 6\n12 12\n4 3\n1 1\n4 4\n2 3\n2 2\n1 1\n2 2\n","output":"0 1 2 3 4 1 1 7 1 9 10 1\n2 0 2 2\nIMPOSSIBLE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHeavyLightDecomposition"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut chains = input.read_size_pair_vec(k).dec();

    chains.sort_unstable_by_key(|&(x, y)| Reverse(y - x));
    let mut ans = vec![0; n];
    let mut heavy = vec![n; n];
    let base = chains[0].0;
    let base_len = chains[0].1 - chains[0].0 + 1;
    let mut graph = Graph::new(n);
    for (x, y) in chains {
        let cur_len = y - x + 1;
        if x != base {
            let par = (base + base_len - cur_len).max(base + 1);
            ans[x] = par;
            graph.add_edge(Edge::new(par - 1, x));
        }
        for i in x + 1..=y {
            ans[i] = i;
            heavy[i - 1] = i;
            graph.add_edge(Edge::new(i - 1, i));
        }
    }
    let mut size = vec![0; n];
    let mut ok = true;
    let mut dfs = RecursiveFunction::new(|dfs, vert: usize| {
        size[vert] = 1;
        let mut max = 0;
        for e in &graph[vert] {
            let call = dfs.call(e.to());
            max.maxim(call);
            size[vert] += call;
        }
        if heavy[vert] != n && size[heavy[vert]] < max || heavy[vert] == n && max > 0 {
            ok = false;
        }
        size[vert]
    });
    dfs.call(base);
    if !ok {
        out.print_line("IMPOSSIBLE");
        return;
    }
    out.print_line(ans);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
