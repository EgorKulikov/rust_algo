//{"name":"F. Forgot to Refuel","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17265","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n","output":"1\n"},{"input":"5 3\n","output":"499122179\n"},{"input":"20 5\n","output":"796329095\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let n = input.read_size();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(l + 1);
    let mut ans = Mod::new(0);
    let mut val = Mod::new(0);
    for m in (1..=l).rev() {
        let mut base = l - 1;
        let mut fact = Mod::new(1);
        let mut cur = Mod::new(0);
        for k in 0..=n {
            cur += c.c(n, k) * c.c(base, n - 1) * fact;
            if base < m {
                break;
            }
            base -= m;
            fact *= -1;
        }
        if m != l {
            ans += (val - cur) * (m + 1);
        }
        val = cur;
    }
    ans += val;
    out.print_line(ans * l / n / c.c(l, n));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
