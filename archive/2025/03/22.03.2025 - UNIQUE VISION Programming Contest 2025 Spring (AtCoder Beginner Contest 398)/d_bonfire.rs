//{"name":"D - Bonfire","group":"AtCoder - UNIQUE VISION Programming Contest 2025 Spring (AtCoder Beginner Contest 398)","url":"https://atcoder.jp/contests/abc398/tasks/abc398_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6 -2 1\nNNEEWS\n","output":"001010\n"},{"input":"10 1 2\nNEESESWEES\n","output":"0001101011\n"},{"input":"20 -1 -2\nWWNNWSWEWNSWWENSNWWN\n","output":"00100111111000101111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let r = input.read_int();
    let c = input.read_int();
    let s = input.read_str();

    let mut x = 0;
    let mut y = 0;
    let mut set = FxHashSet::default();
    set.insert((0, 0));
    let mut ans = Str::new();
    for ch in s {
        match ch {
            b'N' => y += 1,
            b'E' => x -= 1,
            b'S' => y -= 1,
            b'W' => x += 1,
            _ => unreachable!(),
        }
        set.insert((x, y));
        if set.contains(&(x + c, y + r)) {
            ans.push(b'1');
        } else {
            ans.push(b'0');
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
