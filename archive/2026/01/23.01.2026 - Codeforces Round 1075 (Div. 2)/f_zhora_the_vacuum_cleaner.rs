//{"name":"F. Zhora the Vacuum Cleaner","group":"Codeforces - Codeforces Round 1075 (Div. 2)","url":"https://codeforces.com/contest/2189/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4 1 1\n1 1 1 1\n1 2\n2 3\n2 4\n5 1 100\n1 1 1 1 1\n1 2\n2 3\n3 4\n4 5\n5 9 10\n4 8 0 8 4\n1 3\n2 3\n3 4\n3 5\n4 1 4\n0 3 1 1\n2 1\n3 1\n4 3\n5 21 93\n0 0 64 4 87\n2 1\n3 1\n4 3\n5 1\n4 5 8\n2 3 0 8\n4 3\n2 3\n3 1\n","output":"2\n102\n40\n6\n270\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_long();
    let q = input.read_long();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let total = a.copy_sum();
    if q == 0 || total == 0 {
        out.print_line(0);
        return;
    }
    if p == 0 {
        out.print_line(q);
        return;
    }
    let mut need = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> i64 {
        let mut cur = a[vert];
        let mut max = 0;
        let mut non_zero = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            cur += call;
            max.maxim(call);
            if call > 0 {
                non_zero += 1;
            }
        }
        if cur != total {
            non_zero += 1;
        }
        if a[vert] != 0 || non_zero > 2 {
            need[vert] = cur.min(total - max);
        }
        cur
    });
    dfs.call(0, n);
    need.sort();
    let mut ans = q * a.copy_filter(|&x| x != 0).count() as i64;
    for i in 0..n - 1 {
        ans.minim(need[i] * p + q * (n as i64 - i as i64 - 1));
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
