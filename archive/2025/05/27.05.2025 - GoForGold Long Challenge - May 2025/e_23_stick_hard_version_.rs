//{"name":"E. 23 Stick (Hard Version)","group":"Codeforces - GoForGold Long Challenge - May 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/611602/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n5\n","output":"1\n3 5 2 4 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::euler_path::EulerPath;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n <= 2 {
        out.print_line(1);
        return;
    }
    if n == 3 {
        out.print_line([1, 3]);
        return;
    }
    let mut graph = Graph::new(n);
    for i in 0..n - 2 {
        graph.add_edge(BiEdgeWithId::new(i, i + 2));
    }
    for i in 0..n - 3 {
        graph.add_edge(BiEdgeWithId::new(i, i + 3));
    }
    out.print_line(graph.euler_path().unwrap().inc());
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
