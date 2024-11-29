//{"name":"D. Mimicry","group":"Toph","url":"https://toph.co/arena?contest=diu-take-off-fall-24-preliminary-mock#!/p/6745c3e1cb14b2a6cffe49f7","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n9 2 9\n7\n1 2 1 3 4 9 3\n8\n7 2 3 4 5 7 4 4\n5\n1 2 3 4 1\n3\n100 200 100\n12\n10 15 89 10 45 86 45 45 4 45 97 97\n","output":"1\nNo\n2\nYes\n2\nYes\n1\nNo\n1\nNo\n4\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMimicry"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut expected = None;
    let mut len = 0;
    for i in a {
        if let Some(e) = expected {
            if i == e {
                expected = None;
                len += 1;
            }
        } else {
            expected = Some(i);
        }
    }
    out.print_line(len);
    if expected.is_some() {
        out.print_line("Oh No");
    } else {
        out.print_line(len % 2 == 0);
    }
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
