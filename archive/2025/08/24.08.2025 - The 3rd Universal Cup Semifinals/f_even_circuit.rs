//{"name":"F. Even Circuit","group":"Universal Cup - The 3rd Universal Cup Semifinals","url":"https://contest.ucup.ac/contest/2506/problem/14019","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 1\n","output":"Yes\n2\n"},{"input":"5\n7 4 3 1 2\n","output":"Yes\n4\n"},{"input":"6\n40 63 64 9 6 1\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::fwht::FWHT;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut b = vec![Mod::zero(); 1 << 22];
    for x in a.copy_iter() {
        b[x] += Mod::one();
    }
    b.fwht(false);
    for x in b.iter_mut() {
        *x = *x * *x;
    }
    let mut c = vec![Mod::one(); 1 << 22];
    let mut comb = Combinations::<Mod>::new(n);
    let mut mem = Memoization2d::new(100, 100, |mem, taken, remaining| {
        if remaining == 0 {
            Mod::one()
        } else {
            let mut res = Mod::zero();
            for i in (1..remaining).step_by(2) {
                res +=
                    comb.c(remaining - 1, i) * (n - taken) * mem.call(taken + 1, remaining - i - 1);
            }
            res
        }
    });
    out.set_bool_output(BoolOutput::YesNo);
    for i in 1..=n / 2 {
        for i in b.indices() {
            c[i] *= b[i];
        }
        let mut cc = c.clone();
        cc.fwht(true);
        let expected = mem.call(0, 2 * i);
        if cc[0] != expected {
            out.print_line(true);
            out.print_line(2 * i);
            return;
        }
    }
    out.print_line(false);
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
