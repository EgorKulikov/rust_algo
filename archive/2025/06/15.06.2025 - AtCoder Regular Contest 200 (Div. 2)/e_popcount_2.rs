//{"name":"E - popcount <= 2","group":"AtCoder - AtCoder Regular Contest 200 (Div. 2)","url":"https://atcoder.jp/contests/arc200/tasks/arc200_e","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2 3\n7 2\n2025 200\n","output":"56\n16384\n549499339\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModIntF;

    let m = Mod::from(m);
    let ans = (m + 1).power(n - 1) * (m + 1)
        + Mod::new(4).power(n - 1) * (m * 831870294 + m * m * m * 166374059)
        + Mod::new(3).power(n - 1) * (m * 499122177 + m * m * m * 499122176)
        + Mod::new(2).power(n - 1) * (m * 998244352 + m * m * 499122176 + m * m * m * 499122177)
        + m * 665496235
        + m * m * 499122177
        + m * m * m * 831870294;
    out.print_line(ans * Mod::new(2).power(m.val()));
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
