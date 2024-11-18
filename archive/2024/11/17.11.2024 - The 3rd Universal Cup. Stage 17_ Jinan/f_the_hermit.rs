//{"name":"F. The Hermit","group":"Universal Cup - The 3rd Universal Cup. Stage 17: Jinan","url":"https://contest.ucup.ac/contest/1843/problem/9553","interactive":false,"timeLimit":1000,"tests":[{"input":"4 3\n","output":"7\n"},{"input":"11 4\n","output":"1187\n"},{"input":"100000 99999\n","output":"17356471\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTheHermit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(m + 1);
    let mut mem = Memoization2::new(|mem, n: usize, len: usize| -> (Mod, Mod) {
        if n <= len {
            (Mod::zero(), Mod::zero())
        } else if len == 0 {
            (Mod::zero(), Mod::one())
        } else {
            let mut qty = c.c(n - 1, len);
            let mut sum = Mod::zero();
            for i in 2..=n {
                let (call_sum, call_qty) = mem.call(n / i, len - 1);
                qty -= call_qty;
                sum += call_sum;
            }
            sum += qty * Mod::from_index(len);
            (sum, c.c(n - 1, len))
        }
    });
    out.print_line(mem.call(m, n).0 + mem.call(m, n - 1).0);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
