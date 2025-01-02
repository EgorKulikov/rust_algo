//{"name":"Max gcd group","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1151","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 5 6\n","output":"6 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaxGcdGroup"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let zeroes = a.copy_count(0);
    if zeroes == n {
        out.print_line(vec![0; n]);
        return;
    }
    let max = a.copy_max();
    let ad: Vec<Vec<usize>> = all_divisors(max + 1, false);
    let mut cnt = vec![0; max + 1];
    for i in a {
        for j in ad[i].copy_iter() {
            cnt[j] += 1;
        }
    }
    let mut ans = vec![0; n];
    for i in 1..=max {
        if cnt[i] > 0 {
            ans[cnt[i] - 1 + zeroes] = i;
        }
    }
    for i in (1..n).rev() {
        let cand = ans[i];
        ans[i - 1].maxim(cand);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
