//{"name":"F. Thesis Group Selection","group":"Codeforces - IUT Eid Salami Programming Contest 2026 - Powered by Okkhor Technology (Online Mirror)","url":"https://codeforces.com/gym/106438/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 2\n1 2 3 4\n3.00 4.00 4.00 3.00\n3 1\n124 220 217\n3.90 3.97 3.88\n","output":"3.500000000\n3.916666667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let f = input.read_int_vec(n);
    let c = input.read_real_vec(n);

    let order = (0..n)
        .collect::<Vec<_>>()
        .sorted_by_key(|&i| Reverse((c[i], f[i])));
    let mut ans = Real(0.);
    let mut steps = 0;
    for i in (0..n).step_by(k) {
        let mut sum = Real(0.);
        let mut we = false;
        let mut times = 0;
        for j in i..(i + k).min(n) {
            if order[j] == 0 {
                we = true;
                break;
            }
            sum += c[order[j]];
            times += 1;
        }
        if we {
            ans += c[0];
        } else if times == k {
            ans += sum / k;
        } else {
            let v1 = (ans + sum / times) / (steps + 1);
            let v2 = ans / steps;
            ans = ((v1 * times + v2 * (k - times)) / k) * (steps + 1);
        }
        steps += 1;
    }
    ans /= steps;
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
