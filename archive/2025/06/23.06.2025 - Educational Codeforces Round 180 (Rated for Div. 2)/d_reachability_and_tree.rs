//{"name":"D. Reachability and Tree","group":"Codeforces - Educational Codeforces Round 180 (Rated for Div. 2)","url":"https://codeforces.com/contest/2112/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n1 2\n2 4\n1 3\n3 5\n5\n1 2\n1 3\n1 4\n4 5\n2\n2 1\n4\n3 1\n1 2\n2 4\n","output":"YES\n1 2\n3 1\n3 5\n4 2\nYES\n2 1\n3 1\n4 1\n5 4\nNO\nYES\n1 3\n2 1\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let d = graph.degrees();
    let pos = d.iter().position(|&x| x == 2);
    if let Some(pos) = pos {
        let mut dir = false;
        let mut ans = Vec::new();
        for e in &graph[pos] {
            let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, dir: bool| {
                if dir {
                    ans.push((vert + 1, prev + 1));
                } else {
                    ans.push((prev + 1, vert + 1));
                }
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    f.call(e.to(), vert, !dir);
                }
            });
            dfs.call(e.to(), pos, dir);
            dir = !dir;
        }
        out.print_line(true);
        out.print_per_line(&ans);
    } else {
        out.print_line(false);
    }
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
