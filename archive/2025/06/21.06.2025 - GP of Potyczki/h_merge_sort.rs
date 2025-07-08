//{"name":"H. Merge Sort","group":"Universal Cup - GP of Potyczki","url":"https://contest.ucup.ac/contest/2135/problem/12157","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 2 3\n5 4 1\n3 1 3\n","output":"24\n52\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;

type Mod = ModInt7;
type PreCalc = Combinations<Mod>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, comb: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size() - 1;
    let b = input.read_size() - 1;

    let (a, b) = (a.min(b), a.max(b));
    let x = b - a;

    // let hyper_geometric = |a: usize, b: usize, c: usize| -> Mod {
    //     comb.fact(c - 1)
    //         * comb.fact(c + a + b - 1)
    //         * comb.inv_fact(c + a - 1)
    //         * comb.inv_fact(c + b - 1)
    // };
    // let calc = |c: usize, d: usize, f: usize| -> Mod {
    //     (hyper_geometric(d, f, c + 1) - 1) * comb.inv_fact(c) * comb.inv_fact(d) * comb.inv_fact(f)
    // };
    let mut rec = Memoization::new(|mem, len| -> Mod {
        if len == 1 {
            Mod::zero()
        } else {
            let left = len / 2;
            let right = (len + 1) / 2;
            let mut res = Mod::zero();
            if n + left >= x + len {
                res += comb.fact(n - len + left - 1)
                    * comb.inv_fact(n + left - x - len)
                    * Mod::from(left)
                    * Mod::from(right)
                    * comb.fact(n - 1 - x)
            }
            if n + right >= x + len {
                res += comb.fact(n - len + right - 1)
                    * comb.inv_fact(n + right - x - len)
                    * Mod::from(right)
                    * Mod::from(left)
                    * comb.fact(n - 1 - x)
            }
            res + mem.call(left) + mem.call(right)
        }
    });
    out.print_line(rec.call(n));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Combinations::<Mod>::new(1_000_001);

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
