//{"name":"G. Treasure Hunt in Laurasia","group":"Codeforces - IUT Eid Salami Programming Contest 2026 - Powered by Okkhor Technology (Online Mirror)","url":"https://codeforces.com/gym/106438/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 9\n1 1 2 5\n32 21 50 60\n3 3\n1 2 3\n5 2 3\n","output":"32 53 82 103 103 103 113 142 163\n5 5 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let w = input.read_size_vec(n);
    let p = input.read_long_vec(n);

    let mut ans = vec![0; k + 1];
    let mut by_w = vec![Vec::new(); k + 1];
    for i in 0..n {
        if w[i] <= k {
            by_w[w[i]].push(p[i]);
        }
    }
    for i in 1..=k {
        by_w[i].sort();
        by_w[i].reverse();
        for p in by_w[i].copy_iter() {
            let mut updated = false;
            for j in (i..=k).rev() {
                let cand = ans[j - i] + p;
                if ans[j].maxim(cand) {
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }
    }
    for i in 1..=k {
        let cand = ans[i - 1];
        ans[i].maxim(cand);
    }
    out.print_line(&ans[1..]);
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
