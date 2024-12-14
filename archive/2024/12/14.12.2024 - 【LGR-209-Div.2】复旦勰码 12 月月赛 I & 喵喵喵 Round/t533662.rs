//{"name":"T533662 喵喵喵幼儿园","group":"Luogu","url":"https://www.luogu.com.cn/problem/T533662?contestId=218363","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nyummy or yucky?\neat or stick?\nrua or eat?\neat or eat?\n","output":"yummy\nstick\nrua\nor\n"},{"input":"7\nsaint or sinner?\ntobe or nottobe?\nad or able?\neating or eaten?\nor or or?\nand or and?\noneplusoneistwo or oneplusoneisthree?\n","output":"sinner\ntobe\nable\neaten\nor\nand\noneplusoneistwo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T533662"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    scan!(input, "@ or @?", a: Str, b: Str);

    if a.as_slice() != b"eat" {
        out.print_line(a);
    } else if b.as_slice() != b"eat" {
        out.print_line(b);
    } else {
        out.print_line("or");
    }
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
