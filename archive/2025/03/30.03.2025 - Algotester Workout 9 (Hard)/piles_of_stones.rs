//{"name":"Piles of Stones","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136326","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\n2 1 2 3 2 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n);

    if a.copy_any(|x| x > k) {
        out.print_line(0);
        return;
    }
    let mut exp = 0;
    for part in a.split(|&x| x == k) {
        if part.is_empty() {
            continue;
        }
        if part[0] != k - 1 || part[Back(0)] != k - 1 {
            out.print_line(0);
            return;
        }
        for (a, b) in part.consecutive_iter_copy() {
            if (a - b).abs() > 1 {
                out.print_line(0);
                return;
            }
            if a == b {
                exp += 1;
            }
        }
        // for pp in part.split(|&x| x < k - 1) {
        //     if pp.is_empty() {
        //         continue;
        //     }
        //     exp += pp.len() - 1;
        // }
    }
    type Mod = ModInt7;
    out.print_line(Mod::new(2).power(exp));
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
