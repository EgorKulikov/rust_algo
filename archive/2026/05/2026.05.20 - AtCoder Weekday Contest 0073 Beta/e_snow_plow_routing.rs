//{"name":"E - Snow Plow Routing","group":"AtCoder - AtCoder Weekday Contest 0073 Beta","url":"https://atcoder.jp/contests/awc0073/tasks/awc0073_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2 4\n2 3 5\n3 1 6\n","output":"15\n"},{"input":"3 2\n1 2 3\n2 3 5\n","output":"16\n"},{"input":"8 11\n1 2 1\n2 3 2\n3 4 3\n4 5 4\n5 6 5\n6 7 6\n7 8 7\n8 1 8\n1 3 2\n2 5 3\n4 7 1\n","output":"48\n"},{"input":"15 20\n1 2 10\n2 3 8\n3 4 7\n4 5 12\n5 6 3\n6 7 9\n7 8 5\n8 9 11\n9 10 6\n10 11 4\n11 12 8\n12 13 7\n13 14 13\n14 15 2\n15 1 14\n1 8 20\n3 10 15\n5 12 6\n7 14 9\n2 9 18\n","output":"228\n"},{"input":"2 1\n1 2 1000000\n","output":"2000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n, m);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut special = Vec::new();
    for i in 0..n {
        if graph.degrees()[i] % 2 == 1 {
            special.push(i);
        }
    }
    let d = Vec::with_gen(special.len(), |i| graph.distances_from(special[i]));
    let base = graph.edges().map(|e| e.1.weight()).sum::<i64>() / 2;
    let mut mem = Memoization2d::new(
        special.len() + 1,
        1 << special.len(),
        |mem, pos, mask| -> i64 {
            if pos == special.len() {
                0
            } else if mask.is_set(pos) {
                mem.call(pos + 1, mask)
            } else {
                let mut res = i64::MAX;
                for i in pos + 1..special.len() {
                    if !mask.is_set(i) {
                        res.minim(
                            d[pos][special[i]].unwrap().0 + mem.call(pos + 1, mask.with_bit(i)),
                        );
                    }
                }
                res
            }
        },
    );
    out.print_line(base + mem.call(0, 0));
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
