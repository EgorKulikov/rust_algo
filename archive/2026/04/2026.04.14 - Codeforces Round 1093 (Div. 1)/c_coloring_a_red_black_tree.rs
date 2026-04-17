//{"name":"C. Coloring a Red Black Tree","group":"Codeforces - Codeforces Round 1093 (Div. 1)","url":"https://codeforces.com/contest/2219/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n101\n1 2\n2 3\n5\n10010\n1 2\n2 3\n3 4\n2 5\n3\n010\n1 2\n2 3\n","output":"1.00000000000000000000\n4.50000000000000000000\n2.00000000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;
use algo_lib::numbers::real::Real;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut s = input.read_str();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut p = vec![0i64; n];
    let mut q = vec![0; n];
    let mut set = BTreeSet::new();
    for i in 0..n {
        if s[i] == b'1' {
            continue;
        }
        for e in &graph[i] {
            if s[e.to()] == b'1' {
                p[i] += 1;
            } else {
                q[i] += 1;
            }
        }
        set.insert((Reverse(Rational::new(p[i] * (p[i] + 1), q[i] + p[i])), i));
    }
    let mut ans = Real(0.);
    while let Some((_, i)) = set.pop_first() {
        ans += Real::from(p[i] + q[i]) / p[i];
        s[i] = b'1';
        for e in &graph[i] {
            if s[e.to()] == b'0' {
                set.remove(&(
                    Reverse(Rational::new(
                        p[e.to()] * (p[e.to()] + 1),
                        q[e.to()] + p[e.to()],
                    )),
                    e.to(),
                ));
                p[e.to()] += 1;
                q[e.to()] -= 1;
                set.insert((
                    Reverse(Rational::new(
                        p[e.to()] * (p[e.to()] + 1),
                        q[e.to()] + p[e.to()],
                    )),
                    e.to(),
                ));
            }
        }
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
