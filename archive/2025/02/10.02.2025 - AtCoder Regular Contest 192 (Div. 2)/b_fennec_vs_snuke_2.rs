//{"name":"B - Fennec VS. Snuke 2","group":"AtCoder - AtCoder Regular Contest 192 (Div. 2)","url":"https://atcoder.jp/contests/arc192/tasks/arc192_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 9 2\n","output":"Fennec\n"},{"input":"2\n25 29\n","output":"Snuke\n"},{"input":"6\n1 9 2 25 2 9\n","output":"Snuke\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let even = a.iter().filter(|&x| x % 2 == 0).count();
    let odd = n - even;
    if odd >= 3 {
        if odd % 2 == 1 {
            out.print_line("Fennec");
        } else {
            out.print_line("Snuke");
        }
    } else if odd == 2 {
        if even == 1 {
            out.print_line("Fennec");
        } else {
            out.print_line("Snuke");
        }
    } else if odd == 1 {
        if even == 1 {
            out.print_line("Snuke");
        } else {
            out.print_line("Fennec");
        }
    } else {
        if even == 1 {
            out.print_line("Fennec");
        } else {
            out.print_line("Snuke");
        }
    }
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

//START MAIN
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
//END MAIN
