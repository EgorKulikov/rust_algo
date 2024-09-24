//{"name":"D. Tuples Fusion","group":"Codeforces - Theforces Round #34 (ABC-Forces)","url":"https://codeforces.com/gym/105350/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 1\n1000000000 1000000000\n3\n1 3\n2 6\n3 9\n6\n85 65\n35 59\n2 65\n789 21\n12 32\n9 8\n","output":"4000000004000000002\n446\n1220694\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTuplesFusion"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ab = input.read_long_pair_vec(n);

    let mut sum_best = 0;
    let mut sum_sq_worst = 0;
    for &(a, b) in &ab {
        sum_best += a.max(b);
        sum_sq_worst += a.min(b).power(2);
    }
    let mut ans = None;
    for (a, b) in ab {
        let cur_sum = sum_best + a.min(b);
        let cur_sq = sum_sq_worst - a.min(b).power(2);
        ans.maxim(cur_sq + cur_sum.power(2));
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
