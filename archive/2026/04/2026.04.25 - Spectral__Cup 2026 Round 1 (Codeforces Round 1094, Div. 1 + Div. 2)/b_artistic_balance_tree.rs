//{"name":"B. Artistic Balance Tree","group":"Codeforces - Spectral::Cup 2026 Round 1 (Codeforces Round 1094, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2222/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n7 4\n1 2 3 4 5 6 7\n1 2 3 4\n7 4\n1 -2 3 4 -5 -6 -7\n7 6 5 4\n7 5\n21 -45 234 -8 423 12 -987\n6 6 6 6 6\n7 5\n-21 45 -234 8 -423 -12 987\n7 7 7 7 7\n7 3\n-1 2 -3 4 5 6 7\n1 2 3\n7 3\n-1 -2 -3 -4 -5 -6 -7\n1 2 3\n","output":"6\n-20\n-362\n-637\n2\n-25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let x = input.read_size_vec(m).dec();

    let odd = x.copy_filter(|&i| i % 2 == 0).count();
    let even = m - odd;
    let qty = [odd, even];

    let mut ans = a.copy_sum();
    for i in 0..2 {
        let v = a
            .copy_skip(i)
            .step_by(2)
            .collect::<Vec<_>>()
            .sorted()
            .reversed();

        if qty[i] > 0 {
            ans -= v[0];
        }
        for i in 1..(qty[i]).min(v.len()) {
            ans -= v[i].max(0);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
