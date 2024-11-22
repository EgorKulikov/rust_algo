//{"name":"C - 11/22 Substring","group":"AtCoder - AtCoder Beginner Contest 381","url":"https://atcoder.jp/contests/abc381/tasks/abc381_c","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n211/2212\n","output":"5\n"},{"input":"5\n22/11\n","output":"1\n"},{"input":"22\n/1211/2///2111/2222/11\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1122Substring"}}}

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

    let mut ans = 0;
    for i in 0..n {
        if s[i] == b'/' {
            let mut max = 0;
            for j in 1..=i.min(n - i - 1) {
                if s[i - j] == b'1' && s[i + j] == b'2' {
                    max = j;
                } else {
                    break;
                }
            }
            ans.maxim(2 * max + 1);
        }
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
