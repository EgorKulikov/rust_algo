//{"name":"F. SHA75","group":"Toph","url":"https://toph.co/arena?contest=diu-unlock-the-algorithm-fall-24-preliminary-mock#!/p/6745eb8bcb14b2a6cffe556f","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n2 0 1 3 0 4 1 1 0 0\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSHA75"}}}

use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        if i >= a[i] {
            graph.add_edge(Edge::new(i - a[i], i));
        }
        if i + a[i] < n {
            graph.add_edge(Edge::new(i + a[i], i));
        }
    }
    let dist = graph.edge_distances(n - 1);
    out.set_bool_output(BoolOutput::YesNo);
    out.print_line(dist[0] != u32::MAX);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
