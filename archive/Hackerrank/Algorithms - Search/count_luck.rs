//{"name":"Count Luck","group":"HackerRank - Algorithms - Search","url":"https://www.hackerrank.com/challenges/count-luck/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n2 3\n*.M\n.X.\n1\n4 11\n.X.X......X\n.X*.X.XXX.X\n.XX.X.XM...\n......XXXX.\n3\n4 11\n.X.X......X\n.X*.X.XXX.X\n.XX.X.XM...\n......XXXX.\n4\n","output":"Impressed\nImpressed\nOops!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CountLuck"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let map = input.read_char_table(n, m);
    let k = input.read_size();

    let mut graph = Graph::new(n * m);
    let mut start = 0;
    let mut end = 0;
    for i in 0..n {
        for j in 0..m {
            if map[(i, j)] == b'X' {
                continue;
            }
            if map[(i, j)] == b'M' {
                start = i * m + j;
            }
            if map[(i, j)] == b'*' {
                end = i * m + j;
            }
            let mut ways = 0;
            for (ni, nj) in D4::iter(i, j, n, m) {
                if map[(ni, nj)] == b'X' {
                    continue;
                }
                ways += 1;
            }
            let weight = if ways > 2 || ways == 2 && map[(i, j)] == b'M' {
                1
            } else {
                0
            };
            for (ni, nj) in D4::iter(i, j, n, m) {
                if map[(ni, nj)] == b'X' {
                    continue;
                }
                graph.add_edge(WeightedEdge::new(i * m + j, ni * m + nj, weight));
            }
        }
    }
    if graph.distance(start, end).unwrap().0 == k {
        out.print_line("Impressed");
    } else {
        out.print_line("Oops!");
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
