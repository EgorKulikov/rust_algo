//{"name":"C. Bitwise Slides","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 7 9\n4\n179 1 1 179\n5\n1 2 3 3 2\n12\n8 2 5 3 9 1 8 12 9 9 9 4\n1\n1000000000\n","output":"3\n9\n39\n123\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::ops::Add;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut xor = 0;
    type Mod = ModInt7;
    let mut ans = DefaultHashMap::new(Mod::zero());
    ans[0] = Mod::one();
    for i in a {
        if ans[xor] != Mod::zero() {
            ans[xor] *= Mod::new(3);
        }
        if ans[xor ^ i] != Mod::zero() {
            let add = Mod::new(2) * ans[xor ^ i];
            ans[xor] += add;
        }
        xor ^= i;
    }
    out.print_line(ans.into_values().reduce(Mod::add));
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

//START MAIN
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
//END MAIN
