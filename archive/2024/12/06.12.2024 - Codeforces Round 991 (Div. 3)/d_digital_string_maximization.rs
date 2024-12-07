//{"name":"D. Digital string maximization","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n19\n1709\n11555\n51476\n9876543210\n5891917899\n","output":"81\n6710\n33311\n55431\n9876543210\n7875567711\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDigitalStringMaximization"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_str();

    for i in s.indices() {
        let mut best = None;
        for j in i..s.len().min(i + 9) {
            if (s[j] - b'0') as usize >= j - i {
                best.maxim(((s[j] - b'0') as usize - (j - i), Reverse(j)));
            }
        }
        let (_, Reverse(pos)) = best.unwrap();
        for j in (i..pos).rev() {
            s.swap(j, j + 1);
            s[j] -= 1;
        }
    }
    out.print_line(s);
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
