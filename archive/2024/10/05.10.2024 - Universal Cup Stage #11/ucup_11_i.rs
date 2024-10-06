//{"name":"ucup_11_i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_11_i"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    let sum = a.iter().sum::<usize>();
    if sum % 2 == 1 {
        out.print_line(0);
        return;
    }
    let c = Combinations::<Mod>::new(n + 1);
    let mut mem = Memoization3d::new(n + 1, n + 1, sum + 1, |mem, step, q, s| {
        if step == n {
            if s == sum / 2 {
                c.fact(q) * c.fact(n - q)
            } else {
                Mod::zero()
            }
        } else {
            mem.call(step + 1, q, s) + mem.call(step + 1, q + 1, s + a[step])
        }
    });
    out.print_line(mem.call(0, 0, 0));
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
