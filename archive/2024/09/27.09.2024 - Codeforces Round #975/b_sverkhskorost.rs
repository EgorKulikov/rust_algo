//{"name":"B. Сверхскорость","group":"Codeforces - Codeforces Round 975 (Div. 1)","url":"https://codeforces.com/contest/2018/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6\n6 3 3 3 5 5\n6\n5 6 4 1 4 5\n9\n8 6 4 2 1 3 5 7 9\n","output":"3\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSverkhskorost"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    let mut cur_left = order[0];
    let mut cur_right = order[0];
    let mut start_left = 0;
    let mut start_right = n - 1;
    for i in order {
        cur_left.minim(i);
        cur_right.maxim(i);
        if cur_right - cur_left + 1 > a[i] {
            out.print_line(0);
            return;
        }
        let delta = a[i] - (cur_right - cur_left + 1);
        let n_start_left = cur_left.saturating_sub(delta);
        start_left.maxim(n_start_left);
        let n_start_right = cur_right + delta;
        start_right.minim(n_start_right);
    }
    out.print_line(start_right - start_left + 1);
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
