//{"name":"P3 - ScanTron","group":"DMOJ - Arcadia Computing Contest 1","url":"https://dmoj.ca/problem/ahscc1p3","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nABCDA\nACDA\n","output":"4\n"},{"input":"15\nDACAABCACCAADBA\nDCDBDADACCBDDC\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3ScanTron"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    let mut tail = s
        .iter()
        .skip(1)
        .zip(t.iter())
        .filter(|(a, b)| a == b)
        .count();
    let mut ans = tail;
    let mut head = 0;
    for i in 0..n - 1 {
        if s[i] == t[i] {
            head += 1;
        }
        if s[i + 1] == t[i] {
            tail -= 1;
        }
        ans.maxim(head + tail);
    }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
