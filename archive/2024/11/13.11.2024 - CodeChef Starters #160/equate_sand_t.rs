//{"name":"Equate S and T","group":"CodeChef - START160A","url":"https://www.codechef.com/START160A/problems/EQUATEST","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 5\nababb\nabbab\n1 2\nb\nbb\n","output":"Yes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EquateSAndT"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let _m = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    out.print_line(
        s.iter().count_eq(&b'a') == t.iter().count_eq(&b'a')
            && s.iter().take_while(|&c| c == b'b').count()
                == t.iter().take_while(|&c| c == b'b').count(),
    );
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
