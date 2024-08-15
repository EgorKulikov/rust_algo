//{"name":"Counting is Fun 101","group":"CodeChef - START147A","url":"https://www.codechef.com/START147A/problems/CNTSTILLFUN","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2\n1 2\n2\n2 2\n4\n1 2 3 4\n4\n1 1 4 4\n4\n3 4 3 4\n3\n3 3 3\n","output":"1\n2\n1\n0\n16\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CountingIsFun101"}}}

use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let q = a.qty_bound(n);
    type Mod = ModInt7;
    let mut ans = Mod::one();
    let mut free = 0;
    for i in (0..n).rev() {
        if q[i] == 0 {
            if free == 0 {
                ans = Mod::zero();
                break;
            }
            ans *= Mod::new(free);
            ans *= Mod::new(free);
            free -= 1;
        } else if q[i] == 1 {
            ans *= Mod::new(2 * free + 1);
        } else if q[i] == 2 {
            ans *= Mod::new(2);
            free += 1;
        } else {
            ans = Mod::zero();
            break;
        }
    }
    out.print_line(ans);
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
