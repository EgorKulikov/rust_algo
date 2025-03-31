//{"name":"Divisible Array","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136323","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n","output":"8\n"},{"input":"4 7\n","output":"43\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let d = all_divisors(n + 1, false);
    let mut p = vec![Vec::new(); n + 1];
    type Mod = ModInt7;
    let mut all = Vec::new();
    for i in 1..=n {
        p[i] = vec![Mod::one()];
        for j in d[i].copy_iter() {
            if j < i {
                if p[i].len() <= p[j].len() {
                    let new_len = p[j].len() + 1;
                    p[i].resize(new_len, Mod::zero());
                }
                for k in p[j].indices() {
                    let add = p[j][k];
                    p[i][k + 1] += add;
                }
            }
        }
        if all.len() < p[i].len() {
            all.resize(p[i].len(), Mod::zero());
        }
        for j in p[i].indices() {
            all[j] += p[i][j];
        }
    }
    let mut ans = Mod::zero();
    let mut c = Mod::one();
    for i in all.indices() {
        if i >= k {
            break;
        }
        ans += c * all[i];
        c *= Mod::from_index(k - i - 1);
        c /= Mod::from_index(i + 1);
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
