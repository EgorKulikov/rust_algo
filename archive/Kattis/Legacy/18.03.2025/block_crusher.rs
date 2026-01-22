//{"name":"Block Crusher","group":"Kattis","url":"https://open.kattis.com/problems/blockcrusher","interactive":false,"timeLimit":1000,"tests":[{"input":"3 7\n2281431\n2329463\n7183839\n7 6\n517135\n935519\n731353\n375951\n575195\n579573\n359739\n0 0\n","output":"228 431\n23 9463\n7 83839\n\n517 35\n9355 9\n73135 \n37595 \n57519 \n57957 \n3597 9\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D8;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    if h == 0 {
        return;
    }
    let mut s = input.read_char_table(h, w);

    let mut graph = Graph::new(h * w + 2);
    for i in 0..w {
        graph.add_edge(WeightedEdge::new(h * w, i, (s[(0, i)] - b'0') as i32));
        graph.add_edge(WeightedEdge::new((h - 1) * w + i, h * w + 1, 0));
    }
    for i in 0..h {
        for j in 0..w {
            for (r, c) in D8::iter(i, j, h, w) {
                graph.add_edge(WeightedEdge::new(
                    i * w + j,
                    r * w + c,
                    (s[(r, c)] - b'0') as i32,
                ));
            }
        }
    }
    let path = graph.distance(h * w, h * w + 1).unwrap().1.detuple().0;
    for x in path {
        if x < h * w {
            let r = x / w;
            let c = x % w;
            s[(r, c)] = b' ';
        }
    }
    out.print_table(&s);
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
