//{"name":"Gena Playing Hanoi","group":"HackerRank - Algorithms - Search","url":"https://www.hackerrank.com/challenges/gena/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"STDIN   Function\n-----   --------\n3       size of posts[] n = 3\n1 4 1   posts = [1, 4, 1]\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GenaPlayingHanoi"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let posts = input.read_size_vec(n).dec();

    let mut graph = Graph::new(4usize.power(n));
    let powers = powers(4usize, n);
    let mut rec =
        RecursiveFunction3::new(|rec, step: usize, state: usize, smallest: [usize; 4]| {
            if step == 0 {
                for i in 0..4 {
                    for j in 0..4 {
                        if smallest[i] < smallest[j] {
                            graph.add_edge(Edge::new(
                                state,
                                state - powers[smallest[i]] * i + powers[smallest[i]] * j,
                            ));
                        }
                    }
                }
                return;
            }
            for i in 0..4 {
                let mut n_smallest = smallest;
                n_smallest[i] = step - 1;
                rec.call(step - 1, state * 4 + i, n_smallest);
            }
        });
    rec.call(n, 0, [n; 4]);
    let mut start_state = 0;
    for i in (0..n).rev() {
        start_state *= 4;
        start_state += posts[i];
    }
    out.print_line(graph.edge_distances(start_state)[0]);
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
