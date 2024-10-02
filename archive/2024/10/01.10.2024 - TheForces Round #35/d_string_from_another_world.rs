//{"name":"D. String From Another World","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"5\n3 4\nabc\n1 3\nd\n5 3\nasdfg\n7 10\nwuhudsm\n9 11\nyugandhar\n","output":"1\n53\n0\n235\n261\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DStringFromAnotherWorld"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    input.read_str();

    if n > m {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let mut cur = vec![Mod::zero(); m + 1];
    cur[0] = Mod::one();
    let mut next = vec![Mod::zero(); m + 1];
    for i in 0..m {
        next.fill(Mod::zero());
        for j in 0..=i {
            next[j + 1] += cur[j];
            if j > 0 {
                next[j - 1] += cur[j] * Mod::new(26);
            } else {
                next[0] += cur[0];
            }
        }
        swap(&mut cur, &mut next);
    }
    out.print_line(cur[n]);
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
