//{"name":"L. Alice and Bob","group":"Universal Cup - GP of India","url":"https://contest.ucup.ac/contest/3516/problem/17417","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2 20\n1 2\n2 3\n3 4\n2 4\n1 2\n2 3\n3 4\n","output":"1 2\n1 3\n1 2\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(2 * n - 1).dec();

    let graph = Graph::with_biedges(2 * n, &edges);
    for i in 0..2 * n {
        if graph[i].len() != 1 {
            continue;
        }
        let root = graph[i][0].to();
        let lca = graph.lca_with_root(root);
        let mut sum = 0;
        for j in 0..2 * n {
            sum += lca.path_length(root, j);
        }
        if sum % 2 == k % 2 {
            let mut other = 2 * n;
            for e in &graph[root] {
                if e.to() != i {
                    other = e.to();
                    break;
                }
            }
            assert_ne!(other, 2 * n);
            out.print_line((i + 1, root + 1));
            out.print_line((i + 1, other + 1));
        } else {
            out.print_line((i + 1, root + 1));
            out.print_line((i + 1, root + 1));
        }
        return;
    }
    unreachable!();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
