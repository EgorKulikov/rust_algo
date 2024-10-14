//{"name":"A. Два экрана","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nGARAGE\nGARAGEFORSALE\nABCDE\nAABCD\nTRAINING\nDRAINING\n","output":"14\n10\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADvaEkrana"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    let mut common_prefix = 0;
    for i in 0..s.len().min(t.len()) {
        if s[i] == t[i] {
            common_prefix += 1;
        } else {
            break;
        }
    }
    out.print_line(s.len() + t.len() - common_prefix.max(1) + 1);
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
