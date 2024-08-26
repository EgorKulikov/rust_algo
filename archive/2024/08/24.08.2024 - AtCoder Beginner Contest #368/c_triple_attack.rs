//{"name":"C - Triple Attack","group":"AtCoder - Hitachi Vantara Programming Contest 2024（AtCoder Beginner Contest 368）","url":"https://atcoder.jp/contests/abc368/tasks/abc368_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 2 2\n","output":"8\n"},{"input":"9\n1 12 123 1234 12345 123456 1234567 12345678 123456789\n","output":"82304529\n"},{"input":"5\n1000000000 1000000000 1000000000 1000000000 1000000000\n","output":"3000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTripleAttack"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_long_vec(n);

    let mut t = 0;
    for mut i in h {
        while i > 0 && t % 3 != 0 {
            if t % 3 == 2 {
                i -= 3;
            } else {
                i -= 1;
            }
            t += 1;
        }
        let full = i / 5;
        t += full * 3;
        i -= full * 5;
        while i > 0 {
            if t % 3 == 2 {
                i -= 3;
            } else {
                i -= 1;
            }
            t += 1;
        }
    }
    out.print_line(t);
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
