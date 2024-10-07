//{"name":"Link-Cut Dumplings","group":"Eolymp - Basecamp - Eolymp Cup #2","url":"https://basecamp.eolymp.com/en/compete/ptmnufrm6p6nl7gods1loo65go/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n","output":"6 9\n"},{"input":"5\n","output":"14 6\n"},{"input":"6\n","output":"14 12\n"},{"input":"497478941\n","output":"1073741822 239149123\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LinkCutDumplings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_u64();

    type Mod = ModInt7;
    let mut res_equal = Mod::one();
    let mut res_one_less = Mod::zero();
    let mut res_two_less = Mod::zero();
    let mut best = 0;
    for i in (0..60).rev() {
        if n.is_set(i) && best == 0 {
            best = 2i64.power(i + 2) - 2;
        }
        if best != 0 {
            if !n.is_set(i) {
                res_two_less *= Mod::new(3);
                res_equal = Mod::zero();
            } else {
                res_two_less = res_two_less * Mod::new(3) + res_one_less;
                res_one_less = res_one_less * Mod::new(2) + res_equal * Mod::new(2);
            }
        }
    }
    out.print_line((best, res_equal + res_one_less + res_two_less));
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
