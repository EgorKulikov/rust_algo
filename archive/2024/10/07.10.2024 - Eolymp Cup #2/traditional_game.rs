//{"name":"Traditional Game","group":"Eolymp - Basecamp - Eolymp Cup #2","url":"https://basecamp.eolymp.com/en/compete/ptmnufrm6p6nl7gods1loo65go/problem/4","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n33 25 29 28 12 35 10 1\n38 6 5 22 26 15 2 16\n","output":"-231\n"},{"input":"5\n5 9 21 38 13\n33 6 15 19 32\n","output":"20\n"},{"input":"10\n9 2 16 24 29 7 1 4 8 22\n37 10 3 27 30 5 14 18 28 35\n","output":"135\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TraditionalGame"}}}

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

    let mut ans = -a[0] * (n as i64 - 1);
    let mut max_a = a[0];
    for i in 1..n - 1 {
        if b[i] > max_a {
            ans.maxim(-a[0] * (i as i64) + b[i] * (n as i64 - i as i64 - 1));
        }
        max_a.maxim(a[i]);
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
