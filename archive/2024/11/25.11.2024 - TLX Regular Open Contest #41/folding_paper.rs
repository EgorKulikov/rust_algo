//{"name":"Folding Paper","group":"TLX - TLX Regular Open Contest #41","url":"https://tlx.toki.id/contests/troc-41/problems/A","interactive":false,"timeLimit":1000,"tests":[{"input":"12 10 15\n","output":"YA\n"},{"input":"3 3 3\n","output":"TIDAK\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FoldingPaper"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut l = input.read_size();
    let mut w = input.read_size();
    let a = input.read_size();

    while l % 2 == 0 && l * w != a {
        l /= 2;
    }
    while w % 2 == 0 && l * w != a {
        w /= 2;
    }
    if l * w == a {
        out.print_line("YA\n");
    } else {
        out.print_line("TIDAK\n");
    }
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
