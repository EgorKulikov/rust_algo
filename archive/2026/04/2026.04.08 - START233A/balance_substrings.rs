//{"name":"Balance Substrings","group":"CodeChef - START233A","url":"https://www.codechef.com/START233A/problems/BALSUB7","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1\n2\n3\n343\n900000\n","output":"2\n26\n816\n491096447\n113011126\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    type Mod = ModIntF;

    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = Mod::new(0);
    let mut p2 = Mod::new(1);
    for a in 0..=n / 2 {
        let mut cur = c.c(n - a, a) * p2;
        if n > 2 * a {
            cur += c.c(n - a - 1, a) * p2;
        }
        p2 *= 2;
        let mut g = (2 * n - 2 * a) * (a + 1) / 2;
        if 2 * a != n {
            g += (n - 2 * a) * (n - 2 * a - 1) / 2;
        }
        ans += Mod::from(g).power(n) * cur;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
