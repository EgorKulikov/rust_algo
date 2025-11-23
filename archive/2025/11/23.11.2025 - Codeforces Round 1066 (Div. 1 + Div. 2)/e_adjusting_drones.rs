//{"name":"E. Adjusting Drones","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3\n1 1 1 1 1 1\n5 1\n1 3 2 1 4\n6 2\n1 1 1 2 3 3\n4 1\n8 8 8 8\n2 2\n1 2\n","output":"3\n4\n4\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();

    let qty = a.qty_bound(2 * n);
    let ps = qty.partial_sums();
    let last = ps[2 * n] as i64 - (2 * n) as i64;
    let mut mins = vec![(last, 2 * n)];
    let mut ans = 0;
    for i in (0..2 * n).rev() {
        if qty[i] > k {
            let target = ps[i] as i64 - i as i64 + k as i64 - 1;
            let pos = mins.lower_bound(&(target, usize::MAX));
            let x = if pos == 0 {
                (2 * n) as i64 + (last - target)
            } else {
                mins[pos - 1].1 as i64
            };
            ans.maxim(x as usize - i - 1);
        }
        let cur = ps[i] as i64 - i as i64;
        while let Some(&(val, _)) = mins.last() {
            if val >= cur {
                mins.pop();
            } else {
                break;
            }
        }
        mins.push((cur, i));
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
