//{"name":"Number Coloring","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136325","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n","output":"1\n1 1\n"},{"input":"4\n","output":"2\n1 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut colors = Vec::new();
    if n < (1 << 2) {
        colors = vec![1];
    } else if n < (1 << 5) {
        colors = vec![1, 2, 2, 1];
    } else if n < (1 << 14) {
        let s = [b"1001000001001", b"0110000000110", b"0000111110000"];
        for i in 0..13 {
            for j in 0..3 {
                if s[j][12 - i] == b'1' {
                    colors.push(j + 1);
                    break;
                }
            }
        }
    } else {
        let s = [
            b"0000000000000000110",
            b"0000000000000000001",
            b"1111000000001111000",
            b"0000111111110000000",
        ];
        for i in 0..19 {
            for j in 0..4 {
                if s[j][18 - i] == b'1' {
                    colors.push(j + 1);
                    break;
                }
            }
        }
    }
    let pd = divisor_table(n + 1);
    let mut npd = vec![0; n + 1];
    for i in 2..=n {
        npd[i] = 1 + npd[i / pd[i]];
    }
    let mut ans = Vec::new();
    for i in 2..=n {
        ans.push(colors[npd[i] - 1]);
    }
    out.print_line(colors.copy_max());
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
