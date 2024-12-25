//{"name":"Human Cannonball Run","group":"Kattis","url":"https://open.kattis.com/problems/humancannonball","interactive":false,"timeLimit":1000,"tests":[{"input":"25.0 100.0\n190.0 57.5\n4\n125.0 67.5\n75.0 125.0\n45.0 72.5\n185.0 102.5\n","output":"19.984901\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HumanCannonballRun"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::point::Point;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s: Point<Real> = input.read();
    let e: Point<Real> = input.read();
    let n = input.read_size();
    let p: Vec<Point<Real>> = input.read_vec(n).do_with(|p| {
        p.push(s);
        p.push(e);
    });

    let mut graph = Graph::new(n + 2);
    for i in 0..n + 2 {
        for j in 0..n + 2 {
            if i != j {
                let dist = p[i].dist_point(p[j]);
                let mut cur: Real = dist / 5;
                if i < n {
                    cur.minim(2 + Real::abs(dist - 50) / 5);
                }
                graph.add_edge(WeightedEdge::new(i, j, cur));
            }
        }
    }
    out.print_line(graph.distance(n, n + 1).unwrap().0);
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
