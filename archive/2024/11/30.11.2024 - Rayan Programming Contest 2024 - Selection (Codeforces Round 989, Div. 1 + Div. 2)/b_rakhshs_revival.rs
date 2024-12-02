//{"name":"B. Rakhsh's Revival","group":"Codeforces - Rayan Programming Contest 2024 - Selection (Codeforces Round 989, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2034/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 1 1\n10101\n5 2 1\n10101\n6 3 2\n000000\n","output":"2\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRakhshsRevival"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let mut next = m - 1;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == b'1' {
            next.maxim(i + m);
        }
        if i == next {
            ans += 1;
            next = i + k + m - 1;
        }
    }
    out.print_line(ans);
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
