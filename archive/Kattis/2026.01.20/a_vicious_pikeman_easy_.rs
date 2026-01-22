//{"name":"A Vicious Pikeman (Easy)","group":"Kattis","url":"https://open.kattis.com/problems/pikemaneasy","interactive":false,"timeLimit":1000,"tests":[{"input":"1 3\n2 2 2 1\n","output":"1 1\n"},{"input":"2 10\n2 2 2 2\n","output":"2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let t0 = input.read_long();

    let p = Vec::with_gen_prefix(n, |i, p| {
        if i == 0 {
            t0
        } else {
            (a * p[i - 1] + b) % c + 1
        }
    })
    .sorted();
    let mut solved = 0;
    let mut penalty = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += p[i];
        if cur > t {
            break;
        }
        solved += 1;
        penalty += cur;
    }
    out.print_line((solved, ModInt7::from(penalty)));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
