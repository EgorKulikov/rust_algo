//{"name":"C. Kevin and Puzzle","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n0 1 2\n5\n0 0 0 0 0\n5\n0 0 1 1 2\n5\n0 1 2 3 4\n5\n0 0 1 1 1\n5\n5 1 5 2 5\n1\n0\n4\n2 3 1 1\n","output":"1\n2\n3\n0\n4\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut num_liars_last_liar = 0;
    let mut num_liars_last_honest = 0;
    let mut last_liar = Mod::zero();
    let mut last_honest = Mod::one();
    const INF: usize = usize::MAX / 2;
    for i in 0..n {
        let mut next_honest = Mod::zero();
        let mut next_num_liars_last_honest = INF;
        if a[i] == num_liars_last_liar {
            next_honest += last_liar;
            next_num_liars_last_honest = num_liars_last_liar;
        }
        if a[i] == num_liars_last_honest {
            next_honest += last_honest;
            if next_num_liars_last_honest != INF {
                assert_eq!(next_num_liars_last_honest, num_liars_last_honest);
            } else {
                next_num_liars_last_honest = num_liars_last_honest;
            }
        }
        last_liar = last_honest;
        num_liars_last_liar = num_liars_last_honest + 1;
        num_liars_last_honest = next_num_liars_last_honest;
        last_honest = next_honest;
    }
    out.print_line(last_liar + last_honest);
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
