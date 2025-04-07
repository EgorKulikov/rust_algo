//{"name":"D - Takahashi the Wall Breaker","group":"AtCoder - AtCoder Beginner Contest 400","url":"https://atcoder.jp/contests/abc400/tasks/abc400_d","interactive":false,"timeLimit":2000,"tests":[{"input":"10 10\n..........\n#########.\n#.......#.\n#..####.#.\n##....#.#.\n#####.#.#.\n.##.#.#.#.\n###.#.#.#.\n###.#.#.#.\n#.....#...\n1 1 7 1\n","output":"1\n"},{"input":"2 2\n.#\n#.\n1 1 2 2\n","output":"1\n"},{"input":"1 3\n.#.\n1 1 1 3\n","output":"1\n"},{"input":"20 20\n####################\n##...##....###...###\n#.....#.....#.....##\n#..#..#..#..#..#..##\n#..#..#....##..#####\n#.....#.....#..#####\n#.....#..#..#..#..##\n#..#..#.....#.....##\n#..#..#....###...###\n####################\n####################\n##..#..##...###...##\n##..#..#.....#.....#\n##..#..#..#..#..#..#\n##..#..#..#..#..#..#\n##.....#..#..#..#..#\n###....#..#..#..#..#\n#####..#.....#.....#\n#####..##...###...##\n####################\n3 3 18 18\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::const_value_ref;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::{Directions, D4};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let s = input.read_char_table(h, w);
    let a = input.read_size() - 1;
    let b = input.read_size() - 1;
    let c = input.read_size() - 1;
    let d = input.read_size() - 1;

    let mut graph = Graph::new(h * w);
    for i in 0..h {
        for j in 0..w {
            for (r, c) in D4::iter(i, j, h, w) {
                if s[(r, c)] == b'.' {
                    graph.add_edge(WeightedEdge::new(i * w + j, r * w + c, 0));
                }
            }
            const_value_ref!(
                pub D4X2Dirs: [(isize, isize); 8] as [(isize, isize)] = [(0, 1), (1, 0), (0, -1), (-1, 0), (0, 2), (2, 0), (0, -2), (-2, 0)]
            );
            pub type D4X2 = Directions<D4X2Dirs>;

            for (r, c) in D4X2::iter(i, j, h, w) {
                graph.add_edge(WeightedEdge::new(i * w + j, r * w + c, 1));
            }
        }
    }
    out.print_line(graph.distance(a * w + b, c * w + d).unwrap().0);
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
