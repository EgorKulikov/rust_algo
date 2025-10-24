//{"name":"Minimize Maximum Mex","group":"CodeChef - START198A","url":"https://www.codechef.com/START198A/problems/MINMAXMEX","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n0 1\n1 0\n4\n2 3 1 2\n1 3 1 3\n4\n0 1 3 0\n0 3 0 1\n4\n0 1 2 3\n0 3 2 1\n","output":"1\n0\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut both = FxHashSet::default();
    let mut any = FxHashSet::default();
    for i in 0..n {
        if a[i] == b[i] {
            both.insert(a[i]);
        } else {
            any.insert(a[i]);
            any.insert(b[i]);
        }
    }
    let mut first = true;
    for i in 0.. {
        if both.contains(&i) {
            continue;
        }
        if any.contains(&i) {
            if first {
                first = false;
                continue;
            }
        }
        out.print_line(i);
        return;
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
