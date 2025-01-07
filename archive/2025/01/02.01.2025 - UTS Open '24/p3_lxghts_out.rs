//{"name":"P3 - LXghts Out","group":"DMOJ - UTS Open '24","url":"https://dmoj.ca/problem/utso24p3","interactive":false,"timeLimit":1000,"tests":[{"input":"6 1\n110110\n","output":"5\n"},{"input":"4 2\n0000\n","output":"0\n"},{"input":"2 0\n11\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3LXghtsOut"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let mut num_even = 0;
    let mut num_ones: usize = 0;
    let mut start = 0;
    for i in 0..n {
        if s[i] == b'0' {
            let len = i - start;
            if len > 0 && len % 2 == 0 {
                num_even += 1;
            }
            start = i + 1;
        } else {
            num_ones += 1;
        }
    }
    let len = n - start;
    if len > 0 && len % 2 == 0 {
        num_even += 1;
    }
    if num_even * 3 == n + 1 && k == 0 {
        out.print_line(-1);
        return;
    }
    for _ in 0..k {
        num_even = num_even.saturating_sub(1);
        num_ones = num_ones.saturating_sub(1);
    }
    out.print_line(num_even * 2 + num_ones);
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
