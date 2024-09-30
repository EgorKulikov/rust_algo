//{"name":"ucup_10_k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_10_k"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let f = input.read_int_vec(n);
    let pts = input.read_vec::<Point<i64>>(n);

    let area = |i1: usize, i2: usize, i3: usize| -> i64 {
        (pts[i1].x - pts[i3].x) * (pts[i2].y - pts[i1].y)
            - (pts[i1].x - pts[i2].x) * (pts[i3].y - pts[i1].y)
    };
    let mut any = Arr2d::new(n, n, 0);
    let mut same = Arr2d::new(n, n, 0);
    let mut ans = 0;
    for len in 1..n {
        for i in 0..n {
            let j = if i + len < n { i + len } else { i + len - n };
            let mut cur_any = 0;
            let mut cur_same = 0;
            for k in i..i + len {
                let k = if k < n { k } else { k - n };
                let area = area(i, k, j);
                cur_any.maxim(same[(i, k)] + area);
                if f[j] == f[k] {
                    cur_same.maxim(any[(k, j)] + area);
                }
            }
            any[(i, j)] = cur_any;
            same[(i, j)] = cur_same;
            if f[i] == f[j] {
                ans.maxim(cur_any);
            }
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
