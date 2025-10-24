//{"name":"Tree !","group":"Eolymp - Basecamp - Blitz Round #4","url":"https://eolymp.com/en/compete/7mk1e6onrl4pb69590dkne46j4/problem/4","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 2\n4\n1 2\n2 3\n3 4\n4\n1 2\n2 3\n2 4\n","output":"1\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let leaves = (0..n).filter(|&i| graph[i].len() == 1).count();
    let mut ans = 0;
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, degree_up: usize| {
        let mut cur = (graph[vert].len() == 1) as usize;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            cur += f.call(e.to(), vert, graph[vert].len());
        }
        if degree_up != usize::MAX {
            let mut up = leaves - cur;
            if degree_up == 1 {
                up -= 1;
            } else if degree_up == 2 {
                up += 1;
            }
            let mut down = cur;
            if graph[vert].len() == 1 {
                down -= 1;
            } else if graph[vert].len() == 2 {
                down += 1;
            }
            if up == down {
                ans += 1;
            }
        }
        cur
    });
    dfs.call(0, n, usize::MAX);
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
