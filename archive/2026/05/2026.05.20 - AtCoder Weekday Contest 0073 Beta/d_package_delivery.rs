//{"name":"D - Package Delivery","group":"AtCoder - AtCoder Weekday Contest 0073 Beta","url":"https://atcoder.jp/contests/awc0073/tasks/awc0073_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 3\n1 2 3\n2 3 4\n1 3 10\n1 3\n2 3\n","output":"11\n"},{"input":"4 5 7\n1 2 2\n1 3 8\n2 3 3\n2 4 5\n3 4 1\n3 5 6\n4 5 4\n1 5\n3 1\n4 5\n1 4\n","output":"25\n"},{"input":"5 6 9\n1 2 7\n1 3 9\n1 6 14\n2 3 10\n2 4 15\n3 4 11\n3 6 2\n4 5 6\n5 6 9\n1 5\n2 6\n3 4\n6 4\n1 4\n","output":"76\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(k).dec();

    let mut graph = Graph::new(m, k);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let d = graph.all_distances();

    let mut ans = 0;
    for _ in 0..n {
        let s = input.read_size() - 1;
        let t = input.read_size() - 1;
        ans += d[(s, t)].to_finite();
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
