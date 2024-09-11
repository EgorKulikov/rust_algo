//{"name":"Shooting (Hard)","group":"CodeChef - START151A","url":"https://www.codechef.com/START151A/problems/SHOOT","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 3\n0 3 1\n1 0 3\n1 2 2\n5 6\n0 1 2 3 3 1\n2 0 1 1 3 2\n2 2 1 3 1 0\n0 0 2 1 2 1\n1 2 1 2 0 3\n","output":"1 0 0\n0 1 3\n0 3 5\n6 3 1 1 3 3\n10 5 0 4 3 3\n10 7 0 2 4 2\n8 5 5 0 1 3\n5 6 5 6 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ShootingHard"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let a = input.read_int_table(n, m);

    let mut ans = Arr2d::new(n, m, 0i64);
    for (mult, bit) in [(1, 1), (-1, 2)] {
        let mut left_up_qty = Arr2d::new(n, m, 0i64);
        let mut right_down_qty = Arr2d::new(n, m, 0i64);
        let mut left_down_qty = Arr2d::new(n, m, 0i64);
        let mut right_up_qty = Arr2d::new(n, m, 0i64);
        let mut left_up_val = Arr2d::new(n, m, 0i64);
        let mut right_down_val = Arr2d::new(n, m, 0i64);
        let mut left_down_val = Arr2d::new(n, m, 0i64);
        let mut right_up_val = Arr2d::new(n, m, 0i64);
        for i in 0..n {
            for j in 0..m {
                if a[(i, j)] == bit {
                    left_up_qty[(i, j)] = 1;
                    if i + 1 < n && j > 0 {
                        left_down_qty[(i + 1, j - 1)] = 1;
                        left_down_val[(i + 1, j - 1)] = 1;
                    }
                    if i > 0 && j + 1 < m {
                        right_up_qty[(i - 1, j + 1)] = 1;
                        right_up_val[(i - 1, j + 1)] = 1;
                    }
                    right_down_qty[(i, j)] = 1;
                }
            }
        }
        for i in 1..n {
            for j in 0..m {
                if j > 0 {
                    right_down_val[(i, j)] +=
                        right_down_val[(i - 1, j - 1)] + right_down_qty[(i - 1, j - 1)];
                    right_down_qty[(i, j)] += right_down_qty[(i - 1, j - 1)];
                }
                if j + 1 < m {
                    left_down_val[(i, j)] +=
                        left_down_val[(i - 1, j + 1)] + left_down_qty[(i - 1, j + 1)];
                    left_down_qty[(i, j)] += left_down_qty[(i - 1, j + 1)];
                }
            }
        }
        for i in (0..n - 1).rev() {
            for j in 0..m {
                if j > 0 {
                    right_up_val[(i, j)] +=
                        right_up_val[(i + 1, j - 1)] + right_up_qty[(i + 1, j - 1)];
                    right_up_qty[(i, j)] += right_up_qty[(i + 1, j - 1)];
                }
                if j + 1 < m {
                    left_up_val[(i, j)] +=
                        left_up_val[(i + 1, j + 1)] + left_up_qty[(i + 1, j + 1)];
                    left_up_qty[(i, j)] += left_up_qty[(i + 1, j + 1)];
                }
            }
        }
        let mut left_qty = Arr2d::new(n, m, 0i64);
        let mut left_val = Arr2d::new(n, m, 0i64);
        let mut right_qty = Arr2d::new(n, m, 0i64);
        let mut right_val = Arr2d::new(n, m, 0i64);
        for i in 0..n {
            for j in (0..m).rev() {
                left_val[(i, j)] += left_down_val[(i, j)] + left_up_val[(i, j)];
                left_qty[(i, j)] += left_down_qty[(i, j)] + left_up_qty[(i, j)];
                if j < m - 1 {
                    left_val[(i, j)] += left_val[(i, j + 1)] + left_qty[(i, j + 1)];
                    left_qty[(i, j)] += left_qty[(i, j + 1)];
                }
            }
            for j in 0..m {
                right_val[(i, j)] += right_down_val[(i, j)] + right_up_val[(i, j)];
                right_qty[(i, j)] += right_down_qty[(i, j)] + right_up_qty[(i, j)];
                if j > 0 {
                    right_val[(i, j)] += right_val[(i, j - 1)] + right_qty[(i, j - 1)];
                    right_qty[(i, j)] += right_qty[(i, j - 1)];
                }
            }
        }
        let mut up_qty = Arr2d::new(n, m, 0i64);
        let mut up_val = Arr2d::new(n, m, 0i64);
        let mut down_qty = Arr2d::new(n, m, 0i64);
        let mut down_val = Arr2d::new(n, m, 0i64);
        for j in 0..m {
            for i in 0..n {
                down_val[(i, j)] += left_down_val[(i, j)] + right_down_val[(i, j)];
                down_qty[(i, j)] += left_down_qty[(i, j)] + right_down_qty[(i, j)];
                if i > 0 {
                    down_val[(i, j)] += down_val[(i - 1, j)] + down_qty[(i - 1, j)];
                    down_qty[(i, j)] += down_qty[(i - 1, j)];
                }
            }
            for i in (0..n).rev() {
                up_val[(i, j)] += left_up_val[(i, j)] + right_up_val[(i, j)];
                up_qty[(i, j)] += left_up_qty[(i, j)] + right_up_qty[(i, j)];
                if i < n - 1 {
                    up_val[(i, j)] += up_val[(i + 1, j)] + up_qty[(i + 1, j)];
                    up_qty[(i, j)] += up_qty[(i + 1, j)];
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                ans[(i, j)] += mult
                    * (left_val[(i, j)] + right_val[(i, j)] + up_val[(i, j)] + down_val[(i, j)]
                        - left_down_val[(i, j)]
                        - right_down_val[(i, j)]
                        - left_up_val[(i, j)]
                        - right_up_val[(i, j)]);
            }
        }
    }
    ans.iter_mut().for_each(|x| *x = x.abs());
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
