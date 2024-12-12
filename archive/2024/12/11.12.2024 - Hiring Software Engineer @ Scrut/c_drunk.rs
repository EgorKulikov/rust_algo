//{"name":"C - Drunk","group":"LightOJ","url":"https://lightoj.com/contest/scrutcoding/arena/problem/6492","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\nsoda wine\nwater wine\n3\nsoda wine\nwater wine\nwine water\n","output":"Case 1: Yes\nCase 2: No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDrunk"}}}

use algo_lib::collections::id::Id;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let edges = input.read_vec::<(Str, Str)>(m);

    let mut id = Id::new();
    for (a, b) in &edges {
        id.get(a);
        id.get(b);
    }
    let mut graph = Graph::new(id.len());
    for (a, b) in &edges {
        graph.add_edge(Edge::new(id.get(a), id.get(b)));
    }
    out.print_line((
        format!("Case {}:", test_case),
        graph.topological_sort().is_some(),
    ));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
