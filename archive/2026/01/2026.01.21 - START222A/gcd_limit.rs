//{"name":"GCD Limit","group":"CodeChef - START222A","url":"https://www.codechef.com/START222A/problems/GCDLIM","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n3\n4\n5\n190000\n","output":"499122177\n665496236\n831870295\n549034395\n714867861\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();

    type Mod = ModIntF;
    let ad = all_divisors(m + 1, true);
    let mut res = vec![Mod::zero(); 2];
    let mut ans = Mod::zero();
    for i in 2..=m {
        let mut q = vec![0; ad[i].len()];
        for j in ad[i].indices().rev() {
            q[j] = m / ad[i][j];
            for k in j + 1..ad[i].len() {
                if ad[i][k] % ad[i][j] == 0 {
                    q[j] -= q[k];
                }
            }
        }
        let mut cur = Mod::zero();
        for j in 0..q.len() - 1 {
            cur += (res[ad[i][j]] + 1) * q[j];
        }
        cur /= m - q[Back(0)];
        res.push(cur);
        ans += cur;
    }
    out.print_line(ans / m);
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
