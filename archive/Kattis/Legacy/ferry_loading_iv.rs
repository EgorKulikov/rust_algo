//{"name":"Ferry Loading IV","group":"Kattis","url":"https://open.kattis.com/problems/ferryloading4","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n20 4\n380 left\n720 left\n1340 right\n1040 left\n15 4\n380 left\n720 left\n1340 right\n1040 left\n15 4\n380 left\n720 left\n1340 left\n1040 left\n15 4\n380 right\n720 right\n1340 right\n1040 right\n","output":"3\n3\n5\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size() * 100;
    let m = input.read_size();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for _ in 0..m {
        let len = input.read_size();
        let side = input.read_str();
        if side.as_slice() == b"left" {
            left.push(len);
        } else {
            right.push(len);
        }
    }

    let mut ans = 0;
    for (cars, delta) in [(left, 1), (right, 0)] {
        let mut sum = 0;
        let mut cur = 1;
        for len in cars {
            if sum + len > l {
                cur += 1;
                sum = 0;
            }
            sum += len;
        }
        ans.maxim(2 * cur - delta);
    }
    out.print_line(ans);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
