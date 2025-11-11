//{"name":"Jumps","group":"CSES - Datatähti 2026 alku","url":"https://cses.fi/637/task/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4 6 5\n.*.***\n*...**\n*****.\n*..*.*\n1 1 1 3\n2 2 2 2\n1 1 4 5\n4 5 2 4\n3 6 2 2\n","output":"1\n0\n3\n3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::all_distances::{AllDistances, Distance};
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let g = input.read_char_table(n, m);

    let mut graph = Graph::new(n + m);
    for (i, j) in g.indices() {
        if g[(i, j)] == b'.' {
            graph.add_edge(BiWeightedEdge::new(i, n + j, 1));
        }
    }
    let d = graph.all_distances();

    for _ in 0..q {
        let r1 = input.read_size() - 1;
        let c1 = input.read_size() - 1;
        let r2 = input.read_size() - 1;
        let c2 = input.read_size() - 1;
        if r1 == r2 && c1 == c2 {
            out.print_line(0);
            continue;
        }
        let mut ans = Distance::None;
        for i in [r1, n + c1] {
            for j in [r2, n + c2] {
                ans.minim(d[(i, j)]);
            }
        }
        match ans {
            Distance::Infinite => unreachable!(),
            Distance::Finite(x) => {
                out.print_line(x + 1);
            }
            Distance::None => {
                out.print_line(-1);
            }
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
