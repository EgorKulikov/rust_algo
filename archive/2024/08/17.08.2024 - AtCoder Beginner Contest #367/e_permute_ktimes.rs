//{"name":"E - Permute K times","group":"AtCoder - AtCoder Beginner Contest 367","url":"https://atcoder.jp/contests/abc367/tasks/abc367_e","interactive":false,"timeLimit":2000,"tests":[{"input":"7 3\n5 2 6 3 1 4 6\n1 2 3 5 7 9 11\n","output":"7 2 3 5 1 9 3\n"},{"input":"4 0\n3 4 1 2\n4 3 2 1\n","output":"4 3 2 1\n"},{"input":"9 1000000000000000000\n3 7 8 5 9 3 7 4 2\n9 9 8 2 4 4 3 5 3\n","output":"3 3 3 3 3 3 3 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPermuteKTimes"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut x = input.read_size_vec(n).dec();
    let a = input.read_int_vec(n);

    let mut ans = (0..n).collect_vec();
    let mut n_ans = vec![0; n];
    let mut n_x = vec![0; n];
    for i in 0..60 {
        if k.is_set(i) {
            for j in 0..n {
                n_ans[j] = ans[x[j]];
            }
            swap(&mut ans, &mut n_ans);
        }
        for j in 0..n {
            n_x[j] = x[x[j]];
        }
        swap(&mut x, &mut n_x);
    }
    let mut n_a = vec![0; n];
    for i in 0..n {
        n_a[i] = a[ans[i]];
    }
    out.print_line(n_a);
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
