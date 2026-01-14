//{"name":"B - Magic Square","group":"AtCoder - AtCoder Beginner Contest 436","url":"https://atcoder.jp/contests/abc436/tasks/abc436_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n","output":"8 1 6\n3 5 7\n4 9 2\n"},{"input":"5\n","output":"17 24 1 8 15\n23 5 7 14 16\n4 6 13 20 22\n10 12 19 21 3\n11 18 25 2 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Arr2d::new(n, n, 0);
    let mut r = 0;
    let mut c = n / 2;
    ans[(r, c)] = 1;
    for i in 2..=n * n {
        let cr = (r + n - 1) % n;
        let cc = (c + 1) % n;
        if ans[(cr, cc)] == 0 {
            ans[(cr, cc)] = i;
            r = cr;
            c = cc;
        } else {
            let cr = (r + 1) % n;
            let cc = c;
            assert_eq!(ans[(cr, cc)], 0);
            ans[(cr, cc)] = i;
            r = cr;
            c = cc;
        }
    }

    out.print_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

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
