//{"name":"B - Hands on Ring (Easy)","group":"AtCoder - AtCoder Beginner Contest 376","url":"https://atcoder.jp/contests/abc376/tasks/abc376_b","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\nR 4\nL 5\nR 6\n","output":"8\n"},{"input":"100 2\nL 1\nR 2\n","output":"0\n"},{"input":"30 8\nR 23\nR 26\nR 29\nL 20\nR 29\nR 19\nL 7\nL 16\n","output":"92\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHandsOnRingEasy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    let mut ans = 0;
    let mut left = 0;
    let mut right = 1;
    for _ in 0..q {
        let h = input.read_char();
        let t = input.read_size() - 1;
        match h {
            b'L' => {
                if left < right && right < t || left > right && right > t {
                    ans += n - t.abs_diff(left);
                } else {
                    ans += t.abs_diff(left);
                }
                left = t;
            }
            b'R' => {
                if left > right && left < t || left < right && left > t {
                    ans += n - t.abs_diff(right);
                } else {
                    ans += t.abs_diff(right);
                }
                right = t;
            }
            _ => unreachable!(),
        }
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
