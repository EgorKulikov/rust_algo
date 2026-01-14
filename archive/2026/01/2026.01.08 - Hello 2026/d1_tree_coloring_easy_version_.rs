//{"name":"D1. Tree Coloring (Easy Version)","group":"Codeforces - Hello 2026","url":"https://codeforces.com/contest/2183/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n5\n3 1\n1 2\n5 1\n4 1\n5\n3 2\n2 4\n2 5\n1 2\n5\n3 4\n4 1\n5 1\n1 2\n5\n2 5\n3 1\n2 1\n3 4\n5\n1 3\n1 5\n4 3\n2 4\n13\n2 1\n3 2\n4 2\n5 4\n6 3\n7 1\n8 5\n9 6\n10 4\n11 7\n12 8\n13 10\n10\n5 7\n8 1\n1 10\n2 8\n8 4\n9 4\n6 1\n5 3\n7 8\n10\n7 6\n3 7\n6 9\n7 1\n9 8\n5 1\n3 10\n9 2\n1 4\n10\n10 6\n2 8\n4 10\n7 5\n1 2\n7 10\n10 9\n9 1\n7 3\n10\n6 8\n9 7\n4 10\n5 9\n4 2\n3 8\n6 5\n1 5\n1 10\n","output":"5\n4\n4\n3\n3\n3\n4\n4\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let d = graph.edge_distances(0);
    let mut q = vec![0; n];
    for i in d {
        q[i as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        ans.maxim(q[i]);
    }
    ans.maxim(graph[0].len() + 1);
    for i in 1..n {
        ans.maxim(graph[i].len());
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
