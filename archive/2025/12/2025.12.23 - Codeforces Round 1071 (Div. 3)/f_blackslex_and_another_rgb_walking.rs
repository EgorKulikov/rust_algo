//{"name":"F. Blackslex and Another RGB Walking","group":"Codeforces - Codeforces Round 1071 (Div. 3)","url":"https://codeforces.com/contest/2179/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"first\n2\n7 8\n1 2\n1 6\n3 2\n4 2\n6 4\n4 7\n5 6\n5 7\n\n4 4\n1 2\n1 4\n3 2\n3 4\n","output":"rrgbggr\nrbbb\n"},{"input":"second\n2\n2\n3\ngrr\n3\ngbr\n\n1\n2\nrb\n","output":"1\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut cur = vec![0];
    let mut color = vec![b' '; n];
    color[0] = b'r';
    let s = b"rgb";
    for i in 1usize.. {
        let mut next = Vec::new();
        for v in cur.copy_iter() {
            for e in &graph[v] {
                if color[e.to()] == b' ' {
                    color[e.to()] = s[i % 3];
                    next.push(e.to());
                }
            }
        }
        cur = next;
        if cur.is_empty() {
            break;
        }
    }
    out.print_line(Str::from(color));
    out.flush();
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_size();
    for _ in 0..q {
        let d = input.read_size();
        let s = input.read_str();

        let mut colors = s.clone().unwrap().sorted();
        colors.dedup();
        let target = if colors.len() == 1 {
            colors[0]
        } else if colors.as_slice() == b"br" {
            b'r'
        } else if colors.as_slice() == b"bg" {
            b'b'
        } else {
            b'g'
        };
        for i in 0..d {
            if s[i] == target {
                out.print_line(i + 1);
                break;
            }
        }
        out.flush();
    }
}

pub static TEST_TYPE: TestType = TestType::RunTwiceMultiNumber;
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
