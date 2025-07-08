//{"name":"j1","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_int();
    let p = input.read_int();
    let data = input
        .iter::<(Str, i32, i32)>()
        .take(n)
        .map(|(_, x, y)| (x, y))
        .collect::<Vec<_>>();

    let mut ans = 1;
    let mut any = DefaultHashMap::new(Vec::new());
    let mut strict = DefaultHashMap::new(Vec::new());
    let (solved, penalty) = data[0];
    any[(solved, (penalty - 1) % 20)].push(penalty - 1);
    if d < 20 {
        for i in (2..=(20 - d + 1).min(penalty)).rev() {
            strict[(solved, (penalty - i) % 20)].push(penalty - i);
        }
    }
    let mut last_solved = solved;
    let mut last_penalty = penalty;
    for i in 1..n {
        let (solved, penalty) = data[i];
        let last = 19.min(d - 1);
        ans += strict[(solved + 1, (penalty + last) % 20)].more_or_eq(&(penalty + last));
        for m in 0..=last {
            ans += any[(solved + 1, (penalty + m) % 20)].more_or_eq(&(penalty + m));
        }
        if solved == last_solved && solved != p {
            ans += 1;
        }
        if last_solved != solved {
            last_penalty = -1;
        }
        if penalty - 1 > last_penalty {
            any[(solved, (penalty - 1) % 20)].push(penalty - 1);
        }
        for m in 2..=20 - d + 1 {
            if penalty - m > last_penalty {
                strict[(solved, (penalty - m) % 20)].push(penalty - m);
            }
        }
        last_solved = solved;
        last_penalty = penalty;
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
