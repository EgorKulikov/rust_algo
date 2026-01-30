//{"name":"C. Jerry and Tom","group":"Codeforces - Codeforces Round 1077 (Div. 1)","url":"https://codeforces.com/contest/2187/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n2 0\n3 1\n1 3\n4 2\n2 4\n1 4\n5 1\n1 4\n8 3\n1 4\n5 8\n2 4\n","output":"0\n2\n6\n3\n23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::VecDeque;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut to = (1..n).collect::<Vec<_>>();
    for (u, v) in edges {
        to[u].maxim(v);
    }
    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(i, to[i]));
    }
    let mut ans = 0;
    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (VecDeque<usize>, usize) {
            let mut cur = VecDeque::new();
            let mut cur_sum = 0;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let (mut call, mut call_sum) = f.call(e.to(), vert);
                if call.len() > cur.len() {
                    swap(&mut cur, &mut call);
                    swap(&mut cur_sum, &mut call_sum);
                }
                let mut rem_cur = cur_sum;
                let mut rem_call = call_sum;
                for i in 0..call.len() {
                    ans += (rem_cur * call[i] + rem_call * cur[i]) * (i + 1);
                    rem_cur -= cur[i];
                    rem_call -= call[i];
                    cur[i] += call[i];
                }
                cur_sum += call_sum;
            }
            cur.push_front(1);
            cur_sum += 1;
            (cur, cur_sum)
        });
    dfs.call(n - 1, n);
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
