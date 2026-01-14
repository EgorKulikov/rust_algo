//{"name":"Three Strings Problem (Easy Version)","group":"CodeChef - START221A","url":"https://www.codechef.com/START221A/problems/LMP7E","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n0\n0\n0\n3\n010\n010\n010\n4\n1100\n1010\n1001\n5\n00000\n11111\n01010\n6\n101010\n010101\n111000\n8\n00110101\n00100111\n11110000\n","output":"0\n0\n0\n3\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_str();
    let b = input.read_str();
    let c = input.read_str();

    let mut q001 = 0;
    let mut q010 = 0;
    let mut q101 = 0;
    let mut q110 = 0;
    for i in 0..n {
        if b[i] != c[i] {
            if b[i] == b'0' {
                if a[i] == b'0' {
                    q001 += 1;
                } else {
                    q101 += 1;
                }
            } else {
                if a[i] == b'0' {
                    q010 += 1;
                } else {
                    q110 += 1;
                }
            }
        }
    }
    let q0 = q001.min(q010);
    q001 -= q0;
    q010 -= q0;
    let q1 = q101.min(q110);
    q101 -= q1;
    q110 -= q1;
    if q001 == 0 {
        if q101 == 0 {
            out.print_line(q010 + q110);
        } else {
            out.print_line(q010.max(q101));
        }
    } else {
        if q101 == 0 {
            out.print_line(q001.max(q110));
        } else {
            out.print_line(q001 + q101);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
