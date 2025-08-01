//{"name":"T633435 「Cfz Round 6」Imaichi","group":"Luogu","url":"https://www.luogu.com.cn/problem/T633435?contestId=255793","interactive":false,"timeLimit":1000,"tests":[{"input":"0 2\n3 3 1 5\n2 -1 0\n-3 -1 -1\n-1 1 -2\n2 3 1 3\n-3 1 -1\n0 -3 -2\n","output":"4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_long();
    let k = input.read_long();
    let a = input.read_long_table(n, m);

    let mut ans = vec![s; m];
    for i in 0..n {
        for j in 0..m {
            if ans[j] >= 0 {
                ans[j] += a[(i, j)];
                ans[j].minim(k);
            }
        }
        if i == n - 1 {
            break;
        }
        for j in 1..m {
            if ans[j - 1] >= 0 {
                let cand = (ans[j - 1] + a[(i, j)]).min(k);
                ans[j].maxim(cand);
            }
            if a[(i, j - 1)] + a[(i, j)] > 0 && ans[j - 1] >= 0 && ans[j] >= 0 {
                ans[j - 1].maxim(k + a[(i, j - 1)].min(0));
                ans[j].maxim(k + a[(i, j)].min(0));
            }
        }
        for j in (0..m - 1).rev() {
            if ans[j + 1] >= 0 {
                let cand = (ans[j + 1] + a[(i, j)]).min(k);
                ans[j].maxim(cand);
            }
            if a[(i, j + 1)] + a[(i, j)] > 0 && ans[j + 1] >= 0 && ans[j] >= 0 {
                ans[j + 1].maxim(k + a[(i, j + 1)].min(0));
                ans[j].maxim(k + a[(i, j)].min(0));
            }
        }
        for j in 1..m {
            if ans[j - 1] >= 0 {
                let cand = (ans[j - 1] + a[(i, j)]).min(k);
                ans[j].maxim(cand);
            }
            if a[(i, j - 1)] + a[(i, j)] > 0 && ans[j - 1] >= 0 && ans[j] >= 0 {
                ans[j - 1].maxim(k + a[(i, j - 1)].min(0));
                ans[j].maxim(k + a[(i, j)].min(0));
            }
        }
        for j in (0..m - 1).rev() {
            if ans[j + 1] >= 0 {
                let cand = (ans[j + 1] + a[(i, j)]).min(k);
                ans[j].maxim(cand);
            }
            if a[(i, j + 1)] + a[(i, j)] > 0 && ans[j + 1] >= 0 && ans[j] >= 0 {
                ans[j + 1].maxim(k + a[(i, j + 1)].min(0));
                ans[j].maxim(k + a[(i, j)].min(0));
            }
        }
        // for j in 1..m {
        //     if a[(i, j - 1)] + a[(i, j)] > 0 && ans[j - 1] >= 0 && ans[j] >= 0 {
        //         ans[j - 1].maxim(k + a[(i, j - 1)].min(0));
        //         ans[j].maxim(k + a[(i, j)].min(0));
        //     }
        // }
        // for j in 1..m {
        //     if ans[j - 1] >= 0 {
        //         let cand = (ans[j - 1] + a[(i, j)]).min(k);
        //         ans[j].maxim(cand);
        //     }
        // }
        // for j in (0..m - 1).rev() {
        //     if ans[j + 1] >= 0 {
        //         let cand = (ans[j + 1] + a[(i, j)]).min(k);
        //         ans[j].maxim(cand);
        //     }
        // }
    }
    out.print_line(ans.copy_max().max(-1));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    input.read_int();

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
