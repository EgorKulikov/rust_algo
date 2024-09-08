//{"name":"uc9_n","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc9_n"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    input.read_line();
    scan!(input, "for @ in range(@,@):\n", i: char, from_1: i64, to_1: Str<'static>);
    scan!(input, "    for @ in range(@,@):\n", _j: char, from_2: Str<'static>, to_2: Str<'static>);
    input.read_line();
    input.read_line();
    let (to_1, step_1) = if to_1.iter().find_eq(b',').is_some() {
        let tokens = to_1.split(b',');
        (
            tokens[0].clone().parse::<i64>(),
            tokens[1].clone().parse::<i64>(),
        )
    } else {
        (to_1.parse(), 1)
    };
    let from_2 = if from_2[0] == i as u8 {
        None
    } else {
        Some(from_2.parse::<i64>())
    };
    let (to_2, step_2) = if to_2.iter().find_eq(b',').is_some() {
        let tokens = to_2.split(b',');
        (
            if tokens[0][0] == i as u8 {
                None
            } else {
                Some(tokens[0].clone().parse::<i64>())
            },
            if tokens[1][0] == i as u8 {
                None
            } else {
                Some(tokens[1].clone().parse::<i64>())
            },
        )
    } else {
        (
            if to_2[0] == i as u8 {
                None
            } else {
                Some(to_2.parse())
            },
            Some(1),
        )
    };
    let mut ans = 0;
    let mut i = from_1;
    while step_1 > 0 && i < to_1 || step_1 < 0 && i > to_1 {
        let from_2 = from_2.unwrap_or(i);
        let to_2 = to_2.unwrap_or(i);
        let step_2 = step_2.unwrap_or(i);

        if step_2 > 0 {
            if to_2 > from_2 {
                let end = ((to_2 - 1 - from_2) / step_2) * step_2 + from_2;
                ans += (from_2 + end) * ((end - from_2) / step_2 + 1) / 2;
            }
        } else {
            if to_2 < from_2 {
                let end = ((from_2 - 1 - to_2) / (-step_2)) * step_2 + from_2;
                ans += (from_2 + end) * ((from_2 - end) / (-step_2) + 1) / 2;
            }
        }

        i += step_1;
    }
    out.print_line(ans);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
