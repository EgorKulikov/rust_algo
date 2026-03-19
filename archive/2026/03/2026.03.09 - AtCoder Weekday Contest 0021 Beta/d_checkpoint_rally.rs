//{"name":"D - Checkpoint Rally","group":"AtCoder - AtCoder Weekday Contest 0021 Beta","url":"https://atcoder.jp/contests/awc0021/tasks/awc0021_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 6 2\n1 2 3\n2 3 4\n3 5 2\n1 4 10\n4 5 1\n2 4 5\n3 4\n","output":"11\n"},{"input":"4 3 1\n1 2 5\n2 3 3\n3 4 7\n2\n","output":"15\n"},{"input":"10 15 4\n1 2 2\n1 3 5\n2 3 1\n2 4 4\n3 5 3\n4 5 2\n4 6 6\n5 6 1\n5 7 8\n6 8 3\n7 8 2\n7 9 4\n8 9 1\n8 10 5\n9 10 2\n3 6 8 9\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
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
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let mut p = input.read_size_vec(k).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    p.push(n - 1);
    let mut last = 0;
    let mut ans = 0;
    for i in p {
        let d = graph.distances_from(i);
        if let Some((d, ..)) = d[last] {
            ans += d;
        } else {
            out.print_line(-1);
            return;
        }
        last = i;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
