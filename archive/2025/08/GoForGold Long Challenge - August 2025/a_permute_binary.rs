//{"name":"A. Permute Binary","group":"Codeforces - GoForGold Long Challenge - August 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/629150/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n110 3\n0001 2\n","output":"2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let k = input.read_size();

    let ones = s.copy_count(b'1');
    let zeros = s.len() - ones;
    type Mod = ModInt7;
    let mut mem = Memoization3d::new(
        ones + 2,
        zeros + 1,
        k + 1,
        |mem, ones, zeroes, remainder| -> Mod {
            if ones == 0 && zeroes == 0 {
                if remainder == 0 {
                    Mod::one()
                } else {
                    Mod::zero()
                }
            } else {
                let mut res = Mod::zero();
                if ones > 0 {
                    res += mem.call(ones - 1, zeroes, (remainder * 2 + 1) % k);
                }

                if zeroes > 0 {
                    res += mem.call(ones, zeroes - 1, (remainder * 2) % k);
                }
                res
            }
        },
    );

    out.print_line(mem.call(ones, zeros, 0));
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
