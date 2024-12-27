//{"name":"Run-Length Encoding, Run!","group":"Kattis","url":"https://open.kattis.com/problems/runlengthencodingrun","interactive":false,"timeLimit":1000,"tests":[{"input":"E HHHeellloWooorrrrlld!!\n","output":"H3e2l3o1W1o3r4l2d1!2\n"},{"input":"D H3e2l3o1W1o3r4l2d1!2\n","output":"HHHeellloWooorrrrlld!!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RunLengthEncodingRun"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let command = input.read_char();
    let s = input.read_str();

    match command {
        b'E' => {
            let mut last = b' ';
            let mut count = 0;
            for c in s.copy_iter() {
                if c == last {
                    count += 1;
                } else {
                    if count > 0 {
                        out.print(last);
                        out.print(count);
                    }
                    last = c;
                    count = 1;
                }
            }
            if count > 0 {
                out.print(last);
                out.print(count);
            }
            out.print_line(());
        }
        b'D' => {
            for block in s.chunks_exact(2) {
                let c = block[0];
                let count = block[1] - b'0';
                for _ in 0..count {
                    out.print(c);
                }
            }
            out.print_line(());
        }
        _ => unreachable!(),
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
