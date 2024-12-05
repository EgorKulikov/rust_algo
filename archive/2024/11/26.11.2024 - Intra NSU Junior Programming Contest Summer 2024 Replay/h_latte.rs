//{"name":"H - Latte","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6459","interactive":false,"timeLimit":2000,"tests":[{"input":"10 7 100\n","output":"9\n"},{"input":"2 1 100000000000\n","output":"1000000000\n"},{"input":"1000000000 1000000000 100\n","output":"0\n"},{"input":"1234 56789 314159265\n","output":"254309\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HLatte"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::NumDigs;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let x = input.read_long();

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right + 1) / 2;
        if a.saturating_mul(mid)
            .saturating_add(b.saturating_mul(mid.num_digs() as i64))
            <= x
        {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    out.print_line(left);
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
