//{"name":"C. Giving Directions in Harbin","group":"Universal Cup - The 3rd Universal Cup. Stage 14: Harbin","url":"https://contest.ucup.ac/contest/1817/problem/9521","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n2\nS 2\nE 1\n","output":"3 S\nZ 2\nL\nZ 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGivingDirectionsInHarbin"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let dirs = input.read_vec::<(u8, usize)>(n);

    out.print_line((2 * n - 1, dirs[0].0));
    out.print_line((b'Z', dirs[0].1));
    let d = b"NWSENWSE";
    for i in 1..n {
        for j in 1..5 {
            if dirs[i].0 == d[j] {
                if dirs[i - 1].0 == d[j - 1] {
                    out.print_line(b'L');
                } else {
                    out.print_line(b'R');
                }
                break;
            }
        }
        out.print_line((b'Z', dirs[i].1));
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
