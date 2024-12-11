//{"name":"G. GCD","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9868","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 4\n12 20\n114 514\n","output":"3\n4\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGCD"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();

    let mut rec = RecursiveFunction3::new(|rec, a: i64, b: i64, limit: i64| -> i64 {
        if a == 0 && b == 0 || limit == 0 {
            return 0;
        }
        let g = gcd(a, b);
        if g != 1 {
            return rec.call(a / g, b / g, limit);
        }
        if a % 2 == 1 {
            let mut ans = rec.call(a - 1, b, limit - 1) + 1;
            if b != 0 {
                ans.minim(rec.call(a, b - 1, ans - 1) + 1);
            }
            ans
        } else {
            let mut ans = rec.call(a, b - 1, limit - 1) + 1;
            if a != 0 {
                ans.minim(rec.call(a - 1, b, ans - 1) + 1);
            }
            ans
        }
    });
    out.print_line(rec.call(a, b, i64::MAX));
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
