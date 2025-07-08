//{"name":"T565391 「LAOI-8」Count","group":"Luogu","url":"https://www.luogu.com.cn/problem/T565391?contestId=187787","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2 2 1\n","output":"16\n"},{"input":"6\n1 2 2 1 2 2\n","output":"104\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut dp = vec![Mod::zero(); 41];
    let mut rways = vec![Mod::zero(); n + 1];
    rways[n] = Mod::one();
    for i in (0..n).rev() {
        dp[a[i]] += rways[i + 1];
        rways[i] = dp[a[i]];
    }
    dp.fill(Mod::zero());
    let mut ways = Mod::one();
    let mut ans = Mod::zero();
    let mut dp2 = vec![Mod::zero(); 41];
    for i in 0..n {
        dp[a[i]] += ways;
        dp2[a[i]] += ways;
        for j in 0..=40 {
            dp2[j] *= a[i];
        }
        ways = dp[a[i]];
        ans += dp2[a[i]] * rways[i + 1];
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
