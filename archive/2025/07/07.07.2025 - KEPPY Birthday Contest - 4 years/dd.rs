//{"name":"dd","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::num::Wrapping;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lrsp = input.read_vec::<(isize, usize, usize, usize)>(n);

    let mut graph = Graph::new(n);
    for i in 1..n {
        graph.add_edge(Edge::new(lrsp[i].3 - 1, i));
    }
    let mut mem = Memoization2d::new(n, 3002, |mem, id: usize, pos: usize| -> Wrapping<u32> {
        let (_, r, s, _) = lrsp[id];
        if pos > r {
            return Wrapping(0);
        }
        let base = if graph[id].is_empty() {
            Wrapping(1)
        } else {
            let mut res = Wrapping(0);
            for e in &graph[id] {
                let to = e.to();
                if lrsp[to].0 == -1 {
                    res += mem.call(to, pos);
                } else {
                    res += mem.call(to, lrsp[to].0 as usize);
                }
            }
            res
        };
        base + mem.call(id, (pos + s).min(3001))
    });
    out.print_line(mem.call(0, lrsp[0].0 as usize).0);
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
