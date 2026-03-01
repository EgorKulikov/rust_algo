//{"name":"C. Simons and Posting Blogs","group":"Codeforces - Codeforces Round 1083 (Div. 2)","url":"https://codeforces.com/contest/2205/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n5 1 2 3 4 6\n3 2 5 1\n4 1 9 2 3\n2\n2 1 6\n1 6\n1\n3 6 1 1\n5\n4 2 3 3 4\n5 1 2 4 3 1\n2 4 1\n3 3 3 1\n5 4 3 2 2 2\n5\n4 2 3 1 4\n5 2 5 5 6 5\n5 3 4 7 5 5\n8 3 6 4 3 1 1 5 4\n2 1 1\n","output":"1 5 2 3 9 6 4\n6 1\n1 6\n1 3 2 4\n1 4 3 2 5 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input
        .iter::<Vec<i32>>()
        .take(n)
        .map(|x| {
            let mut seen = FxHashSet::default();
            let mut res = Vec::new();
            for i in x.iter_rev() {
                if seen.insert(i) {
                    res.push(i);
                }
            }
            res
        })
        .collect::<Vec<_>>()
        .sorted();

    let mut done = FxHashSet::default();
    let mut ans = Vec::new();
    for _ in 0..n {
        ans.extend(a[0].copy_iter());
        for i in a[0].copy_iter() {
            assert!(done.insert(i));
        }
        let mut na = Vec::new();
        for i in 1..a.len() {
            let mut cur = Vec::new();
            for j in a[i].copy_iter() {
                if !done.contains(&j) {
                    cur.push(j);
                }
            }
            na.push(cur);
        }
        a = na.sorted();
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
