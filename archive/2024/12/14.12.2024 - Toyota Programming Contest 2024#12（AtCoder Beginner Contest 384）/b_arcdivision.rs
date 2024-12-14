//{"name":"B - ARC Division","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 1255\n2 900\n1 521\n2 600\n1 52\n","output":"2728\n"},{"input":"2 3031\n1 1000\n2 -1000\n","output":"3031\n"},{"input":"15 2352\n2 -889\n2 420\n2 -275\n1 957\n1 -411\n1 -363\n1 151\n2 -193\n2 289\n2 -770\n2 109\n1 345\n2 551\n1 -702\n1 355\n","output":"1226\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BARCDivision"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut r = input.read_int();
    let contests = input.read_int_pair_vec(n);

    for (d, a) in contests {
        let (from, to) = if d == 1 { (1600, 2800) } else { (1200, 2400) };
        if r >= from && r < to {
            r += a;
        }
    }
    out.print_line(r);
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
