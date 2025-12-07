//{"name":"E. Shiro's Mirror Duel","group":"Codeforces - Codeforces Round 1068 (Div. 2)","url":"https://codeforces.com/contest/2173/problem/E","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n5\n5 1 3 4 2\n\n1 5\n\n2 1\n\n2\n1 2\n","output":"\n\n\n? 1 5\n\n? 4 5\n\n!\n\n\n!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n).dec();

    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        while p[i] != i {
            let pos = p.copy_position(|x| x == i).unwrap();
            out.print_line(("?", i + 1, pos + 1));
            out.flush();
            let x = input.read_size() - 1;
            let y = input.read_size() - 1;
            p.swap(x, y);
        }
        while p[i] != i || p[j] != j {
            if p[i] != i {
                let pos = p.copy_position(|x| x == i).unwrap();
                out.print_line(("?", i + 1, pos + 1));
                out.flush();
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                p.swap(x, y);
            } else {
                let pos = p.copy_position(|x| x == j).unwrap();
                out.print_line(("?", j + 1, pos + 1));
                out.flush();
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                p.swap(x, y);
            }
        }
        i += 1;
        j -= 1;
    }
    out.print_line("!");
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
