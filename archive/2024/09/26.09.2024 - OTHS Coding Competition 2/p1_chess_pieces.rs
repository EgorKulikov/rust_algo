//{"name":"P1 - Chess Pieces","group":"DMOJ - OTHS Coding Competition 2","url":"https://dmoj.ca/problem/othscc2p1","interactive":false,"timeLimit":1000,"tests":[{"input":"queen\n","output":"9\n"},{"input":"king\n","output":"priceless\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1ChessPieces"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let costs = [
        (b"pawn" as &[u8], "1"),
        (b"knight", "3"),
        (b"bishop", "3"),
        (b"rook", "5"),
        (b"queen", "9"),
        (b"king", "priceless"),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let s = input.read_str();
    out.print_line(costs[&s.as_slice()]);
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
