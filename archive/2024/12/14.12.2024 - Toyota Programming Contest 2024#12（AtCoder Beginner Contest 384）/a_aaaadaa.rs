//{"name":"A - aaaadaa","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 b g\nabc\n","output":"gbg\n"},{"input":"1 s h\ns\n","output":"s\n"},{"input":"7 d a\natcoder\n","output":"aaaadaa\n"},{"input":"10 b a\nacaabcabba\n","output":"aaaabaabba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAaaadaa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let c1 = input.read_char();
    let c2 = input.read_char();
    let mut s = input.read_str();

    for c in s.iter_mut() {
        if *c != c1 {
            *c = c2;
        }
    }
    out.print_line(s);
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
