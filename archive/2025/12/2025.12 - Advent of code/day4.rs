//{"name":"day4","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D8;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_lines();

    let n = s.len();
    let m = s[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'.' {
                continue;
            }
            let mut q = 0;
            for (r, c) in D8::iter(i, j, n, m) {
                if s[r][c] == b'@' {
                    q += 1;
                }
            }
            if q < 4 {
                ans += 1;
            }
        }
    }
    out.print_line(ans);
    let mut ans2 = 0;
    loop {
        let mut updated = false;
        for i in 0..n {
            for j in 0..m {
                if s[i][j] == b'.' {
                    continue;
                }
                let mut q = 0;
                for (r, c) in D8::iter(i, j, n, m) {
                    if s[r][c] == b'@' {
                        q += 1;
                    }
                }
                if q < 4 {
                    ans2 += 1;
                    s[i][j] = b'.';
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }
    out.print_line(ans2);
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
