//{"name":"B - Electric Bill","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6453","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n5 12\n6 24\n12 12\n9 8\n8 9\n3\n43 41\n86 92\n33 33\n","output":"492\n2 144\n10764\n2 7912\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BElectricBill"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let devices = input.read_size_pair_vec(n);

    out.print_line(devices.copy_fold(0, |acc, (a, b)| acc + a * b));
    out.print_line(
        devices
            .copy_map(|(a, b)| a * b)
            .enumerate()
            .max_by_key(|(i, a)| (*a, Reverse(*i)))
            .map(|(i, x)| (i + 1, x)),
    );
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
