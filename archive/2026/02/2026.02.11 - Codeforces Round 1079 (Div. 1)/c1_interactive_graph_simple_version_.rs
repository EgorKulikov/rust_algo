//{"name":"C1. Interactive Graph (Simple Version)","group":"Codeforces - Codeforces Round 1079 (Div. 1)","url":"https://codeforces.com/contest/2196/problem/C1","interactive":true,"timeLimit":2000,"tests":[{"input":"3\n5\n\n1 1\n\n2 1 2\n\n3 1 2 4\n\n3 1 2 5\n\n2 1 3\n\n3 1 3 4\n\n3 1 3 5\n\n1 2\n\n1 3\n\n1 4\n\n1 5\n\n1\n\n0\n\n2\n\n1 1\n\n1 2\n\n2 2 1\n","output":"\n\n? 1\n\n? 2\n\n? 3\n\n? 4\n\n? 5\n\n? 6\n\n? 7\n\n? 8\n\n? 11\n\n? 14\n\n? 15\n\n! 6\n1 3\n1 2\n2 4\n3 4\n2 5\n3 5\n\n? 2\n\n! 0\n\n? 1\n\n? 2\n\n? 3\n\n! 1\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::slicelike::Slicelike;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut id = 2;
    let mut graph = Graph::new(n);
    let mut seen = BitSet::new(n);
    let mut cur = vec![0];
    let mut num_paths = vec![0; n];
    num_paths[0] = 1;

    loop {
        out.print_line(('?', id));
        out.flush();
        let len = input.read_size();
        if len == 0 {
            break;
        }
        let path = input.read_size_vec(len).dec();
        while !path.starts_with(&cur) {
            let last = cur.pop().unwrap();
            seen.set(last);
        }
        assert_eq!(path.len(), cur.len() + 1);
        if !cur.is_empty() {
            graph.add_edge(Edge::new(cur[Back(0)], path[Back(0)]));
        }
        cur = path;
        let add = if seen[cur[Back(0)]] {
            let last = cur.pop().unwrap();
            num_paths[last]
        } else {
            1
        };
        for v in cur.copy_iter() {
            num_paths[v] += add;
        }
        id += add;
    }
    out.print_line(('!', graph.edge_count()));
    for (from, e) in graph.edges() {
        out.print_line((from + 1, e.to() + 1));
    }
    out.flush();
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
