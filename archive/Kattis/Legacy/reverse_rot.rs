//{"name":"Reverse Rot","group":"Kattis","url":"https://open.kattis.com/problems/reverserot","interactive":false,"timeLimit":1000,"tests":[{"input":"1 ABCD\n3 YO_THERE.\n1 .DOT\n14 ROAD\n9 SHIFTING_AND_ROTATING_IS_NOT_ENCRYPTING\n2 STRING_TO_BE_CONVERTED\n1 SNQZDRQDUDQ\n0\n","output":"EDCB\nCHUHKWBR.\nUPEA\nROAD\nPWRAYF_LWNHAXWH.RHPWRAJAX_HMWJHPWRAORQ.\nFGVTGXPQEAGDAQVAIPKTVU\nREVERSE_ROT\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ReverseRot"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }
    let s = input.read_str();
    let order = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ_.";
    let ans = s
        .iter_rev()
        .map(|c| order[(order.copy_find(c).unwrap() + n) % 28])
        .collect::<Str>();
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
