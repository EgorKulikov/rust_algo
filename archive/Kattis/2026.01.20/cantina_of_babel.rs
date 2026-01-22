//{"name":"Cantina of Babel","group":"Kattis","url":"https://open.kattis.com/problems/cantinaofbabel","interactive":false,"timeLimit":1000,"tests":[{"input":"7\nJabba-the-Hutt Huttese\nBib-Fortuna Huttese Basic\nBoba-Fett Basic Huttese\nChewbacca Shyriiwook Basic\nLuke Basic Jawaese Binary\nGrakchawwaa Shyriiwook Basic Jawaese\nR2D2 Binary Basic\n","output":"2\n"},{"input":"6\nFran French Italian\nEnid English German\nGeorge German Italian\nIan Italian French Spanish\nSpencer Spanish Portugese\nPolly Portugese Spanish\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::split::StrSplit;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut base = Vec::with_capacity(n);
    let mut additional = Vec::with_capacity(n);
    for _ in 0..n {
        let s = input.read_line();
        let t = s.str_split(b" ");
        base.push(Str::from(t[1]));
        additional.push(t.iter().skip(2).map(|x| Str::from(*x)).collect::<Vec<_>>());
    }

    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..n {
            if i != j && (additional[i].contains(&base[j]) || base[i] == base[j]) {
                graph.add_edge(Edge::new(i, j));
            }
        }
    }
    let col = graph.strongly_connected_component_colors();
    let q = col.qty();
    out.print_line(n - q.copy_max());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
