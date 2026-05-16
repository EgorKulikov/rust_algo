//{"name":"C. Walking On A Cube","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 0\n3 1 2\n","output":"5\n"},{"input":"4\n1 4 2\n0 1 3\n","output":"5\n"},{"input":"28000\n3297 0 2562\n11770 28000 7658\n","output":"46693\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let p1 = input.read_long_vec(3);
    let p2 = input.read_long_vec(3);

    for i in 0..3 {
        if p1[i] == 0 && p2[i] == 0 || p1[i] == n && p2[i] == n {
            let mut ans = 0;
            for j in 0..3 {
                ans += (p1[j] - p2[j]).abs();
            }
            out.print_line(ans);
            return;
        }
    }
    let mut pts = vec![p1.clone(), p2.clone()];
    for p in [p1, p2] {
        for i in 0..3 {
            let mut pp = p.clone();
            for x in [0, n] {
                pp[(i + 1) % 3] = x;
                for y in [0, n] {
                    pp[(i + 2) % 3] = y;
                    pts.push(pp.clone());
                }
            }
        }
    }
    let mut graph = Graph::new(pts.len());
    for i in pts.indices() {
        for j in 0..i {
            let mut diff = 0;
            let mut sum = 0;
            for k in 0..3 {
                if pts[i][k] != pts[j][k] {
                    diff += 1;
                    sum += (pts[i][k] - pts[j][k]).abs();
                }
            }
            if diff <= 1 {
                graph.add_edge(BiWeightedEdge::new(i, j, sum));
            }
        }
    }
    out.print_line(graph.distance(0, 1).unwrap().0);
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
