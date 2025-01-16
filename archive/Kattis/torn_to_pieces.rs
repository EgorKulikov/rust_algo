//{"name":"Torn To Pieces","group":"Kattis","url":"https://open.kattis.com/problems/torn2pieces","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nUptown Midtown\nMidtown Uptown Downtown\nDowntown Midtown\nUptown Downtown\n","output":"Uptown Midtown Downtown\n"},{"input":"6\nA B\nB A D\nC D\nE D F G\nF E\nG E\nF A\n","output":"F E D B A\n"},{"input":"4\nFirstStop SecondStop\nSecondStop FirstStop ThirdStop\nFifthStop FourthStop SixthStop\nSixthStop FifthStop\nFirstStop FifthStop\n","output":"no route found\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let map = input.read_vec::<(Str, EolVec<Str>)>(n);
    let from = input.read_str();
    let to = input.read_str();

    let mut id = Id::new();
    id.get(from.clone());
    id.get(to.clone());
    for (from, to) in &map {
        id.get(from.clone());
        for to in to.iter() {
            id.get(to.clone());
        }
    }
    let mut graph = Graph::new(id.len());
    for (vert, to) in map {
        let vert = id.get(vert);
        for to in to.unwrap() {
            let to = id.get(to);
            graph.add_edge(BiWeightedEdge::new(vert, to, 1));
        }
    }
    let to = id.get(to);
    let Some(dist) = graph.distance(id.get(from), to) else {
        out.print_line("no route found");
        return;
    };
    let path = dist.1.detuple().0;
    let bi = id.by_id();
    let ans = path.iter_chain(once(to)).map(|x| &bi[x]);
    out.print_line_iter(ans);
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
