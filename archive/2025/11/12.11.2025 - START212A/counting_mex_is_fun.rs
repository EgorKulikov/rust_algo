//{"name":"Counting MEX Is Fun","group":"CodeChef - START212A","url":"https://www.codechef.com/START212A/problems/P6BAR","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n2 1\n2 2\n3 3\n","output":"1\n0\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if n == 1 {
        if k == 1 {
            out.print_line(1);
        } else {
            out.print_line(0);
        }
        return;
    }
    if k < 2 {
        out.print_line(0);
        return;
    }

    type Mod = ModInt7;
    let mut mem = Memoization2d::new(n + 1, k - 1, |mem, cur, rem| {
        if cur == n {
            if rem == 0 {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let mut res = mem.call(cur + 1, rem) * (cur - 1);
            if rem > 0 {
                res += mem.call(cur + 1, rem - 1) * 2;
            }
            res
        }
    });
    out.print_line(mem.call(1, k - 2));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
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
