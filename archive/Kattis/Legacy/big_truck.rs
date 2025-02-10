//{"name":"Big Truck","group":"Kattis","url":"https://open.kattis.com/problems/bigtruck","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 1 2 3 1 0\n7\n1 2 2\n2 3 3\n3 6 4\n1 4 4\n4 3 2\n4 5 3\n5 6 2\n","output":"9 5\n"},{"input":"9\n1 1 1 1 1 1 1 1 1\n10\n1 2 3\n2 5 3\n1 6 2\n6 7 2\n7 5 2\n5 3 1\n3 4 2\n4 9 3\n5 8 2\n8 9 4\n","output":"12 7\n"},{"input":"2\n5 5\n0\n","output":"impossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_size_vec(n);
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let graph = Graph::new(n).do_with(|graph| {
        for (u, v, w) in edges {
            graph.add_edge(BiWeightedEdge::new(u, v, w));
        }
    });
    let d0 = graph.distances_from(0);
    if d0[n - 1].is_none() {
        out.print_line("impossible");
        return;
    }
    let target = d0[n - 1].unwrap().0;
    let dn = graph.distances_from(n - 1);
    let order = (0..n)
        .filter(|i| d0[*i].is_some())
        .collect::<Vec<_>>()
        .sorted_by_key(|i| d0[*i].unwrap().0);
    let mut ans = vec![0; n];
    ans[0] = t[0];
    for i in order.iter_skip(1) {
        if d0[i].unwrap().0 + dn[i].unwrap().0 != target {
            continue;
        }
        for e in &graph[i] {
            if d0[e.to()].unwrap().0 + e.weight() == d0[i].unwrap().0 {
                let cand = ans[e.to()] + t[i];
                ans[i].maxim(cand);
            }
        }
    }
    out.print_line((target, ans[n - 1]));
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
//END MAIN
