//{"name":"E - Restoration of Compressed Number Sequence","group":"AtCoder - AtCoder Weekday Contest 0044 Beta","url":"https://atcoder.jp/contests/awc0044/tasks/awc0044_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n1 1 2 0 2\n","output":"48\n"},{"input":"4 2\n1 2 3 0\n","output":"0\n"},{"input":"10 6\n1 0 2 0 2 3 0 1 4 0\n","output":"51840\n"},{"input":"24 15\n1 0 2 0 3 0 1 4 0 2 5 0 0 3 6 0 4 0 6 7 0 5 0 7\n","output":"280519446\n"},{"input":"1 1\n0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut mem = Memoization2d::new(n + 1, m.min(n) + 1, |mem, pos, len| -> Mod {
        if pos == 0 {
            if len == 0 {
                Mod::new(1)
            } else {
                Mod::new(0)
            }
        } else {
            if len == 0 {
                Mod::new(0)
            } else if d[pos - 1] > len {
                Mod::new(0)
            } else if d[pos - 1] == len {
                mem.call(pos - 1, len - 1) + mem.call(pos - 1, len)
            } else if d[pos - 1] > 0 {
                mem.call(pos - 1, len)
            } else {
                mem.call(pos - 1, len - 1) + mem.call(pos - 1, len) * len
            }
        }
    });
    let mut ans = Mod::new(0);
    let mut mult = Mod::new(1);
    for i in 1..=n.min(m) {
        mult *= m + 1 - i;
        ans += mult * mem.call(n, i);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
