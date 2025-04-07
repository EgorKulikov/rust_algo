//{"name":"Counting Problem","group":"CodeChef - START180A","url":"https://www.codechef.com/START180A/problems/CNTR","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2\n-1 -1\n2 2\n-1 1\n3 3\n1 2 3\n","output":"9\n4\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_utils::powers;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    type Mod = ModIntF;
    let num_tail = Vec::with_gen_back(n + 1, |i, num_tail| {
        if i == n {
            0
        } else {
            num_tail[i + 1] + if a[i] == -1 { 1 } else { 0 }
        }
    });
    let pw = powers(Mod::from_index(k), n);
    let mut mem = Memoization3d::new(n + 1, k + 1, 2, |mem, pos, last, is_first| -> Mod {
        if pos == n {
            Mod::zero()
        } else if a[pos] == -1 {
            let mut res = if last != k {
                pw[num_tail[pos + 1]] + mem.call(pos + 1, last + 1, 1) + mem.call(pos, last + 1, 0)
            } else {
                Mod::zero()
            };
            if is_first == 1 {
                res += Mod::from_index(k) * mem.call(pos + 1, last, 1);
            }
            res
        } else if a[pos] as usize > last {
            pw[num_tail[pos + 1]]
                + mem.call(pos + 1, a[pos] as usize, 1)
                + mem.call(pos + 1, last, 1)
        } else {
            mem.call(pos + 1, last, 1)
        }
    });
    out.print_line(mem.call(0, 0, 1));
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
