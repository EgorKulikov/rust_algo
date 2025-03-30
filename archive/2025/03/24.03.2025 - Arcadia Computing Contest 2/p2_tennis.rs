//{"name":"P2 - Tennis","group":"DMOJ - Arcadia Computing Contest 2","url":"https://dmoj.ca/problem/ahscc2p2","interactive":false,"timeLimit":2000,"tests":[{"input":"8 2\nSSFSPFSF\n","output":"YES\n6\n"},{"input":"6 2\nSPSPSP\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    if s.str_contains(b"PP") {
        out.print_line(false);
        return;
    }
    let mut last_f = 0;
    for i in 0..n {
        if s[i] == b'F' {
            if i - last_f > k {
                out.print_line(false);
                return;
            }
            last_f = i + 1;
        }
    }
    if n - last_f > k {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    out.print_line(s.copy_count(b'P') * 2 + s.copy_count(b'S'));
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
