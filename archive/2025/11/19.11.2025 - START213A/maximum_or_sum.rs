//{"name":"Maximum OR Sum","group":"CodeChef - START213A","url":"https://www.codechef.com/START213A/problems/MAXOSMR","interactive":false,"timeLimit":2500,"tests":[{"input":"6\n1 1\n1 3\n2 4\n4 8\n5 16\n39 2025\n","output":"2\n44\n3816\n142605239\n708796143\n571614584\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModIntF;
    let vars = n * (n + 1) / 2 + 1;

    let e2n = Mod::new(2).power(n);
    let p = powers(Mod::new(2), m + 1);
    let p2 = powers(e2n, m + 1);
    if vars > m {
        let mut ans = Mod::zero();
        for i in 0..m {
            ans += p[i] * p2[m];
        }
        out.print_line(ans);
        return;
    }

    let mut mem1 = Memoization1d::new(m + 1, |mem, pos| {
        if pos == 0 {
            Mod::zero()
        } else {
            let sum = mem.call(pos - 1);
            let qty = p2[pos - 1];
            sum * e2n + (e2n - Mod::one()) * p[pos - 1] * qty
        }
    });
    let mut a_sum = vec![Mod::zero(); vars + 1];
    for pos in 1..=m {
        for rem in (2..=vars).rev() {
            let sum1 = a_sum[rem];
            let sum2 = a_sum[rem - 1];
            a_sum[rem] = sum1 * (e2n - rem) + sum2 * rem + p[pos - 1] * p2[pos];
        }
        a_sum[1] = mem1.call(pos);
    }
    out.print_line(a_sum[vars]);
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
