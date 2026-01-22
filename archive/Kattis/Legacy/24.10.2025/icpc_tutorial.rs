//{"name":"ICPC Tutorial","group":"Kattis","url":"https://open.kattis.com/problems/tutorial","interactive":false,"timeLimit":1000,"tests":[{"input":"100000000 500 3\n","output":"TLE\n"},{"input":"100000000 50 3\n","output":"AC\n"},{"input":"100000000 10001 5\n","output":"TLE\n"},{"input":"100000000 10000 5\n","output":"AC\n"},{"input":"19931568 1000000 6\n","output":"TLE\n"},{"input":"19931569 1000000 6\n","output":"AC\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();
    let k = input.read_size();

    let res = if k == 1 {
        if n > 20 {
            out.print_line("TLE");
            return;
        }
        let mut x: usize = 1;
        for i in 1..=n {
            x = x.saturating_mul(i);
        }
        x
    } else if k == 2 {
        if n > 30 {
            out.print_line("TLE");
            return;
        }
        1 << n
    } else if k <= 5 {
        let mut x: usize = 1;
        for _ in 0..7 - k {
            x = x.saturating_mul(n);
        }
        x
    } else if k == 7 {
        n
    } else {
        (n.into_real().log2().into_real() * n).ceil() as usize
    };
    if res <= m {
        out.print_line("AC");
    } else {
        out.print_line("TLE");
    }
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
