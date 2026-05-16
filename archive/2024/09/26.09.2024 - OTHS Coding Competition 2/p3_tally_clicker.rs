//{"name":"P3 - Tally Clicker","group":"DMOJ - OTHS Coding Competition 2","url":"https://dmoj.ca/problem/othscc2p3","interactive":false,"timeLimit":1000,"tests":[{"input":"9999\n","output":"9\n"},{"input":"101\n","output":"101\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3TallyClicker"}}}

use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut graph = Graph::new(10000);
    for i in 0..10000 {
        graph.add_edge(Edge::new(i, (i + 1) % 10000));
        let mut j = i;
        let mut ten = 1;
        for _ in 0..4 {
            let d = (i / ten) % 10;
            if d == 9 {
                j -= 9 * ten;
            } else {
                j += ten;
            }
            ten *= 10;
        }
        graph.add_edge(Edge::new(i, j));
    }
    out.print_line(graph.edge_distances(0)[n]);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
