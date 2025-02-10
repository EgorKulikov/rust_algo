//{"name":"Running MoM","group":"Kattis","url":"https://open.kattis.com/problems/runningmom","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nArlington San_Antonio\nSan_Antonio Baltimore\nBaltimore New_York\nNew_York Dallas\nBaltimore Arlington\nSan_Antonio\nBaltimore\nNew_York\n","output":"San_Antonio safe\nBaltimore safe\nNew_York trapped\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::id::Id;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges: Vec<(Str, Str)> = input.read_vec(n);

    let mut id = Id::new();
    id.add_pairs(edges.iter().cloned());
    let graph = Graph::new(id.len()).do_with(|graph| {
        for (a, b) in edges {
            let a = id.get(a);
            let b = id.get(b);
            graph.add_edge(Edge::new(a, b));
        }
    });
    let scc = graph.strongly_connected_components();
    let q = scc.color.qty_bound(scc.condensed.vertex_count());
    let mut good = BitSet::new(scc.condensed.vertex_count());
    for i in (0..scc.condensed.vertex_count()).rev() {
        if q[i] > 1 {
            good.set(i);
            continue;
        }
        for e in &scc.condensed[i] {
            if good[e.to()] {
                good.set(i);
                break;
            }
        }
    }

    for s in input.iter_str() {
        let id = id.get(s.clone());
        if good[scc.color[id]] {
            output!(out, "{} safe", s);
        } else {
            output!(out, "{} trapped", s);
        }
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
