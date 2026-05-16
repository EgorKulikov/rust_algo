//{"name":"Arbitrage?","group":"Kattis","url":"https://open.kattis.com/problems/arbitrage","interactive":false,"timeLimit":3000,"tests":[{"input":"2\nCZK EUR\n2\nCZK EUR 25:1\nEUR CZK 1:25\n2\nGBP USD\n2\nUSD GBP 8:5\nGBP USD 5:9\n3\nBON DEM CZK\n3\nDEM BON 1:6\nBON CZK 1:5\nDEM CZK 1:20\n3\nCZK EUR GBP\n3\nCZK EUR 24:1\nEUR GBP 5:4\nGBP CZK 1:30\n3\nCZK USD GBP\n4\nCZK USD 28:1\nCZK GBP 31:1\nGBP CZK 1:31\nUSD GBP 1:1\n0\n","output":"Ok\nArbitrage\nOk\nOk\nArbitrage\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::graph::all_distances::{AllDistances, Distance};
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::real::{IntoReal, Real};
use algo_lib::scan;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let c = input.read_size();
    if c == 0 {
        return;
    }
    let mut cti = FxHashMap::default();
    for (i, s) in input.iter_str().take(c).enumerate() {
        cti.insert(s, i);
    }
    let r = input.read_size();
    let mut graph = Graph::new(c);
    for _ in 0..r {
        scan!(input, "@ @ @:@", f: Str, t: Str, a: Real, b: Real);
        graph.add_edge(WeightedEdge::new(
            cti[&f],
            cti[&t],
            (a / b).ln().into_real(),
        ));
    }
    let ad = graph.all_distances();
    for i in 0..c {
        if ad[(i, i)] < Distance::Finite(Real::zero()) {
            out.print_line("Arbitrage");
            return;
        }
    }
    out.print_line("Ok");
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
