//{"name":"day_7","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_7"}}}

use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        scan!(input, "@: @", test: i64, s: EolVec<i64>);
        data.push((test, s.0));
    }

    // part 1
    {
        let mut ans = 0;
        for (test, s) in &data {
            let mut good = false;
            let mut rec = RecursiveFunction2::new(|rec, step: usize, val: i64| {
                if step == s.len() {
                    if &val == test {
                        good = true;
                    }
                    return;
                }
                rec.call(step + 1, val + s[step]);
                rec.call(step + 1, val * s[step]);
            });
            rec.call(1, s[0]);
            if good {
                ans += test;
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for (test, s) in &data {
            let mut good = false;
            let mut rec = RecursiveFunction2::new(|rec, step: usize, val: i64| {
                if step == s.len() {
                    if &val == test {
                        good = true;
                    }
                    return;
                }
                rec.call(step + 1, val + s[step]);
                rec.call(step + 1, val * s[step]);
                rec.call(
                    step + 1,
                    (val.to_string() + &s[step].to_string()).parse().unwrap(),
                );
            });
            rec.call(1, s[0]);
            if good {
                ans += test;
            }
        }
        out.print_line(ans);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
