//{"name":"Non-matching Name","group":"CodeChef - START157A","url":"https://www.codechef.com/START157A/problems/ABNOMAT","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n8 7\ndijkstra\nblossom\n24 19\nfastquicklazypropagation\nwestmajixhoverboard\n34 45\nsupercalifragilisticexpialidocious\npneumonoultramicroscopicsilicovolcanoconiosis\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NonMatchingName"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    input.read_size();
    input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    let mut mask = 0;
    for c in s.into_iter().chain(t) {
        mask.set_bit(c as usize - 'a' as usize);
    }
    out.print_line(mask != i32::all_bits(26));
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
