//{"name":"B - 1122 String","group":"AtCoder - AtCoder Beginner Contest 381","url":"https://atcoder.jp/contests/abc381/tasks/abc381_b","interactive":false,"timeLimit":2000,"tests":[{"input":"aabbcc\n","output":"Yes\n"},{"input":"aab\n","output":"No\n"},{"input":"zzzzzz\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1122String"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    out.set_bool_output(BoolOutput::YesNo);
    if s.len() % 2 != 0 {
        out.print_line(false);
        return;
    }
    let mut set = FxHashSet::default();
    for i in s.indices().step_by(2) {
        if s[i] != s[i + 1] {
            out.print_line(false);
            return;
        }
        set.insert(s[i]);
    }
    out.print_line(set.len() * 2 == s.len());
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
