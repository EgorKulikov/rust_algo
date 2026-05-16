//{"name":"Chess","group":"Kattis","url":"https://open.kattis.com/problems/chess","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nE 2 E 3\nF 1 E 8\nA 3 A 3\n","output":"Impossible\n2 F 1 B 5 E 8\n0 A 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut graph = Graph::new(64);
    for i in 0..8 {
        for j in 0..8 {
            for k in 1..(8 - i).min(8 - j) {
                graph.add_edge(BiWeightedEdge::new(i * 8 + j, (i + k) * 8 + j + k, 1));
            }
            for k in 1..(8 - i).min(j + 1) {
                graph.add_edge(BiWeightedEdge::new(i * 8 + j, (i + k) * 8 + j - k, 1));
            }
        }
    }

    let x1 = (input.read_char() - b'A') as usize;
    let y1 = input.read_size() - 1;
    let x2 = (input.read_char() - b'A') as usize;
    let y2 = input.read_size() - 1;

    let start = x1 * 8 + y1;
    let end = x2 * 8 + y2;
    let path = graph.distance(start, end);
    if let Some((dist, path)) = path {
        out.print(dist);
        for (from, _) in path {
            out.print(format!(
                " {} {}",
                (from / 8 + b'A' as usize) as u8 as char,
                from % 8 + 1
            ));
        }
        output!(out, " {} {}", (x2 + b'A' as usize) as u8 as char, y2 + 1);
    } else {
        out.print_line("Impossible");
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

//START MAIN
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
//END MAIN
