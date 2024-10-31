//{"name":"Buying Game","group":"CodeChef - START158A","url":"https://www.codechef.com/START158A/problems/BUYGAME","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 2 3\n3 2 1\n2\n1 2\n2 1\n","output":"4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BuyingGame"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut ans = 0;
    let mut bs = 0;
    let mut best_delta = i64::MAX;
    let mut delta = 0;
    for i in 0..n {
        if a[i] < b[i] {
            ans += a[i];
            delta = b[i] - a[i];
        } else {
            ans += b[i];
            bs += 1;
            best_delta.minim(a[i] - b[i]);
        }
    }
    if bs == n - 1 {
        ans += best_delta.min(delta);
    }
    out.print_line(ans);
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
