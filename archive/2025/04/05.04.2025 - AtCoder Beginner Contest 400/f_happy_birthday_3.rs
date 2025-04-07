//{"name":"F - Happy Birthday! 3","group":"AtCoder - AtCoder Beginner Contest 400","url":"https://atcoder.jp/contests/abc400/tasks/abc400_f","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1 4 2 1 2 5\n1 2 3 4 5 6\n","output":"20\n"},{"input":"5\n1 2 3 4 5\n1000000000 1000000000 1000000000 1000000000 1000000000\n","output":"5000000005\n"},{"input":"8\n2 3 3 1 2 1 3 1\n3 4 1 2 5 3 1 2\n","output":"23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size_vec(n).dec();
    let x = input.read_size_vec(n);

    if c.copy_count(c[0]) == n {
        out.print_line(x[c[0]] + n);
        return;
    }
    let next = Vec::with_gen_back(n + 1, |i, next| {
        if i == n {
            let mut res = 0;
            while c[res] == c[0] {
                res += 1;
            }
            n + res
        } else {
            if c[(i + 1) % n] == c[i] {
                next[i + 1]
            } else {
                i + 1
            }
        }
    });
    let mut mem = Memoization3d::new(n, n, n + 1, |mem, mut from, mut to, col| -> usize {
        if from > to {
            to += n;
        }
        if c[from] == col {
            from = next[from];
        }
        if from >= to {
            return 0;
        }
        if from >= n {
            from -= n;
            to -= n;
        }
        let mut res = usize::MAX;
        for i in from..to {
            if c[i % n] == c[from] {
                res.minim(
                    mem.call(from, (i + 1) % n, c[i % n])
                        + mem.call((i + 1) % n, to % n, col)
                        + x[c[i % n]]
                        + i
                        - from
                        + 1,
                );
            }
        }
        res
    });
    let mut ans = usize::MAX;
    for i in 0..n {
        for j in i..i + n {
            if c[j % n] == c[i] {
                ans.minim(
                    mem.call((i + 1) % n, (j + 1) % n, c[i])
                        + mem.call((j + 1) % n, i, n)
                        + x[c[i]]
                        + j
                        - i
                        + 1,
                );
            }
        }
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
