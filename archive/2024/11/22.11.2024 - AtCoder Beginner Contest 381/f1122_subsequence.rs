//{"name":"F - 1122 Subsequence","group":"AtCoder - AtCoder Beginner Contest 381","url":"https://atcoder.jp/contests/abc381/tasks/abc381_f","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 3 3 1 2 2 1\n","output":"4\n"},{"input":"1\n20\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1122Subsequence"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut next = Arr2d::new(n + 1, 20, n);
    for i in (0..n).rev() {
        for j in 0..20 {
            next[(i, j)] = next[(i + 1, j)];
        }
        next[(i, a[i])] = i;
    }
    let mut dp = vec![n; 1 << 20];
    dp[0] = 0;
    let mut ans = 0;
    for i in usize::iter_all(20) {
        for j in 0..20 {
            if i.is_set(j) {
                let mut cand = dp[i.without_bit(j)];
                if cand < n {
                    cand = next[(cand, j)];
                }
                if cand < n {
                    cand = next[(cand + 1, j)];
                }
                dp[i].minim(cand);
            }
        }
        if dp[i] != n {
            ans.maxim(i.count_ones());
        }
    }
    out.print_line(ans * 2);
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
