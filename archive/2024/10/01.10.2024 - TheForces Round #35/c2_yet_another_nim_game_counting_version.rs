//{"name":"C2. Yet Another Nim Game (Counting version)","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/C2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n4\n7\n","output":"1\n12\n1440\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2YetAnotherNimGameCountingVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModInt7;
    let c = Combinations::<Mod>::new(300_001);

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        if n % 2 == 0 {
            out.print_line(c.fact(n / 2) * c.fact(n / 2) * c.c(n / 2 + 1, 1));
        } else {
            out.print_line(c.fact(n / 2) * c.fact(n / 2 + 1) * c.comb_with_rep(n / 2 + 1, 2));
        }
    }
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
