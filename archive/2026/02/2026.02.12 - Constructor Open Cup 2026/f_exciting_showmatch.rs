//{"name":"F. Exciting Showmatch","group":"Codeforces - Constructor Open Cup 2026","url":"https://constructor2026.contest.codeforces.com/group/XdjJUfzFUt/contest/670933/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2 3 4\n2\n1 2 4 5\n3\n1 3 10 13 37 42\n3\n1 2 3 13 37 42\n","output":"3 2\n2 4\n0 20\n2 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(2 * n);

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(2 * n + 1);
    let mut exc = 0;
    let mut parts = 0;
    let mut last = usize::MAX - 1;
    let mut cur: usize = 0;
    let mut ans = Mod::new(1);
    for i in a {
        if i == last + 1 {
            cur += 1;
        } else {
            exc += cur.saturating_sub(1);
            if cur % 2 != 0 {
                parts += 1;
            } else {
                ans *= 2;
            }
            cur = 1;
        }
        last = i;
    }
    if cur % 2 != 0 {
        parts += 1;
    } else {
        ans *= 2;
    }
    ans /= 2;
    exc += cur - 1;
    ans *= c.c(parts, parts / 2);
    out.print_line((exc, ans));
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
