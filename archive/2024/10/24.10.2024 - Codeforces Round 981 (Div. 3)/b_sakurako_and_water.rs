//{"name":"B. Sakurako and Water","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1\n2\n-1 2\n3 0\n3\n1 2 3\n-2 1 -1\n0 0 -1\n5\n1 1 -1 -1 3\n-3 1 4 4 -4\n-1 -1 3 0 -5\n4 5 3 -3 -1\n3 1 -3 -1 5\n","output":"0\n1\n4\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSakurakoAndWater"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_table(n, n);

    let mut ans = 0;
    for i in 0..n {
        let mut left = 0;
        let mut right = 0;
        for j in 0..n - i {
            left.minim(a[(i + j, j)]);
            right.minim(a[(j, i + j)]);
        }
        ans -= left.min(0);
        if i != 0 {
            ans -= right.min(0);
        }
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
