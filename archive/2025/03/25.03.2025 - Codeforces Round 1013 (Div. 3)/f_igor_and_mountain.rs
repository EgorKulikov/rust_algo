//{"name":"F. Igor and Mountain","group":"Codeforces - Codeforces Round 1013 (Div. 3)","url":"https://codeforces.com/contest/2091/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n3 4 1\nXX#X\n#XX#\n#X#X\n3 4 2\nXX#X\n#XX#\n#X#X\n3 1 3\nX\nX\n#\n","output":"2\n60\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_size();
    let map = input.read_char_table(n, m);

    type Mod = ModIntF;
    let next = d - 1;
    let cur = d;
    let mut mem = Memoization3d::new(n, m, 2, |mem, r, c, s| -> (Mod, Mod) {
        let mut res = Mod::zero();
        if map[(r, c)] == b'X' {
            if r == 0 {
                res += Mod::one();
            } else {
                res += mem.call(r - 1, (c + next).min(m - 1), 0).1;
                if c >= next + 1 {
                    res -= mem.call(r - 1, c - next - 1, 0).1;
                }
            }
            if s == 0 {
                res += mem.call(r, (c + cur).min(m - 1), 1).1;
                if c >= cur + 1 {
                    res -= mem.call(r, c - cur - 1, 1).1;
                }
                res -= mem.call(r, c, 1).0;
            }
        }
        if c > 0 {
            let sum = res + mem.call(r, c - 1, s).1;
            (res, sum)
        } else {
            (res, res)
        }
    });
    let ans = mem.call(n - 1, m - 1, 0).1;
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
