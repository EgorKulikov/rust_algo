//{"name":"C. Uninteresting Number","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n123\n322\n333333333333\n9997\n5472778912773\n1234567890\n23\n33\n52254522632\n","output":"NO\nYES\nYES\nNO\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUninterestingNumber"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_str();

    let sum_digs = n.iter().fold(0, |a, c| a + (c - b'0') as usize);
    let twos = n.iter().count_eq(&b'2');
    let threes = n.iter().count_eq(&b'3');
    for i in 0..=twos.min(8) {
        for j in 0..=threes.min(2) {
            if (sum_digs + i * 2 + j * 6) % 9 == 0 {
                out.print_line(true);
                return;
            }
        }
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
