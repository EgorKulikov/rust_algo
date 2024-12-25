//{"name":"Bike Racers","group":"HackerRank - Algorithms - Search","url":"https://www.hackerrank.com/challenges/bike-racers/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"3 3 2\n0 1\n0 2\n0 3\n100 1\n200 2\n300 3\n","output":"40000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BikeRacers"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::geometry::point::Point;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::max_flow::MaxFlow;
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
    let p1: Vec<Point<i64>> = input.read_vec(n);
    let p2: Vec<Point<i64>> = input.read_vec(m);

    let mut edges = Vec::with_capacity(n * m);
    for i in 0..n {
        for j in 0..m {
            let dist = p1[i].square_dist_point(p2[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort();
    let mut left = k;
    let mut right = n * m;
    while left < right {
        let mid = (left + right) / 2;
        let mut graph = Graph::new(n + m + 2);
        let source = n + m;
        let sink = n + m + 1;
        for i in 0..n {
            graph.add_edge(FlowEdge::new(source, i, 1));
        }
        for i in 0..m {
            graph.add_edge(FlowEdge::new(n + i, sink, 1));
        }
        for (_, i, j) in edges.copy_take(mid) {
            graph.add_edge(FlowEdge::new(i, n + j, 1));
        }
        if graph.max_flow(source, sink) >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(edges[left - 1].0);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
