//{"name":"E. The MEX Chef's Secret Recipe","group":"Toph","url":"https://toph.co/arena?contest=diu-code-trap-fall-2024-r#!/p/67f37555cf762cd9aec91ecf","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 2\n","output":"6\n"},{"input":"3\n3 1 4\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let q = qty(&a);
    let ones = q[1];
    let mut ans = (n - ones) * (n - ones + 1) / 2;
    if ones == 0 {
        out.print_line(ans);
        return;
    }
    let twos = q[2];
    if twos == 0 {
        ans += n * (n + 1) - (n - ones) * (n - ones + 1);
        out.print_line(ans);
        return;
    }
    ans += (n - twos) * (n - twos + 1) - (n - ones - twos) * (n - ones - twos + 1);
    let mut mex = 3;
    while q[mex] > 0 {
        mex += 1;
    }
    ans += mex * ones * twos;
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
