//{"name":"E. GCD and LCM in Perfect Sync","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 3\n4 3\n666 999\n1000000000 1000000000\n","output":"6\n12\n15861573\n208999360\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a1 = input.read_u64();
    let n = input.read_size();

    type Mod = ModIntF;
    let pd = a1.prime_divisors();

    let mut mem = Memoization1d::new(pd.len() + 1, |mem, step| -> Mod {
        if step == pd.len() {
            Mod::one()
        } else {
            let q = pd[step].1;
            (Mod::from_index(q + 1).power(n)
                - Mod::from_index(q).power(n - 1) * Mod::from_index(2 * q)
                + Mod::from_index(q - 1).power(n - 1) * Mod::from_index(q - 1))
                * mem.call(step + 1)
        }
    });
    out.print_line(mem.call(0));
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
