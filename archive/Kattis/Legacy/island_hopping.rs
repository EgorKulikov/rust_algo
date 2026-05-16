//{"name":"Island Hopping","group":"Kattis","url":"https://open.kattis.com/problems/islandhopping","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3\n0.0 0.0\n0.0 1.0\n1.0 0.0\n10\n30.0 38.0\n43.0 72.0\n47.0 46.0\n49.0 69.0\n52.0 42.0\n58.0 17.0\n73.0 7.0\n84.0 81.0\n86.0 75.0\n93.0 50.0\n","output":"2.000\n168.01015709273446\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::geometry::point::Point;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p: Vec<Point<Real>> = input.read_vec(n);

    let graph = Graph::new(n).do_with(|g| {
        for i in 0..n {
            for j in i + 1..n {
                let dist = p[i].dist_point(p[j]);
                g.add_edge(BiWeightedEdge::new(i, j, dist));
            }
        }
    });
    let tree = graph.minimal_spanning_tree();
    let mut ans = Real(0.);
    for i in 0..n {
        for e in &tree[i] {
            if e.to() < i {
                ans += e.weight();
            }
        }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
