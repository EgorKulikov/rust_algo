//{"name":"F. Isla's Memory Thresholds","group":"Codeforces - Codeforces Round 1068 (Div. 2)","url":"https://codeforces.com/contest/2173/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"3\n5 4\n7 5 5 2 1\n1 3 10\n2 5 6\n1 5 7\n3 5 4\n6 5\n6 6 5 3 2 2\n1 6 2\n1 6 7\n2 6 7\n2 5 4\n2 5 3\n11 7\n938412006 792864920 746880066 729862150 704473377 550436315 381392172 326088331 316506801 301443698 190862681\n1 3 417253102\n9 11 857592497\n1 11 344359921\n1 7 408760015\n8 8 544749974\n7 10 361090133\n3 11 888178376\n","output":"1 5\n1 3\n2 3\n1 3\n6 0\n2 4\n2 0\n3 0\n3 2\n3 0\n0 808813180\n9 0\n6 381392172\n0 326088331\n2 301443698\n3 492306379\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    let s = a.partial_sums();
    for _ in 0..q {
        let mut l = input.read_size() - 1;
        let r = input.read_size();
        let x = input.read_size();

        let mut cnt = 0;
        let mut len = 1;
        while s[r] - s[l] >= x {
            let mut left = len;
            let mut right = r - l;
            while left < right {
                let mid = (left + right) / 2;
                if s[l + mid] - s[l] >= x {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            len = left;
            let mut left = 1;
            let mut right = (r - l) / len;
            while left < right {
                let mid = (left + right + 1) / 2;
                if s[l + mid * len] - s[l + (mid - 1) * len] >= x {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            cnt += left;
            l += left * len;
        }
        out.print_line((cnt, s[r] - s[l]));
    }
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
