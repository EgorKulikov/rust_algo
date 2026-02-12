//{"name":"E1. Fuzzy Concatenation (Easy Version)","group":"Codeforces - Codeforces Round 1079 (Div. 1)","url":"https://codeforces.com/contest/2196/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\na\nb\n5 5\naaaaa\nabzba\n10 13\ndhhtyhwbsl\nhtahbsehtyhzb\n7 2\ncontest\non\n","output":"1\n3\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let _n = input.read_size();
    let t = input.read_str();
    let s = input.read_str();

    let sets = Vec::with_gen(26, |i| {
        let mut set = BitSet::new(m + 1);
        for j in 0..m {
            if t[j] - b'a' == i as u8 {
                set.set(j);
            }
        }
        set
    });
    let mut ans = 1;
    let mut strict = BitSet::new(m + 1);
    strict.fill(true);
    let mut non_strict = BitSet::new(m + 1);
    for c in s {
        non_strict &= &sets[(c - b'a') as usize];
        non_strict |= &strict;
        strict &= &sets[(c - b'a') as usize];
        strict <<= 1;
        non_strict <<= 1;
        if non_strict.count_ones() == 0 {
            ans += 1;
            strict = sets[(c - b'a') as usize].clone();
            strict <<= 1;
            non_strict.fill(true);
            non_strict.unset(0);
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
